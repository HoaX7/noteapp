mod disk;
mod notion;

use disk::*;
use notion::*;
use std::{future::Future, io::Error};

trait StorageAbstract {
    fn new() -> Self;
    fn get_contents(path: &str) -> impl Future<Output = Result<String, Error>>;
}

pub struct StorageManager{
    storage: StorageType
}
impl StorageManager {
    pub fn new(is_disk: bool) -> Self {
        // Check cfg if user is connected to notion
       let storage: StorageType = if is_disk {
        StorageType::Disk(Disk::new())
       } else {
        StorageType::Notion(Notion::new())
       };
       Self { storage }
    }
    pub async fn get_contents(&self, path: &str) -> Result<String, Error> {
        let result = match &self.storage {
            StorageType::Disk(_) => {
                Disk::get_contents(path).await?
            },
            StorageType::Notion(_) => {
                Notion::get_contents(path).await?
            }
        };

        Ok(result)
    }
}

enum StorageType {
    Disk(Disk),
    Notion(Notion)
}
