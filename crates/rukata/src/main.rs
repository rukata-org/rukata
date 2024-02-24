use clap::FromArgMatches;
use rukata::argument_builder::{generate_command, SubCommands};
use rukata::command::{Command, CommandHandler};
use rukata::commands::check::CheckCommand;
use rukata::commands::generate::GenerateCommand;
use rukata::commands::settings::SettingsCommand;
use rukata::commands::solution::SolutionCommand;

fn main() {
    let cmd = generate_command();

    let matches = cmd.get_matches();
    let derived_subcommands = SubCommands::from_arg_matches(&matches)
        .map_err(|err| err.exit())
        .unwrap();

    let command: Box<dyn Command> = match derived_subcommands {
        SubCommands::Check(arguments) => Box::new(CheckCommand::new(arguments)),
        SubCommands::Generate(arguments) => Box::new(GenerateCommand::new(arguments)),
        SubCommands::Solution(arguments) => Box::new(SolutionCommand::new(arguments)),
        SubCommands::Settings(arguments) => Box::new(SettingsCommand::new(arguments)),
    };

    let mut command_handler = CommandHandler::new(command);
    command_handler.run()
}
