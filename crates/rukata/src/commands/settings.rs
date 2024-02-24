use crate::argument_builder::SettingsArguments;
use crate::command::Command;
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

    fn initialize(&mut self) {
        todo!()
    }

    fn execute(&mut self) {
        todo!()
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
