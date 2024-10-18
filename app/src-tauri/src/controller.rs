use std::io::ErrorKind;

use noteapp_lib::{app_settings::load_config, errors::AppError, storage::StorageType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct FileList{
    pub filename: Option<String>,
    pub modified: Option<u128>,
}

pub async fn load_file_list() -> Result<Vec<FileList>, AppError> {
    let mut result = Vec::new();
    let settings = load_config()?;
    if let Some(dir) = settings.save_files_to_dir {
        let files = StorageType::Disk.get_dir_files(dir.as_str()).await?;
        result = files.iter()
            .filter_map(|e| {
                let ms = e.metadata()
                    .expect("Missing File Metadata")
                    .modified().expect("Missing modified date")
                    .duration_since(std::time::UNIX_EPOCH)
                    .expect("Need duration")
                    .as_millis();
                let filename = e.file_name()
                    .map(|f| f.to_string_lossy().to_string());
                Some(FileList{
                    filename,
                    modified: Some(ms)
                })
            })
            .collect::<Vec<_>>();
    }
    Ok(result)
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ControllerError {
    error: bool,
    message: String,
    code: String,
    status: u16
}
impl ControllerError {
    fn new(message: String, code: &str, status: u16) -> Self {
        Self { error: true, message, code: code.into(), status }
    }
}
impl ControllerError {
    const INVALID_SETTINGS: &str = "IV_S";
    const IO: &str = "IO_E";
    const IO_NOTFOUND: &str = "IO_NF";
}

impl From<AppError> for ControllerError {
    fn from(value: AppError) -> Self {
        match value {
            AppError::Io(err) => ControllerError::new(err.to_string(), ControllerError::IO, 500),
            _ => ControllerError::new("Invalid settings".into(), ControllerError::INVALID_SETTINGS, 500)
        }
    }
}
impl From<std::io::Error> for ControllerError {
    fn from(value: std::io::Error) -> Self {
        if value.kind() == ErrorKind::NotFound {
            ControllerError::new(value.to_string(), ControllerError::IO_NOTFOUND, 404)
        } else {
            ControllerError::new(value.to_string(), ControllerError::IO, 500)
        }
    }
}
