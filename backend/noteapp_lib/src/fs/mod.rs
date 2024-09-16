pub mod async_fs;
use std::ffi::OsStr;
use std::fs;
use std::io::{Error, Read, Write};
use std::path::{Path, PathBuf};

#[doc = "Currently only `md`, `txt` and `plain` files are supported"]
pub fn is_supported_file(path: &PathBuf) -> bool {
    let ext = path.extension().and_then(OsStr::to_str);
    path.is_file() && (ext.is_none() || ext == Some("md") || ext == Some("txt"))
}

pub fn try_read(path: &Path, create: bool) -> Result<String, Error> {
    let mut file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(create)
        .open(path)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn try_write(path: &Path, data: &[u8], append: bool) -> Result<(), Error> {
    let mut file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .append(append)
        .create(true)
        .truncate(!append)
        .open(path)?;

    file.write_all(data)?;
    file.write("\n".as_bytes())?;
    file.flush()?;
    Ok(())
}

#[allow(dead_code)]
pub fn try_rename<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> Result<(), Error> {
    fs::rename(from, to)
}

#[allow(dead_code)]
pub fn try_unlink(path: &PathBuf) -> Result<(), Error> {
    fs::remove_file(path)
}

#[allow(dead_code)]
pub fn get_dir_files(dir: &Path) -> Result<Vec<PathBuf>, Error> {
    let mut files = Vec::new();
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if is_supported_file(&path) {
                files.push(entry.path())
            }
        }
    }
    Ok(files)
}

#[allow(dead_code)]
pub async fn create_dir_all(path: &Path) -> Result<(), Error> {
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir)?
    }
    Ok(())
}
