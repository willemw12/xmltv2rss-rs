//! Main module.

use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use thiserror::Error;
use xmltv::{NameAndLang, Url, ValueAndLang};

use crate::export::rss::OptionsBuilderError;

/// Datetime with timezone
pub const DEFAULT_XMLTV_DATETIME_FORMAT: &str = "%Y%m%d%H%M%S %z";
/// Datetime without timezone
pub const DEFAULT_XMLTV_DATETIME_FORMAT_UTC: &str = "%Y%m%d%H%M%S";

/// Common error type.
#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    De(#[from] quick_xml::DeError),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    OptionsBuilder(#[from] OptionsBuilderError),

    #[error(transparent)]
    Parse(#[from] chrono::ParseError),

    #[error(transparent)]
    Rss(#[from] rss::Error),
}

//

// XMLTV access functions

/// Returns the name in the specified language or the first value or an empty string.
pub(crate) fn find_name<'a>(elements: &'a [NameAndLang], language: Option<&'a str>) -> &'a str {
    elements
        .iter()
        .find(|e| language.is_none() || e.lang.as_deref() == language)
        .or_else(|| elements.first())
        .map_or("", |e| &e.name)
}

/// Returns the value in the specified language or the first value or an empty string.
pub(crate) fn find_value<'a>(elements: &'a [ValueAndLang], language: Option<&'a str>) -> &'a str {
    elements
        .iter()
        .find(|e| language.is_none() || e.lang.as_deref() == language)
        .or_else(|| elements.first())
        .map_or("", |e| &e.value)
}

// /// Returns the first URL string or an empty string.
// pub(crate) fn first_url(urls: &[Url]) -> &str {
//     urls.first().map_or("", |url| &url.value)
// }

// /// Tries to return the first URL string.
// pub(crate) fn first_url(urls: &[Url]) -> Option<&str> {
//     urls.first().map(|url| &url.value).map(|url| url.as_str())
// }

/// Tries to return the first URL string.
pub(crate) fn first_url(urls: &[Url]) -> Option<String> {
    urls.first().map(|url| &url.value).cloned()
}

/// Tries parsing with a datetime format string, which is timezone-aware.
/// Or else tries parsing with a naive datetime format string, which has no timezone.
pub(crate) fn parse_from_str(
    datetime: &str,
    datetime_format: &str,
    naive_datetime_format: &str,
) -> Result<DateTime<Utc>, Error> {
    let datetime = DateTime::parse_from_str(datetime, datetime_format)
        .or_else(|_| {
            NaiveDateTime::parse_from_str(datetime, naive_datetime_format)
                .map(|datetime| Utc.from_utc_datetime(&datetime).fixed_offset())
        })?
        .into();

    Ok(datetime)
}
