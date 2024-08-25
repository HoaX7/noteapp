use noteapp_lib::*;

#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        lib_main().await;
    });
    handle.await.unwrap();
}