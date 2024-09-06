use std::io::{Error, ErrorKind};
use crate::fs;

pub async fn read_disk(path: &str) -> Result<Option<String>, Error> {
    let result = read(path).await;
    match result {
        Ok(contents) => Ok(Some(contents)),
        Err(err) => {
            if err.kind() == ErrorKind::NotFound {
                return Ok(None);
            }
            Err(err)
        }
    }
}

pub async fn write_disk(path: &str, data: &str) -> Result<(), Error> {
    fs::async_fs::try_write(path, data, false).await
}

async fn read(path: &str) -> std::io::Result<String> {
    let contents: String = fs::async_fs::try_read(path, true).await?;
    Ok(contents)
}
