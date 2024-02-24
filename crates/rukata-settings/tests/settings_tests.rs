#[macro_use]
extern crate lazy_static;

#[cfg(test)]
use pretty_assertions::{assert_eq, assert_ne};

use camino::Utf8PathBuf;
use rukata_settings::versioned_settings::VersionedSettings;
use rukata_settings::SettingsHandler;
use std::{env, fs};

lazy_static! {
    static ref FILES_DIR: Utf8PathBuf = Utf8PathBuf::from_path_buf(env::current_dir().unwrap())
        .unwrap()
        .join("tests")
        .join("files");
}

fn remove_file_if_exists(path: Utf8PathBuf) {
    if path.exists() {
        fs::remove_file(path).unwrap()
    }
}

#[test]
fn test_default() {
    let settings_handler = SettingsHandler::new(FILES_DIR.join("default.json")).unwrap();
    assert_eq!(
        settings_handler.get_settings(),
        VersionedSettings::default().get_settings()
    )
}

#[test]
fn test_display() {
    let settings_handler = SettingsHandler::new(FILES_DIR.join("default.json")).unwrap();
    assert_eq!(
        format!("{}", settings_handler.get_settings()),
        "{\n  \"directory\": \"\"\n}"
    )
}

#[test]
fn test_path() {
    let settings_handler = SettingsHandler::new(FILES_DIR.join("default.json")).unwrap();
    assert_eq!(settings_handler.get_path(), FILES_DIR.join("default.json"))
}

#[test]
fn test_save() {
    let path = FILES_DIR.join("save.json");
    let _cleanup = scopeguard::guard(path.clone(), remove_file_if_exists);

    let mut settings_handler = SettingsHandler::new(path.clone()).unwrap();
    {
        let settings = settings_handler.get_mut_settings();
        settings.set_directory(path.clone());
    }

    settings_handler.save().unwrap();

    let settings = settings_handler.get_settings();
    let settings_handler_load = SettingsHandler::new(path).unwrap();
    assert_eq!(settings, settings_handler_load.get_settings());
    assert_ne!(settings, VersionedSettings::default().get_settings());
}

#[test]
fn test_missing_version() {
    let path = FILES_DIR.join("missing_version.json");
    assert_eq!(
        SettingsHandler::new(path.clone()).err(),
        Some(format!(
            "Failed to read settings `{}` with error: missing field `version` at line 3 column 1",
            path
        ))
    );
}

#[test]
fn test_missing_directory() {
    let path = FILES_DIR.join("missing_directory.json");
    assert_eq!(
        SettingsHandler::new(path.clone()).err(),
        Some(format!(
            "Failed to read settings `{}` with error: missing field `directory`",
            path
        ))
    );
}

#[test]
fn test_valid_basic() {
    let path = FILES_DIR.join("valid_basic.json");
    assert_eq!(SettingsHandler::new(path.clone()).err(), None);
    let settings_handler = SettingsHandler::new(path.clone()).unwrap();
    {
        let settings = settings_handler.get_settings();
        assert_eq!(settings.get_directory(), Utf8PathBuf::new())
    }
}
