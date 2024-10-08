#[allow(unused_imports)]
use std::path::{Path, PathBuf, absolute};

use crate::{fs, errors::AppError};
use serde::{Deserialize, Serialize};

#[cfg(target_os = "windows")]
const SETTINGS_PATH: &str = "./settings.toml";
#[cfg(target_os = "windows")]
const DEFAULT_DIR: &str = "./docs";
#[cfg(target_os = "macos")]
const SETTINGS_PATH: &str = "scribe/settings.toml";
#[cfg(target_os = "macos")]
const DEFAULT_DIR: &str = "scribe/docs";
const DEFAULT_EXT: &str = "md";

fn get_absolute_path(path: &str) -> PathBuf {
    #[cfg(target_os = "macos")]
    if let Some(pathbuf) = dirs::home_dir() {
        pathbuf.join(path)
    } else {
        PathBuf::from(path)
    }
    #[cfg(target_os = "windows")]
    absolute(path).unwrap()
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AppSettings {
    pub save_files_to_dir: Option<String>,
    pub notion: Option<bool>
}

impl Default for AppSettings {
    fn default() -> Self {
        Self { save_files_to_dir: Some(get_absolute_path(DEFAULT_DIR).to_string_lossy().to_string()), notion: None }
    }
}

#[doc = "Load config from `settings.toml` file."]
#[doc = ""]
#[doc = "The config struct is cached and refreshed on file change."]
pub fn load_config() -> Result<AppSettings, AppError> {
    let path = get_absolute_path(SETTINGS_PATH);
    let contents = fs::try_read(path.as_path(), true)
        .map_err(AppError::Io)?;

    let mut settings = toml::from_str(&contents)
        .map_err(|e| AppError::TomlDe(SETTINGS_PATH.to_string(), e))?;

    verify_settings(&mut settings);
    Ok(settings)
}

#[doc = "Joins the `AppSettings` base dir and specified path"]
pub fn make_path(path: &str) -> PathBuf {
    let dir = get_settings_dir();
    let dir_path = Path::new(&dir);
    let mut file_path = dir_path.join(path);
    if file_path.extension().is_none() {
        file_path.set_extension(DEFAULT_EXT);
    }
    file_path
}

pub fn get_settings_dir() -> String {
    let settings = load_config().unwrap_or(AppSettings::default());
    settings.save_files_to_dir.unwrap_or(get_absolute_path(DEFAULT_DIR).to_string_lossy().to_string())
}

#[doc = "Saves your settings to `settings.toml` file"]
pub fn save_settings(settings: &AppSettings) -> Result<(), AppError> {
    let contents = toml::to_string(settings)
        .map_err(AppError::TomlSer)?;
    fs::try_write(get_absolute_path(SETTINGS_PATH).as_path(), &contents.as_bytes(), false)?;
    Ok(())
}

#[doc = "Verify `settings.toml` file."]
#[doc = ""]
#[doc = "If the config attributes are missing the file will be overwritten with default values."]
fn verify_settings(settings: &mut AppSettings) {
    let mut rewrite_settings = false;
    if settings.save_files_to_dir.is_none() {
        settings.save_files_to_dir = AppSettings::default().save_files_to_dir;
        rewrite_settings = true;
    };
    if rewrite_settings {
        let err = save_settings(&settings).err();
        if err.is_some() {
            println!("unable to rewrite settings: {err:?}");
        }
    }
}
