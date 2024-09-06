use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::io::Error;
use std::path::Path;

pub async fn try_read(path: &str, create: bool) -> Result<String, Error> {
    let file_path = Path::new(path);
    let mut file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(create)
        .open(file_path).await?;

    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    file.flush().await?;
    Ok(contents)
}

pub async fn try_write(path: &str, data: &str, append: bool) -> Result<(), Error> {
    let file_path = Path::new(path);
    let mut file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .append(append)
        .create(true)
        .open(file_path).await?;

    file.write_all(data.as_bytes()).await?;
    file.flush().await?;
    Ok(())
}