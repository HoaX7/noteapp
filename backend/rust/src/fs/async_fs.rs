use tokio::fs::File;
use tokio::fs;
use std::path::Path;

async fn _create(path: &str) {
    let path: &Path = Path::new(path);
    let parent: &Path = path.parent().unwrap();
    println!("creating new file under: {:?}", parent);
    fs::create_dir_all(parent).await.unwrap();
    File::create_new(path).await.unwrap();
}
