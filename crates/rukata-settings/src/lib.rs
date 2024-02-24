use crate::versioned_settings::VersionedSettings;
use camino::Utf8PathBuf;
use std::fs;

pub mod versioned_settings;
pub mod versions;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "test-utils", derive(Default))]
pub struct SettingsHandler {
    settings: VersionedSettings,
    path: Utf8PathBuf,
}

impl SettingsHandler {
    pub fn new(path: Utf8PathBuf) -> Result<SettingsHandler, String> {
        if path.is_relative() {
            return Err(format!("Given scan_config path is relative `{}`", path));
        }

        match fs::read_to_string(&path) {
            Ok(s) => match serde_json::from_str::<VersionedSettings>(&s) {
                Ok(settings) => Ok(SettingsHandler { settings, path }),
                Err(e) => Err(format!(
                    "Failed to read settings `{}` with error: {}",
                    path, e
                )),
            },
            Err(_e) => Ok(SettingsHandler {
                settings: VersionedSettings::default(),
                path,
            }),
        }
    }

    pub fn get_path(&self) -> Utf8PathBuf {
        self.path.clone()
    }

    pub fn save(&self) -> Result<(), String> {
        let json_string = match serde_json::to_string_pretty(&self.settings) {
            Ok(s) => s,
            Err(e) => {
                return Err(format!(
                    "Failed to convert settings to String with error: {}",
                    e
                ))
            }
        };

        let folder_path = match self.path.parent() {
            Some(p) => p,
            None => return Err(format!("Failed to obtain parent of path `{}`", self.path)),
        };

        if let Err(e) = fs::create_dir_all(folder_path) {
            return Err(format!(
                "Failed to create folder path `{}` with error: {}",
                folder_path, e
            ));
        }

        if let Err(e) = fs::write(&self.path, json_string) {
            return Err(format!("Failed to save `{}` with error: {}", self.path, e));
        }

        Ok(())
    }

    pub fn get_mut_settings(&mut self) -> &mut versions::v1::Settings {
        self.settings.get_mut_settings()
    }

    pub fn get_settings(&self) -> &versions::v1::Settings {
        self.settings.get_settings()
    }
}
