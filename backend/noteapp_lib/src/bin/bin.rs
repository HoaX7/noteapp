
use noteapp_lib::*;

#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        get_contents().await;
    });
    handle.await.unwrap();
}

pub async fn get_contents() {
    let settings = app_settings::load_config();
    println!("settings: {settings:?}");
    // let files = storage::StorageType::Disk.get_dir_files("../docs").await.unwrap();
    // println!("yoooo {:?}", files);
    // let contents = storage::StorageType::Disk.write_strategy("shortnotes.txt", "Data", false).await.unwrap();
    // println!("{:?}", contents);
    // let contents = storage::StorageType::Disk.read_strategy("shortnotes.txt").await.unwrap();
    // println!("{:?}", contents);
    // let found = storage::StorageType::Disk.search_strategy("test").unwrap();
    // println!("found data {:?}", found);
}
