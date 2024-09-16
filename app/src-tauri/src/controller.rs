use noteapp_lib::{app_settings::load_config, errors::AppError, storage::StorageType};
use serde::{Deserialize, Serialize};

pub async fn load_file_list() -> Result<Vec<String>, AppError> {
    let mut result = Vec::new();
    let settings = load_config()?;
    if let Some(dir) = settings.save_files_to_dir {
        let files = StorageType::Disk.get_dir_files(dir.as_str()).await?;
        result = files.iter()
            .filter_map(|e| e.file_name()
            .map(|f| f.to_string_lossy().to_string()))
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
        ControllerError::new(value.to_string(), ControllerError::IO, 500)
    }
}
