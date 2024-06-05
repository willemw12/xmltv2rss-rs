xmltv2rss
=========

Generate an RSS or Atom feed from an XMLTV TV listing.


Installation
------------

The following requires [Rust](https://www.rust-lang.org/).

To install xmltv2rss, for example, in folder ~/.local/bin, run:

    $ cargo install --git=https://github.com/willemw12/xmltv2rss-rs.git --no-track --root=$HOME/.local

Or the same, but download separately:

    $ git clone https://github.com/willemw12/xmltv2rss-rs.git
    $ cargo install --no-track --path=./xmltv2rss-rs --root=$HOME/.local


Usage
-----

    $ xmltv2rss --help
    Generate an RSS or Atom feed from an XMLTV TV listing. Print the result to standard output.
    
    For information about date and time format strings ("%Y", "%H", etc.), search for "strftime" on <https://docs.rs/chrono/latest/chrono/index.html>.
    
    Usage: xmltv2rss [OPTIONS] [FILE]
    
    Arguments:
      [FILE]
              Read XMLTV file or from standard input if '-'
    
    Options:
      -d, --feed-date-format <FEED_DATE_FORMAT>
              Output feed date format. Examples: "%%Y-%%m-%%d", "%%a %%d %%B, %%Y", "%%x"
    
              [default: "%a %d %B, %Y"]
    
          --feed-description <FEED_DESCRIPTION>
              Output feed description
    
          --feed-indent <FEED_INDENT>
              Output feed indentation
    
              [default: 2]
    
          --feed-language <FEED_LANGUAGE>
              Output feed language
    
          --feed-link <FEED_LINK>
              Output feed URL
    
              [default: ]
    
      -t, --feed-time-format <FEED_TIME_FORMAT>
              Output feed time format. Examples: "%%H:%%M", "%%I:%%M %%p", "%%X"
    
              [default: %H:%M]
    
          --feed-title <FEED_TITLE>
              Output feed title
    
              [default: "XMLTV feed"]
    
          --feed-type <FEED_TYPE>
              Output feed type
    
              [default: rss]
    
              Possible values:
              - atom
              - rss:  Rss 2.0
    
          --xmltv-datetime-format <XMLTV_DATETIME_FORMAT>
              XMLTV date and time format
              [default fallback: "%Y%m%d%H%M%S"]
    
              [default: "%Y%m%d%H%M%S %z"]
    
      -h, --help
              Print help (see a summary with '-h')
    
      -V, --version
              Print version


Library usage
-------------

```rust
use std::io;
use xmltv2rss::error::Result;
use xmltv2rss::export::{rss, OptionsBuilder};

fn print() -> Result<()> {
    // let options = rss::Options::default();
    let options = OptionsBuilder::default()
        // .language(&*string)
        // .language(string.as_str())
        .language("en")
        .build()?;

    let channel = rss::export("Title", "https://example.com/", Some("Description"),
                              &options, Some("./tests/input/simple.xml"))?;

    channel.pretty_write_to(io::stdout(), b' ', 2)?;

    Ok(())
}
```


License
-------

GPL-3.0 or later


Link
----

[GitHub](https://github.com/willemw12/xmltv2rss-rs)

