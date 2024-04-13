use crate::versions;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "version")]
pub enum VersionedSettings {
    V1(versions::v1::Settings),
}

impl Default for VersionedSettings {
    fn default() -> Self {
        VersionedSettings::V1(versions::v1::Settings::default())
    }
}

impl VersionedSettings {
    pub fn get_mut_settings(&mut self) -> &mut versions::v1::Settings {
        match self {
            VersionedSettings::V1(ref mut settings) => settings,
        }
    }

    pub fn get_settings(&self) -> &versions::v1::Settings {
        match self {
            VersionedSettings::V1(ref settings) => settings,
        }
    }
}
