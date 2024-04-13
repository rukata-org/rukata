use crate::argument_builder::SettingsArguments;
use crate::command::Command;
use crate::common::{print_cyan_title, print_white};
use rukata_settings::SettingsHandler;

pub struct SettingsCommand {
    arguments: SettingsArguments,
    settings: Option<SettingsHandler>,
    errors: Vec<String>,
}

impl Command for SettingsCommand {
    fn set_settings(&mut self, settings: SettingsHandler) {
        self.settings = Some(settings)
    }

    fn initialize(&mut self) {}

    fn execute(&mut self) {
        let settings_handler = self.settings.as_mut().expect("Failed to set settings");

        // Print current settings path.
        print_cyan_title("Current path for settings file:");
        print_white(settings_handler.get_path());

        // Get the settings
        let settings = settings_handler.get_mut_settings();

        // Print current settings.
        print_cyan_title("Current settings data:");
        print_white(format!("{}", settings));

        let mut has_changed = false;

        // Update the directory if provided.
        if let Some(directory) = &self.arguments.directory {
            has_changed = true;
            settings.set_directory(directory);
        }

        // If the settings have changed show the user and update file.
        if has_changed {
            // Print current settings.
            print_cyan_title("New settings data:");
            print_white(format!("{}", settings));

            // Save the settings.
            if let Err(e) = settings_handler.save() {
                self.errors.push(e);
            }
        }
    }

    fn get_errors(&self) -> Vec<String> {
        self.errors.clone()
    }
}

impl SettingsCommand {
    pub fn new(arguments: SettingsArguments) -> Self {
        Self {
            arguments,
            settings: None,
            errors: vec![],
        }
    }
}
