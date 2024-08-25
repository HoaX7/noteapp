pub mod async_fs;
use std::fs;
use std::io::{Error, Read};

pub fn try_read(path: &str, create: bool) -> Result<String, Error> {
    let mut file = fs::OpenOptions::new()
        .read(true)
        .create_new(create)
        .open(path)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}