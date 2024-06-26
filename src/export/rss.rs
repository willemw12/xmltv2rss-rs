use chrono::FixedOffset;
use chrono::{DateTime, Local};
use quick_xml::de::from_str;
use rss::{Channel, ChannelBuilder, Guid, Item, ItemBuilder};
use std::fs;
use std::io;
use xmltv::{Programme, Tv};

use crate::error::Error;
use crate::export::{Options, Visitor, GUID_DATETIME_FORMAT};
use crate::export::{DEFAULT_FEED_CHANNEL_DESCRIPTION, DEFAULT_FEED_CHANNEL_TITLE};
use crate::xmltv::DEFAULT_XMLTV_DATETIME_FORMAT_UTC;
use crate::xmltv::{find_name, find_value, first_url, parse_from_str};

/// Exports an XMLTV TV listing to an RSS channel/feed.
pub fn export(
    title: &str,
    link: &str,
    description: Option<&str>,
    options: &Options,
    // reader: &mut impl Read,
    file: Option<&str>,
) -> Result<Channel, Error> {
    let last_build_date = Local::now();
    let (xmltv_listing, pub_date) = match &file {
        Some(file) if *file != "-" => (
            fs::read_to_string(file)?,
            DateTime::<Local>::from(fs::metadata(file)?.modified()?),
        ),
        _ => (io::read_to_string(io::stdin())?, last_build_date),
    };
    let xmltv_listing: Tv = from_str(&xmltv_listing)?;

    let mut visitor = Rss::new(
        title,
        link,
        description,
        Some(pub_date),
        Some(last_build_date),
        options,
        &xmltv_listing.channels,
    );

    super::export::<Channel>(&mut visitor, &xmltv_listing)
}

//

/// RSS channel/feed export struct.
pub(crate) struct Rss<'a> {
    title: &'a str,
    link: &'a str,
    description: Option<&'a str>,
    pub_date: Option<DateTime<Local>>,
    last_build_date: Option<DateTime<Local>>,
    options: &'a Options<'a>,

    // Visitor state
    xmltv_channels: &'a Vec<xmltv::Channel>,
    channel: ChannelBuilder,
    items: Vec<Item>,
}

impl<'a> Rss<'a> {
    pub fn new(
        title: &'a str,
        link: &'a str,
        description: Option<&'a str>,
        pub_date: Option<DateTime<Local>>,
        last_build_date: Option<DateTime<Local>>,
        options: &'a Options,

        // Input data
        xmltv_channels: &'a Vec<xmltv::Channel>,
    ) -> Self {
        let title = if !title.is_empty() {
            title
        } else {
            DEFAULT_FEED_CHANNEL_TITLE
        };

        Self {
            title,
            link,
            description,
            pub_date,
            last_build_date,
            options,

            // Visitor state
            xmltv_channels,
            channel: ChannelBuilder::default(),
            items: vec![],
        }
    }

    fn item_description(
        &mut self,
        language: Option<&str>,
        title: &str,
        channel_id: &str,
        starttime_dt: DateTime<FixedOffset>,
        stoptime_dt: DateTime<FixedOffset>,
        xmltv_programme: &Programme,
    ) -> Result<String, Error> {
        let channel = if let Some(channel_callsign) = self
            .xmltv_channels
            .iter()
            .find(|channel| channel.id == *channel_id)
        {
            let display_name = find_name(&channel_callsign.display_names, language);
            format!("{channel_id}-{display_name}")
        } else {
            channel_id.to_string()
        };

        let airdate = format!("{}", starttime_dt.format(self.options.date_format));
        let airtime = format!(
            "{} - {}",
            starttime_dt.format(self.options.time_format),
            stoptime_dt.format(self.options.time_format)
        );

        let airtime_length_td = stoptime_dt - starttime_dt;
        let airtime_length_mins = airtime_length_td.num_seconds() / 60;
        let airtime_length = format!(
            "{:02}:{:02}:00",
            airtime_length_mins / 60,
            airtime_length_mins % 60
        );

        let category = find_name(&xmltv_programme.categories, language);

        let desc = find_value(&xmltv_programme.descriptions, language);
        let desc = desc
            .trim()
            .lines()
            .map(|line| line.trim())
            .collect::<Vec<_>>()
            .join("<br/>");

        let description = format!("\
<table>\
<tr><td align=\"right\" valign=\"top\">Title:</td><td>{title}</td></tr>\
<tr><td align=\"right\" valign=\"top\">Channel:</td><td>{channel}</td></tr>\
<tr><td align=\"right\" valign=\"top\">Airdate:</td><td>{airdate}</td></tr>\
<tr><td align=\"right\" valign=\"top\">Airtime:</td><td>{airtime}</td></tr>\
<tr><td align=\"right\" valign=\"top\" style=\"white-space: nowrap\">Length:</td><td>{airtime_length}</td></tr>\
<tr><td align=\"right\" valign=\"top\">Category:</td><td>{category}</td></tr>\
<tr><td align=\"right\" valign=\"top\">Description:</td><td>{desc}</td></tr>\
</table>");

        Ok(description)
    }
}

impl Visitor for Rss<'_> {
    type Output = Channel;

    /// Exports from XMLTV TV listing to RSS channel/feed.
    fn visit_tv(&mut self, _xmltv_listing: &Tv) -> Result<(), Error> {
        self.channel
            .title(self.title)
            .link(self.link)
            .description(self.description.unwrap_or(DEFAULT_FEED_CHANNEL_DESCRIPTION));

        if let Some(language) = self.options.language {
            self.channel.language(language.to_string());
        }
        if let Some(pub_date) = self.pub_date {
            self.channel.pub_date(pub_date.to_rfc2822());
        }
        if let Some(last_build_date) = self.last_build_date {
            self.channel.last_build_date(last_build_date.to_rfc2822());
        }

        Ok(())
    }

    fn visit_programmes_start(&mut self) -> Result<(), Error> {
        self.items.clear();

        Ok(())
    }

    /// Exports from XMLTV programme to RSS item.
    fn visit_programme(&mut self, xmltv_programme: &Programme) -> Result<(), Error> {
        // let language = self.options.language;
        let language = self.options.language.filter(|l| !l.is_empty());

        let channel_id = &xmltv_programme.channel;

        let starttime = &xmltv_programme.start;
        let stoptime = match &xmltv_programme.stop {
            Some(time) => time,
            None => starttime,
        };

        let starttime_dt = parse_from_str(
            starttime,
            self.options.xmltv_datetime_format,
            DEFAULT_XMLTV_DATETIME_FORMAT_UTC,
        )?;
        let stoptime_dt = parse_from_str(
            stoptime,
            self.options.xmltv_datetime_format,
            DEFAULT_XMLTV_DATETIME_FORMAT_UTC,
        )?;

        //

        let title = find_value(&xmltv_programme.titles, language);

        let link = first_url(&xmltv_programme.urls);

        let description = self.item_description(
            language,
            title,
            channel_id,
            starttime_dt,
            stoptime_dt,
            xmltv_programme,
        )?;

        let mut guid = Guid::default();
        guid.set_value(format!(
            "{channel_id}-{}",
            starttime_dt.format(GUID_DATETIME_FORMAT)
        ));
        let guid = guid;

        let pub_date = starttime_dt.to_rfc2822();

        let item = ItemBuilder::default()
            .title(title.to_string())
            .link(link)
            .description(description.to_string())
            .guid(Some(guid))
            .pub_date(pub_date)
            .build();

        self.items.push(item);

        Ok(())
    }

    fn visit_programmes_end(&mut self) -> Result<(), Error> {
        self.channel.items(&*self.items);
        self.items.clear();

        Ok(())
    }

    /// Returns the exported RSS channel/feed.
    fn result(&self) -> Result<Self::Output, Error> {
        Ok(self.channel.build())
    }
}

//

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use quick_xml::de::from_str;
    use rss::Channel;
    use std::fs;

    use super::*;
    use crate::export;

    const DEFAULT_XML_INDENT: usize = 2;
    const LAST_BUILD_DATE: &str = "Tue, 30 Apr 2024 12:00:00 +0000";
    const PUB_DATE: &str = "Mon, 29 Apr 2024 12:00:00 +0000";

    #[derive(Default)]
    struct Test<'a> {
        input_file: &'a str,
        expected_file: &'a str,
        language: Option<&'a str>,
    }

    #[test]
    fn test() {
        // const TESTS: [Test; _] = [Test {
        const TESTS: [Test; 3] = [
            Test {
                input_file: "tests/input/simple.xml",
                expected_file: "tests/output/rss/simple.xml",
                language: None,
            },
            Test {
                input_file: "tests/input/simple.xml",
                expected_file: "tests/output/rss/simple-language.xml",
                language: Some("fr-FR"),
            },
            Test {
                input_file: "tests/input/timezones.xml",
                expected_file: "tests/output/rss/timezones.xml",
                language: None,
            },
        ];

        for test in TESTS.iter() {
            // Get test values
            let expected_file = test.expected_file;
            let input_file = test.input_file;

            let expected = fs::read_to_string(expected_file).unwrap();
            // fs::read_to_string() adds a newline character at the end of the string
            let expected = expected.trim_end();

            // Run tests in the expected timezone
            // May not work on all platforms and set_var() will be defined as "unsafe" in a future Rust release
            std::env::set_var("TZ", "UTC");

            // Get input arguments
            let input = fs::read_to_string(input_file).unwrap();
            let link = "";
            let last_build_date = DateTime::parse_from_rfc2822(LAST_BUILD_DATE).unwrap();
            let pub_date = DateTime::parse_from_rfc2822(PUB_DATE).unwrap();
            // let options = Options::default();
            let options = Options {
                language: test.language,
                ..Default::default()
            };
            let xmltv_listing: Tv = from_str(&input).unwrap();

            // Run test
            let mut visitor = Rss::new(
                DEFAULT_FEED_CHANNEL_TITLE,
                link,
                None,
                Some(pub_date.into()),
                Some(last_build_date.into()),
                &options,
                &xmltv_listing.channels,
            );
            let channel = export::export::<Channel>(&mut visitor, &xmltv_listing).unwrap();

            let output = String::from_utf8(
                channel
                    .pretty_write_to(Vec::new(), b' ', DEFAULT_XML_INDENT)
                    .unwrap(),
            )
            .unwrap();

            // Check result
            // assert_eq!(output, expected, "for output file {expected_file} and failed formatted content:\n{output}\n");
            assert_eq!(output, expected, "for output file {expected_file}");
        }
    }
}
