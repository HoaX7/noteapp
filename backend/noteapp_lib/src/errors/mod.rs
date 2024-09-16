use thiserror::Error;
use std::io;
use toml::{ser, de};

#[derive(Error, Debug)]
pub enum AppError {
    #[error("unable to serialize data: {0}")]
    TomlSer(ser::Error),
    #[error("error parsing `{0}`: {1}")]
    TomlDe(String, de::Error),
    #[error("there was an error loading file: {0}")]
    Io(io::Error)
}

impl From<io::Error> for AppError {
    fn from(err: io::Error) -> Self {
        AppError::Io(err)
    }
}