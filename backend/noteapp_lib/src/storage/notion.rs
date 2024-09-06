use std::io::Error;

pub async fn read_notion(_path: &str) -> Result<Option<String>, Error> {
    Ok(None)
}
pub async fn write_notion(_path: &str, data: &str) -> Result<(), Error> {
    Ok(())
}