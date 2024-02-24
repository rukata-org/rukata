use crate::argument_builder::GeneralArguments;
use crate::command::Command;
use rukata_settings::SettingsHandler;

pub struct CheckCommand {
    arguments: GeneralArguments,
    settings: Option<SettingsHandler>,
    errors: Vec<String>,
}

impl Command for CheckCommand {
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

impl CheckCommand {
    pub fn new(arguments: GeneralArguments) -> CheckCommand {
        CheckCommand {
            arguments,
            settings: None,
            errors: vec![],
        }
    }
}
