use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Atom(#[from] atom_syndication::Error),

    #[error(transparent)]
    De(#[from] quick_xml::DeError),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    OptionsBuilder(#[from] crate::export::OptionsBuilderError),

    #[error(transparent)]
    Parse(#[from] chrono::ParseError),

    #[error(transparent)]
    Rss(#[from] rss::Error),
}
