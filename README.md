xmltv2rss
=========

Generate an RSS feed from an XMLTV TV listing.


Installation
------------

The following requires [Rust](https://www.rust-lang.org/).

To install xmltv2rss, for example, in folder ~/.local/bin, run:

    $ git clone https://github.com/willemw12/xmltv2rss-rs.git
    $ cargo install --no-track --path=./xmltv2rss-rs --root=$HOME/.local


Usage
-----

    $ xmltv2rss --help
    Generate an RSS feed from an XMLTV TV listing. Print the result to standard output.
    
    For information about date and time format strings ("%Y", "%H", etc.), search for "strftime" on <https://docs.rs/chrono/latest/chrono/index.html>.
    
    Usage: xmltv2rss [OPTIONS] [FILE]
    
    Arguments:
      [FILE]
              Read XMLTV file or from standard input if '-'
    
    Options:
      -d, --feed-date-format <FEED_DATE_FORMAT>
              RSS feed date format. Examples: "%%Y-%%m-%%d", "%%a %%d %%B, %%Y", "%%x"
              
              [default: "%a %d %B, %Y"]
    
          --feed-language <FEED_LANGUAGE>
              RSS feed language
    
      -t, --feed-time-format <FEED_TIME_FORMAT>
              RSS feed time format. Examples: "%%H:%%M", "%%I:%%M %%p", "%%X"
              
              [default: %H:%M]
    
          --feed-title <FEED_TITLE>
              RSS feed title
              
              [default: "XMLTV feed"]
    
          --feed-link <FEED_LINK>
              RSS feed URL
              
              [default: ]
    
          --feed-indent <FEED_INDENT>
              RSS feed indentation
              
              [default: 2]
    
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

    use std::io;
    use xmltv2rss::export::rss::{export, OptionsBuilder};
    
    // let options = Options::default();
    let options = OptionsBuilder::default()
        // .language(string.as_str())
        .language("en")
        .build()?;
    
    let channel = export("Title", "https://example.com/", &options, Some("file.xml"))?;
    
    channel.pretty_write_to(io::stdout(), b' ', 2)?;


License
-------

GPL-3.0 or later


Link
----

[GitHub](https://github.com/willemw12/xmltv2rss-rs)

