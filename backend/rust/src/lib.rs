mod storage;
mod fs;
mod app_settings;
use storage::*;

pub async fn lib_main() {
    let config = app_settings::get_config();
    println!("{config:?}")
    // let _result = StorageManager::new(true);
    // let contents = result.get_contents("abc").await;
    // match contents {
    //     Ok(res) => println!("contents: {}", res),
    //     Err(e) => println!("error {:?}", e)
    // };
}