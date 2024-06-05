//! Generate an RSS or Atom feed from an XMLTV TV listing. Print the result to standard output.
//!
//! [...more][`Args`]

use atom_syndication::WriteConfig;
use clap::{crate_version, Parser, ValueEnum};
use std::io::{self, Write};

mod error;
mod export;
mod xmltv;

use error::Result;
use export::{atom, rss, Options, OptionsBuilder};
use export::{DEFAULT_FEED_CHANNEL_TITLE, DEFAULT_FEED_DATE_FORMAT, DEFAULT_FEED_TIME_FORMAT};
use xmltv::{DEFAULT_XMLTV_DATETIME_FORMAT, DEFAULT_XMLTV_DATETIME_FORMAT_UTC};

pub const DEFAULT_XML_INDENT: u8 = 2;

#[derive(Clone, Default, Debug, ValueEnum)]
enum FeedType {
    Atom,

    /// Rss 2.0
    #[default]
    Rss,
}

/// Generate an RSS or Atom feed from an XMLTV TV listing. Print the result to standard output.
///
/// For information about date and time format strings ("%Y", "%H", etc.),
/// search for "strftime" on <https://docs.rs/chrono/latest/chrono/index.html>.
#[derive(Parser)]
#[command(version = crate_version!())]
struct Args {
    /// Output feed date format. Examples: "%%Y-%%m-%%d", "%%a %%d %%B, %%Y", "%%x".
    #[arg(long, short = 'd', default_value = DEFAULT_FEED_DATE_FORMAT)]
    feed_date_format: String,

    /// Output feed description.
    #[arg(long)]
    feed_description: Option<String>,

    /// Output feed indentation.
    #[arg(long, default_value_t = DEFAULT_XML_INDENT)]
    feed_indent: u8,

    /// Output feed language.
    #[arg(long)]
    feed_language: Option<String>,

    /// Output feed URL.
    #[arg(long, default_value = "")]
    feed_link: String,

    /// Output feed time format. Examples: "%%H:%%M", "%%I:%%M %%p", "%%X".
    #[arg(long, short = 't', default_value = DEFAULT_FEED_TIME_FORMAT)]
    feed_time_format: String,

    /// Output feed title.
    #[arg(long, default_value = DEFAULT_FEED_CHANNEL_TITLE)]
    feed_title: String,

    /// Output feed type.
    #[arg(long, default_value_t, value_enum)]
    feed_type: FeedType,

    // #[arg(long, default_value = DEFAULT_XMLTV_DATETIME_FORMAT,
    //       help = concatcp!("XMLTV date and time format\n[default fallback: \"", DEFAULT_XMLTV_DATETIME_FORMAT_UTC, "\"]"))]
    #[arg(long, default_value = DEFAULT_XMLTV_DATETIME_FORMAT,
          help = format!("XMLTV date and time format\n[default fallback: {DEFAULT_XMLTV_DATETIME_FORMAT_UTC:?}]"))]
    xmltv_datetime_format: String,

    /// Read XMLTV file or from standard input if '-'.
    file: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    export(&args)
}

//

fn export(args: &Args) -> Result<()> {
    let mut options = OptionsBuilder::default();
    // if let Some(language) = &args.feed_language && !language.is_empty()
    if let Some(language) = &args.feed_language {
        if !language.is_empty() {
            options.language(language.as_str());
        }
    }
    let options = options.build()?;

    let mut writer = io::stdout();

    match args.feed_type {
        FeedType::Atom => export_to_atom(args, &options, &mut writer),
        FeedType::Rss => export_to_rss(args, &options, &mut writer),
    }
}

fn export_to_atom(args: &Args, options: &Options, writer: &mut impl Write) -> Result<()> {
    let feed = atom::export(
        &args.feed_title,
        &args.feed_link,
        args.feed_description.as_deref(),
        options,
        args.file.as_deref(),
    )?;

    let feed_indent = args.feed_indent;
    if feed_indent > 0 {
        let config = WriteConfig {
            indent_size: Some(feed_indent.into()),
            ..Default::default()
        };
        feed.write_with_config(writer, config)?;
    } else {
        feed.write_to(writer)?;
    }

    Ok(())
}

fn export_to_rss(args: &Args, options: &Options, writer: &mut impl Write) -> Result<()> {
    let channel = rss::export(
        &args.feed_title,
        &args.feed_link,
        args.feed_description.as_deref(),
        options,
        args.file.as_deref(),
    )?;

    let feed_indent = args.feed_indent;
    if feed_indent > 0 {
        channel.pretty_write_to(writer, b' ', feed_indent.into())?;
    } else {
        channel.write_to(writer)?;
    }

    Ok(())
}
