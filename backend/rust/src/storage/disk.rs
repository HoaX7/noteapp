use tokio::fs;
use std::io::{Error, ErrorKind};


use super::StorageAbstract;

pub struct Disk {}

impl StorageAbstract for Disk {
    fn new() -> Self {
        println!("disk storage initialized...");
        Self {}
    }
    async fn get_contents(path: &str) -> Result<String, Error> {
        let result: Result<String, Error> = _read(path).await;
        match result {
            Ok(contents) => Ok(contents),
            Err(err) => {
                if err.kind() == ErrorKind::NotFound {
                    return Ok(String::new());
                }
                Err(err)
            }
        }
    }
}

async fn _read(path: &str) -> std::io::Result<String> {
    let contents: String = fs::read_to_string(path).await?;
    Ok(contents)
}
