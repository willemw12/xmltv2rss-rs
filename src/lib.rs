//! Generate an RSS feed from an XMLTV TV listing.
//!
//! # Example
//!
//! ```
//! use std::io;
//! use xmltv2rss::export::rss::{export, OptionsBuilder};
//!
//! // let options = Options::default();
//! let options = OptionsBuilder::default()
//!     // .language(string.as_str())
//!     .language("en")
//!     .build()?;
//!
//! let channel = export("Title", "https://example.com/", &options,
//!                      Some("./tests/input/simple.xml"))?;
//!
//! channel.pretty_write_to(io::stdout(), b' ', 2)?;
//! # Ok::<(), xmltv2rss::xmltv::Error>(())
//! ```

pub mod export;
pub mod xmltv;
