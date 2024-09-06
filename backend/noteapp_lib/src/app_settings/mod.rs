use crate::{fs, errors::ConfigErrorKind};
use serde::{Deserialize, Serialize};

const SETTINGS_PATH: &str = "./settings.toml";

#[allow(dead_code)]
#[derive(Deserialize, Serialize, Debug)]
pub struct AppSettings {
    pub save_files_to_dir: Option<String>,
    pub notion: Option<bool>
}

impl Default for AppSettings {
    fn default() -> Self {
        Self { save_files_to_dir: Some("./".to_string()), notion: None }
    }
}

#[doc = "Load config from `settings.toml` file."]
#[doc = "The config struct is cached and refreshed on file change."]
pub fn load_config() -> Result<AppSettings, ConfigErrorKind> {
    let contents = fs::try_read(SETTINGS_PATH, true)
        .map_err(ConfigErrorKind::Io)?;

    let mut settings = toml::from_str(&contents)
        .map_err(|e| ConfigErrorKind::TomlDe(SETTINGS_PATH.to_string(), e))?;

    verify_settings(&mut settings);
    Ok(settings)
}

#[doc = "Verify `settings.toml` file."]
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

fn save_settings(settings: &AppSettings) -> Result<(), ConfigErrorKind> {
    let contents = toml::to_string(settings)
        .map_err(ConfigErrorKind::TomlSer)?;
    fs::try_write(SETTINGS_PATH, &contents, false)?;
    Ok(())
}
