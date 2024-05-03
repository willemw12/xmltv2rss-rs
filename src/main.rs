//! Generate an RSS feed from an XMLTV TV listing. Print the result to standard output.
//!
//! [...more][`Args`]

use std::io;

use clap::{crate_version, Parser};

mod export;
mod xmltv;

use export::rss::{export, OptionsBuilder};
use export::rss::{
    DEFAULT_RSS_CHANNEL_DESCRIPTION, DEFAULT_RSS_CHANNEL_TITLE, DEFAULT_RSS_DATE_FORMAT,
    DEFAULT_RSS_TIME_FORMAT,
};
use xmltv::Error;
use xmltv::{DEFAULT_XMLTV_DATETIME_FORMAT, DEFAULT_XMLTV_DATETIME_FORMAT_UTC};

pub const DEFAULT_XML_INDENT: u8 = 2;

/// Generate an RSS feed from an XMLTV TV listing. Print the result to standard output.
///
/// For information about date and time format strings ("%Y", "%H", etc.),
/// search for "strftime" on <https://docs.rs/chrono/latest/chrono/index.html>.
#[derive(Parser)]
#[command(version = crate_version!())]
struct Args {
    /// RSS feed date format. Examples: "%%Y-%%m-%%d", "%%a %%d %%B, %%Y", "%%x".
    #[arg(long, short = 'd', default_value = DEFAULT_RSS_DATE_FORMAT)]
    feed_date_format: String,

    /// RSS feed description.
    #[arg(long, default_value = DEFAULT_RSS_CHANNEL_DESCRIPTION)]
    feed_description: String,

    /// RSS feed indentation.
    #[arg(long, default_value_t = DEFAULT_XML_INDENT)]
    feed_indent: u8,

    /// RSS feed language.
    #[arg(long)]
    feed_language: Option<String>,

    /// RSS feed URL.
    #[arg(long, default_value = "")]
    feed_link: String,

    /// RSS feed time format. Examples: "%%H:%%M", "%%I:%%M %%p", "%%X".
    #[arg(long, short = 't', default_value = DEFAULT_RSS_TIME_FORMAT)]
    feed_time_format: String,

    /// RSS feed title.
    #[arg(long, default_value = DEFAULT_RSS_CHANNEL_TITLE)]
    feed_title: String,

    // #[arg(long, default_value = DEFAULT_XMLTV_DATETIME_FORMAT,
    //       help = concatcp!("XMLTV date and time format\n[default fallback: \"", DEFAULT_XMLTV_DATETIME_FORMAT_UTC, "\"]"))]
    #[arg(long, default_value = DEFAULT_XMLTV_DATETIME_FORMAT,
          help = format!("XMLTV date and time format\n[default fallback: {DEFAULT_XMLTV_DATETIME_FORMAT_UTC:?}]"))]
    xmltv_datetime_format: String,

    /// Read XMLTV file or from standard input if '-'.
    file: Option<String>,
}

fn main() -> Result<(), Error> {
    let args = Args::parse();

    run(args)
}

fn run(args: Args) -> Result<(), Error> {
    let feed_language = args.feed_language.unwrap_or_default();
    let options = OptionsBuilder::default()
        // .language(feed_language.as_str())
        .language(&*feed_language)
        .build()?;

    let channel = export(
        &args.feed_title,
        &args.feed_link,
        Some(&args.feed_description),
        &options,
        args.file.as_deref(),
    )?;

    let feed_indent = args.feed_indent;
    if feed_indent > 0 {
        channel.pretty_write_to(io::stdout(), b' ', args.feed_indent.into())?;
    } else {
        channel.write_to(io::stdout())?;
    }

    Ok(())
}
