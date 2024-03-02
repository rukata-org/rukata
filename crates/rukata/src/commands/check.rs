use crate::argument_builder::GeneralArguments;
use crate::command::Command;
use crate::common::print_green;
use crate::validation::validate_settings;
use rukata_puzzle_data::{get_file_data, PuzzleData};
use rukata_settings::SettingsHandler;
use std::fs;

#[cfg(windows)]
static CARGO_CMD: &str = "cargo.cmd";
#[cfg(not(windows))]
static CARGO_CMD: &str = "cargo";

pub struct CheckCommand {
    arguments: GeneralArguments,
    settings: Option<SettingsHandler>,
    errors: Vec<String>,
}

impl Command for CheckCommand {
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

        // Get folder path.
        let title = puzzle_data.get_title();
        let folder_name = format!("p{:0>5} - {}", puzzle_id, title);
        let directory = settings.get_directory().join("working").join(folder_name);

        // Check folder existing.
        if !directory.exists() {
            self.errors
                .push(format!("Directory `{}` does not exist", directory));
        }

        if !self.errors.is_empty() {
            return;
        }

        // Check files.
        let read_only_files = puzzle_data.get_read_only_files();
        for file_data in read_only_files {
            let file_path = directory.join(file_data.get_relative_path());
            if !file_path.exists() {
                self.errors
                    .push(format!("File `{}` does not exist", file_path));
            } else {
                match fs::read(&file_path) {
                    Ok(file_content) => {
                        if !file_data.check_data(&file_content) {
                            self.errors
                                .push(format!("File `{}` does not match stored data", file_path));
                        }
                    }
                    Err(e) => {
                        self.errors.push(format!(
                            "Failed to read file `{}` with error: {} ",
                            file_path, e
                        ));
                    }
                }
            }
        }

        if !self.errors.is_empty() {
            return;
        }

        // Run cargo test.
        let cargo_exit_status = std::process::Command::new(CARGO_CMD)
            .args(["test"])
            .current_dir(directory)
            .output();
        match cargo_exit_status {
            Ok(output) => {
                if !output.status.success() {
                    self.errors.push("Run `cargo test` has failed".to_string());
                }
            }
            Err(e) => {
                self.errors
                    .push(format!("Failed to run `cargo test` with error: {} ", e));
            }
        }

        if !self.errors.is_empty() {
            return;
        }

        print_green(format!("You have completed puzzle {}", puzzle_id))
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
