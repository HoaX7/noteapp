pub mod async_fs;
use std::fs;
use std::io::{Error, Read, Write};
use std::path::Path;

pub fn try_read(path: &str, create: bool) -> Result<String, Error> {
    let file_path = Path::new(path);
    let mut file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(create)
        .open(file_path)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn try_write(path: &str, data: &str, append: bool) -> Result<(), Error> {
    let file_path = Path::new(path);
    let mut file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .append(append)
        .create(true)
        .open(file_path)?;

    file.write_all(data.as_bytes())
}