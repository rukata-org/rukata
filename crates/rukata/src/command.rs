use crate::common::{print_red_title, print_white};
use camino::Utf8PathBuf;
use rukata_settings::SettingsHandler;

fn get_config_path() -> Result<Utf8PathBuf, String> {
    let config = dirs::config_dir();

    match config {
        Some(path) => {
            match Utf8PathBuf::try_from(path) {
                Ok(utf8_path) => {
                    Ok(utf8_path.join(concat!("dev.engineern.", env!("CARGO_PKG_NAME"))))
                }

                // This should never happen...
                _ => Err("Failed to convert config path to UTF8.".into()),
            }
        }

        // This should never happen...
        _ => Err("Failed to find config path.".into()),
    }
}

fn get_settings_file_path() -> Result<Utf8PathBuf, String> {
    match get_config_path() {
        Ok(path) => Ok(path.join("settings.json")),
        Err(e) => Err(e),
    }
}

pub(crate) fn get_settings() -> Result<SettingsHandler, String> {
    SettingsHandler::new(get_settings_file_path()?)
}

pub trait Command {
    fn set_settings(&mut self, settings: SettingsHandler);
    fn initialize(&mut self);
    fn execute(&mut self);
    fn get_errors(&self) -> Vec<String>;
}

pub struct CommandHandler {
    command: Box<dyn Command>,
}

impl CommandHandler {
    pub fn new(command: Box<dyn Command>) -> Self {
        Self { command }
    }

    pub fn run(&mut self) {
        let settings_handler = match get_settings() {
            Ok(handler) => handler,
            Err(e) => {
                println!("{}", e);
                return;
            }
        };

        self.command.set_settings(settings_handler.clone());

        self.command.initialize();
        let errors = self.command.get_errors();
        if !errors.is_empty() {
            print_red_title("The following errors occurred:");
            for error in errors {
                print_white(format!("- {}", error));
            }
            return;
        }

        self.command.execute();
        let errors = self.command.get_errors();
        if !errors.is_empty() {
            print_red_title("The following errors occurred:");
            for error in errors {
                print_white(format!("- {}", error));
            }
        }
    }
}
