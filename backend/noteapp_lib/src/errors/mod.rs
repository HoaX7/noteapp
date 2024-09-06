use thiserror::Error;
use std::io;
use toml::{ser, de};

#[derive(Error, Debug)]
pub enum ConfigErrorKind {
    #[error("unable to serialize data: {0}")]
    TomlSer(ser::Error),
    #[error("error parsing `{0}`: {1}")]
    TomlDe(String, de::Error),
    #[error("there was an error reading config file: {0}")]
    Io(io::Error)
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error(transparent)]
    ConfigError(#[from] ConfigErrorKind)
}

impl From<io::Error> for ConfigErrorKind {
    fn from(err: io::Error) -> Self {
        ConfigErrorKind::Io(err)
    }
}