use std::path::PathBuf;

use noteapp_lib::{app_settings, storage::StorageType};

use crate::{controller::*, shortcuts};

#[tauri::command]
pub async fn save_content(path: &str, text: &str, append: bool) -> Result<(), ControllerError> {
    StorageType::Disk.write_strategy(path, text, append).await?;
    Ok(())
}

#[tauri::command]
pub async fn get_content(path: &str) -> Result<Option<String>, ControllerError> {
    let contents = StorageType::Disk.read_strategy(path).await?;
    Ok(contents)
}

#[tauri::command]
pub async fn get_file_list() -> Result<Vec<String>, ControllerError> {
    Ok(load_file_list().await?)
}

#[tauri::command]
pub async fn delete_file(path: &str) -> Result<(), ControllerError> {
    StorageType::Disk.unlink_strategy(path).await?;
    Ok(())
}

#[tauri::command]
pub async fn rename_file(cur_name: &str, new_name: &str) -> Result<(), ControllerError> {
    if cur_name != new_name {
        StorageType::Disk.rename_strategy(cur_name, new_name).await?;
    }
    Ok(())
}

#[tauri::command]
pub async fn open_shortnote_window(app: tauri::AppHandle) {
    shortcuts::make_quicknote_window(&app);
}

#[tauri::command]
pub async fn get_settings() -> Result<app_settings::AppSettings, ControllerError> {
    Ok(app_settings::load_config()?)
}

#[tauri::command]
pub async fn save_settings(new_settings: app_settings::AppSettings) -> Result<(), ControllerError> {
    let mut current_settings = app_settings::load_config()?;
    let mut has_settings_changed = false;
    if new_settings.save_files_to_dir.is_some() {
        current_settings.save_files_to_dir = new_settings.save_files_to_dir;
        has_settings_changed = true;
    };
    if has_settings_changed {
        app_settings::save_settings(&current_settings)?;
    }
    Ok(())
}

#[tauri::command]
pub fn search_contents(query: &str) -> Result<Vec<PathBuf>, ()> {
    StorageType::Disk.search_strategy(query)
}
