use noteapp_lib::*;

#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        get_contents().await;
    });
    handle.await.unwrap();
}

pub async fn get_contents() {
    let config = app_settings::load_config().unwrap();
    
}
