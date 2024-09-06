mod disk;
mod notion;
use std::io::Error;

pub enum StorageType {
    Disk,
    Notion,
}

impl StorageType {
    pub async fn read_strategy(&self, path: &str) -> Result<Option<String>, Error> {
        match self {
            StorageType::Disk => disk::read_disk(path).await,
            StorageType::Notion => notion::read_notion(path).await
        }
    }
    pub async fn write_strategy(&self, path: &str, data: &str) -> Result<(), Error> {
        match self {
            StorageType::Disk => disk::write_disk(path, data).await,
            StorageType::Notion => notion::write_notion(path, data).await
        }
    }
}
