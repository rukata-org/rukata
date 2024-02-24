use crate::argument_builder::GeneralArguments;
use crate::command::Command;
use rukata_settings::SettingsHandler;

pub struct SolutionCommand {
    arguments: GeneralArguments,
    settings: Option<SettingsHandler>,
    errors: Vec<String>,
}

impl Command for SolutionCommand {
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

impl SolutionCommand {
    pub fn new(arguments: GeneralArguments) -> Self {
        Self {
            arguments,
            settings: None,
            errors: vec![],
        }
    }
}
