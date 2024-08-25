use std::io::Error;

use super::StorageAbstract;

pub struct Notion {}

impl StorageAbstract for Notion {
    fn new() -> Self {
        println!("Notion storage initialized...");
        Self {}
    }
    async fn get_contents(_path: &str) -> Result<String, Error> {
        Ok(String::from(""))
    }
}