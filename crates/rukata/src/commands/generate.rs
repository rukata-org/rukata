use crate::argument_builder::GeneralArguments;
use crate::command::Command;
use crate::common::generate_file;
use crate::validation::validate_settings;
use rukata_puzzle_data::{get_file_data, PuzzleData};
use rukata_settings::SettingsHandler;
use std::fs;

pub struct GenerateCommand {
    arguments: GeneralArguments,
    settings: Option<SettingsHandler>,
    errors: Vec<String>,
}

impl Command for GenerateCommand {
    fn set_settings(&mut self, settings: SettingsHandler) {
        self.settings = Some(settings)
    }

    fn initialize(&mut self) {}

    fn execute(&mut self) {
        let settings_handler = self.settings.as_ref().expect("Failed to set settings");

        // Get the settings
        let settings = settings_handler.get_settings();
        self.errors = validate_settings(settings);
        if !self.errors.is_empty() {
            return;
        }

        // Get the puzzle data.
        let puzzle_id = self.arguments.puzzle_id;
        let puzzle_data: &'static PuzzleData = match get_file_data(puzzle_id) {
            Some(data) => data,
            None => {
                self.errors
                    .push(format!("Puzzle ID is not valid {}", puzzle_id));
                return;
            }
        };

        // Generate the main folder.
        let title = puzzle_data.get_title();
        let folder_name = format!("p{:0>5} - {}", puzzle_id, title);

        let directory = settings.get_directory().join("working").join(folder_name);
        if directory.exists() {
            self.errors
                .push(format!("Directory `{}` already exists", directory));
        }

        if !directory.exists() {
            if let Err(e) = fs::create_dir_all(directory.clone()) {
                self.errors.push(format!(
                    "Failed to create directory `{}` with error: {}",
                    directory, e
                ));
            }
        }

        if !self.errors.is_empty() {
            return;
        }

        // Populate the main folder.
        for file_data in puzzle_data.get_base_files() {
            let file_path = directory.join(file_data.get_relative_path());
            if let Some(error) = generate_file(file_path, file_data.get_raw_data()) {
                self.errors.push(error);
            }
        }

        if !self.errors.is_empty() {
            return;
        }

        // Mark files read-only.
        let read_only_files = puzzle_data.get_read_only_file_paths();
        for file_name in read_only_files {
            let file_path = directory.join(file_name);
            match file_path.metadata() {
                Ok(metadata) => {
                    let mut permissions = metadata.permissions();
                    permissions.set_readonly(true);
                    if let Err(e) = fs::set_permissions(&file_path, permissions) {
                        self.errors.push(format!(
                            "Failed to modify metadata for `{}` with error: {}",
                            file_path, e
                        ));
                    }
                }
                Err(e) => {
                    self.errors.push(format!(
                        "Failed to read metadata for `{}` with error: {}",
                        file_path, e
                    ));
                }
            }
        }
    }

    fn get_errors(&self) -> Vec<String> {
        self.errors.clone()
    }
}

impl GenerateCommand {
    pub fn new(arguments: GeneralArguments) -> Self {
        Self {
            arguments,
            settings: None,
            errors: vec![],
        }
    }
}
