use std::fmt;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Parse(String),
    Config(String),
    Template(String),
    Lua(String),
    Module {
        name: &'static str,
        source: Box<Error>,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(e) => write!(f, "I/O: {e}"),
            Error::Parse(s) => write!(f, "parse: {s}"),
            Error::Config(s) => write!(f, "config: {s}"),
            Error::Template(s) => write!(f, "template: {s}"),
            Error::Lua(s) => write!(f, "lua: {s}"),
            Error::Module { name, source } => write!(f, "module {name}: {source}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
