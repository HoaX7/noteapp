use crate::fs;
use serde::Deserialize;

const _SETTINGS_PATH: &str = "./settings.toml";

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct AppSettings {
    save_files_to_dir: Option<String>
}

#[doc = "Load config from `settings.toml` file."]
#[doc = "The config struct is cached and refreshed on file change."]
pub fn get_config() -> AppSettings {
    let contents = fs::try_read(_SETTINGS_PATH, false).unwrap();
    toml::from_str(&contents).unwrap()
}
