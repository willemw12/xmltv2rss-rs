use atom_syndication::{
    Entry, EntryBuilder, Feed, FeedBuilder, GeneratorBuilder, LinkBuilder, Text,
};
use chrono::{DateTime, FixedOffset, Local};
use quick_xml::de::from_str;
use std::fs;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::io;
use uuid::Uuid;
use xmltv::{Channel, Programme, Tv};

use crate::error::Error;
use crate::export::{Options, Visitor, GUID_DATETIME_FORMAT};
use crate::export::{DEFAULT_FEED_CHANNEL_DESCRIPTION, DEFAULT_FEED_CHANNEL_TITLE};
use crate::xmltv::DEFAULT_XMLTV_DATETIME_FORMAT_UTC;
use crate::xmltv::{find_name, find_value, first_url, parse_from_str};

/// Exports an XMLTV TV listing to an Atom feed.
pub fn export(
    title: &str,
    link: &str,
    subtitle: Option<&str>,
    options: &Options,
    // reader: &mut impl Read,
    file: Option<&str>,
) -> Result<Feed, Error> {
    let last_build_date = Local::now();
    let (xmltv_listing, updated) = match &file {
        Some(file) if *file != "-" => (
            fs::read_to_string(file)?,
            DateTime::<Local>::from(fs::metadata(file)?.modified()?),
        ),
        _ => (io::read_to_string(io::stdin())?, last_build_date),
    };
    let xmltv_listing: Tv = from_str(&xmltv_listing)?;

    let mut visitor = Atom::new(
        title,
        link,
        subtitle,
        Some(updated),
        options,
        &xmltv_listing.channels,
    );

    super::export::<Feed>(&mut visitor, &xmltv_listing)
}

//

/// Atom feed export struct.
pub(crate) struct Atom<'a> {
    title: &'a str,
    link: &'a str,
    subtitle: Option<&'a str>,
    updated: Option<DateTime<Local>>,
    options: &'a Options<'a>,

    // Visitor state
    xmltv_channels: &'a Vec<Channel>,
    feed: FeedBuilder,
    entries: Vec<Entry>,
}

impl<'a> Atom<'a> {
    pub fn new(
        title: &'a str,
        link: &'a str,
        subtitle: Option<&'a str>,
        updated: Option<DateTime<Local>>,
        options: &'a Options,

        // Input data
        xmltv_channels: &'a Vec<Channel>,
    ) -> Self {
        let title = if !title.is_empty() {
            title
        } else {
            DEFAULT_FEED_CHANNEL_TITLE
        };

        Self {
            title,
            link,
            subtitle,
            updated,
            options,

            // Visitor state
            xmltv_channels,
            feed: FeedBuilder::default(),
            entries: vec![],
        }
    }

    fn entry_summary(
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

        let summary = format!("\
<table>\
<tr><td align=\"right\" valign=\"top\">Title:</td><td>{title}</td></tr>\
<tr><td align=\"right\" valign=\"top\">Channel:</td><td>{channel}</td></tr>\
<tr><td align=\"right\" valign=\"top\">Airdate:</td><td>{airdate}</td></tr>\
<tr><td align=\"right\" valign=\"top\">Airtime:</td><td>{airtime}</td></tr>\
<tr><td align=\"right\" valign=\"top\" style=\"white-space: nowrap\">Length:</td><td>{airtime_length}</td></tr>\
<tr><td align=\"right\" valign=\"top\">Category:</td><td>{category}</td></tr>\
<tr><td align=\"right\" valign=\"top\">Description:</td><td>{desc}</td></tr>\
</table>");

        Ok(summary)
    }
}

impl Visitor for Atom<'_> {
    type Output = Feed;

    /// Exports from XMLTV TV listing to Atom feed.
    fn visit_tv(&mut self, _xmltv_listing: &Tv) -> Result<(), Error> {
        self.feed
            .title(self.title)
            .link(LinkBuilder::default().href(self.link).build())
            .generator(
                GeneratorBuilder::default()
                    .value(DEFAULT_FEED_CHANNEL_DESCRIPTION)
                    .build(),
            );

        if let Some(subtitle) = self.subtitle {
            self.feed.subtitle(Text::plain(subtitle));
        }
        if let Some(language) = self.options.language {
            self.feed.lang(language.to_string());
        }
        if let Some(updated) = self.updated {
            self.feed.updated(updated);
        }

        Ok(())
    }

    fn visit_programmes_start(&mut self) -> Result<(), Error> {
        self.entries.clear();

        Ok(())
    }

    /// Exports from XMLTV programme to Atom entry.
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

        let link = first_url(&xmltv_programme.urls).unwrap_or_default();

        let summary = self.entry_summary(
            language,
            title,
            channel_id,
            starttime_dt,
            stoptime_dt,
            xmltv_programme,
        )?;

        let hash_data = format!("{channel_id}-{}", starttime_dt.format(GUID_DATETIME_FORMAT));
        let uuid = uuid(hash_data.as_bytes());

        let published = starttime_dt;

        let entry = EntryBuilder::default()
            .title(title.to_string())
            .link(LinkBuilder::default().href(link).build())
            .summary(Text::plain(summary))
            .id(format!("urn:uuid:{uuid}"))
            .published(published)
            .updated(published)
            .build();

        self.entries.push(entry);

        Ok(())
    }

    fn visit_programmes_end(&mut self) -> Result<(), Error> {
        self.feed.entries(&*self.entries);
        self.entries.clear();

        Ok(())
    }

    /// Returns the exported Atom feed.
    fn result(&self) -> Result<Self::Output, Error> {
        Ok(self.feed.build())
    }
}

fn uuid(hash_data: &[u8]) -> Uuid {
    let mut hasher = DefaultHasher::new();
    Hash::hash_slice(hash_data, &mut hasher);
    let hash = hasher.finish();

    // Uuid::from_u128(hash as u128)
    Uuid::from_u128(hash as u128 * u64::MAX as u128)
}

//

#[cfg(test)]
mod tests {
    use atom_syndication::WriteConfig;
    use pretty_assertions::assert_eq;
    use quick_xml::de::from_str;
    use std::fs;

    use super::*;
    use crate::export;

    const DEFAULT_XML_INDENT: usize = 2;
    const UPDATED: &str = "Tue, 30 Apr 2024 12:00:00 +0000";

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
                expected_file: "tests/output/atom/simple.xml",
                language: None,
            },
            Test {
                input_file: "tests/input/simple.xml",
                expected_file: "tests/output/atom/simple-language.xml",
                language: Some("fr-FR"),
            },
            Test {
                input_file: "tests/input/timezones.xml",
                expected_file: "tests/output/atom/timezones.xml",
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
            let subtitle = None;
            let updated = DateTime::parse_from_rfc2822(UPDATED).unwrap();
            // let options = Options::default();
            let options = Options {
                language: test.language,
                ..Default::default()
            };
            let xmltv_listing: Tv = from_str(&input).unwrap();

            // Run test
            let mut visitor = Atom::new(
                DEFAULT_FEED_CHANNEL_TITLE,
                link,
                subtitle,
                Some(updated.into()),
                &options,
                &xmltv_listing.channels,
            );
            let feed = export::export::<Feed>(&mut visitor, &xmltv_listing).unwrap();

            let config = WriteConfig {
                indent_size: DEFAULT_XML_INDENT.into(),
                ..Default::default()
            };
            let output =
                String::from_utf8(feed.write_with_config(Vec::new(), config).unwrap()).unwrap();

            // Check result
            // assert_eq!(output, expected, "for output file {expected_file} and failed formatted content:\n{output}\n");
            assert_eq!(output, expected, "for output file {expected_file}");
        }
    }
}
