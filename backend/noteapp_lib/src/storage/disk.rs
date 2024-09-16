use std::{io::{Error, ErrorKind}, path::{Path, PathBuf}};
use crate::fs;

pub async fn read_disk(path: &PathBuf) -> Result<Option<String>, Error> {
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

pub async fn write_disk(path: &PathBuf, data: &[u8], append: bool) -> Result<(), Error> {
    fs::async_fs::create_dir_all(path).await?;
    fs::async_fs::try_write(path, data, append).await
}

pub async fn get_dir_files(dir: &PathBuf) -> Result<Vec<PathBuf>, Error> {
    fs::async_fs::get_dir_files(dir).await
}

pub async fn remove_file(path: &PathBuf) -> Result<(), Error> {
    fs::async_fs::try_unlink(path).await
}

async fn read(path: &PathBuf) -> std::io::Result<String> {
    let contents: String = fs::async_fs::try_read(path, false).await?;
    Ok(contents)
}

pub async fn rename_file(from: &Path, to: &Path) -> Result<(), Error> {
    fs::async_fs::try_rename(from, to).await
}
