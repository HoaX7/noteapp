mod disk;
mod notion;
use std::{io::Error, path::{Path, PathBuf}};

use crate::{app_settings::{get_settings_dir, make_path}, fs};

pub enum StorageType {
    Disk,
    Notion,
}

impl StorageType {
    pub async fn read_strategy(&self, path: &str) -> Result<Option<String>, Error> {
        let file_path = make_path(path);
        match self {
            StorageType::Disk => disk::read_disk(&file_path).await,
            StorageType::Notion => notion::read_notion().await
        }
    }
    pub async fn write_strategy(&self, path: &str, data: &str, append: bool) -> Result<(), Error> {
        let file_path = make_path(path);
        match self {
            StorageType::Disk => disk::write_disk(&file_path, data.as_bytes(), append).await,
            StorageType::Notion => notion::write_notion().await
        }
    }
    pub async fn unlink_strategy(&self, path: &str) ->Result<(), Error> {
        let file_path = make_path(path);
        match self {
            StorageType::Disk => disk::remove_file(&file_path).await,
            StorageType::Notion => notion::remove_file().await
        }
    }
    pub async fn get_dir_files(&self, dir: &str) -> Result<Vec<PathBuf>, std::io::Error> {
        let dir_path = Path::new(dir);
        match self {
            StorageType::Disk => disk::get_dir_files(&dir_path.to_path_buf()).await,
            StorageType::Notion => notion::get_dir_files().await
        }
    }
    pub async fn rename_strategy(&self, from: &str, to: &str) -> Result<(), std::io::Error> {
        let from_path = make_path(from);
        let to_path = make_path(to);
        match self {
            StorageType::Disk => disk::rename_file(&from_path.as_path(), &to_path.as_path()).await,
            StorageType::Notion => notion::rename_file().await
        }
    }
    pub fn search_strategy(&self, query: &str) -> Result<Vec<PathBuf>, ()> {
        let dir = get_settings_dir();
        let dir_path = Path::new(&dir);
        match self {
            StorageType::Disk => fs::search_content(dir_path, query),
            StorageType::Notion => todo!()
        }
    }
}
