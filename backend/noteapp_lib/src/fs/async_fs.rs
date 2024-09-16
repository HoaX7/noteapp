use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::io::Error;
use std::path::{Path, PathBuf};

use super::is_supported_file;

pub async fn try_read(path: &PathBuf, create: bool) -> Result<String, Error> {
    let mut file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(create)
        .open(path).await?;

    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    file.flush().await?;
    Ok(contents)
}

pub async fn try_write(path: &PathBuf, data: &[u8], append: bool) -> Result<(), Error> {
    let mut file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(append)
        .truncate(!append)
        .open(path).await?;

    file.write_all(data).await?;
    file.write("\n".as_bytes()).await?;
    file.flush().await?;
    Ok(())
}

#[doc = "Currently only `md`, `txt` and `plain` files are supported"]
pub async fn get_dir_files(dir: &PathBuf) -> Result<Vec<PathBuf>, Error> {
    let mut files = Vec::new();
    if dir.is_dir() {
        let mut traverse_dir = fs::read_dir(dir).await?;
        while let Some(entry) = traverse_dir.next_entry().await? {
            let path = entry.path();
            if is_supported_file(&path) {
                files.push(entry.path())
            }
        }
    };
    Ok(files)
}

pub async fn create_dir_all(path: &PathBuf) -> Result<(), Error> {
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir).await?
    }
    Ok(())
}

pub async fn try_unlink(path: &PathBuf) -> Result<(), Error> {
    fs::remove_file(path).await
}

pub async fn try_rename<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> Result<(), Error> {
    fs::rename(from, to).await
}