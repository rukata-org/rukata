use camino::Utf8PathBuf;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Settings {
    pub(crate) directory: Utf8PathBuf,
}

impl fmt::Display for Settings {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}

impl Settings {
    pub fn get_directory(&self) -> Utf8PathBuf {
        self.directory.clone()
    }

    pub fn set_directory<P: Into<Utf8PathBuf>>(&mut self, path: P) {
        self.directory = path.into()
    }
}
