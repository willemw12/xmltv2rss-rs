//! Generate an RSS feed from an XMLTV TV listing.
//!
//! # Example
//!
//! ```
//! use std::io;
//! use xmltv2rss::error::Result;
//! use xmltv2rss::export::{rss, OptionsBuilder};
//!
//! fn print() -> Result<()> {
//!     // let options = rss::Options::default();
//!     let options = OptionsBuilder::default()
//!         // .language(&*string)
//!         // .language(string.as_str())
//!         .language("en")
//!         .build()?;
//!
//!     let channel = rss::export("Title", "https://example.com/", Some("Description"),
//!                               &options, Some("./tests/input/simple.xml"))?;
//!
//!     channel.pretty_write_to(io::stdout(), b' ', 2)?;
//!
//!     Ok(())
//! }
//! ```

pub mod error;
pub mod export;
pub mod xmltv;
