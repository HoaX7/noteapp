use noteapp_lib::*;

#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        get_contents().await;
    });
    handle.await.unwrap();
}

pub async fn get_contents() {
    // let files = storage::StorageType::Disk.get_dir_files("../docs").await.unwrap();
    // println!("yoooo {:?}", files);
    let contents = storage::StorageType::Disk.write_strategy("shortnotes.txt", "Data", false).await.unwrap();
    println!("{:?}", contents);
    // let contents = storage::StorageType::Disk.read_strategy("shortnotes.txt").await.unwrap();
    // println!("{:?}", contents);
}
