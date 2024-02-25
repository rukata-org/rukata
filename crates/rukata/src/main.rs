use clap::FromArgMatches;
use rukata::argument_builder::{generate_command, SubCommands};

fn main() {
    let cmd = generate_command();

    let matches = cmd.get_matches();
    let derived_subcommands = SubCommands::from_arg_matches(&matches)
        .map_err(|err| err.exit())
        .unwrap();

    match derived_subcommands {
        SubCommands::Check(args) => {
            println!("{args:?}");
        }
        SubCommands::Generate(args) => {
            println!("{args:?}");
        }
        SubCommands::Solution(args) => {
            println!("{args:?}");
        }
        SubCommands::Settings(args) => {
            println!("{args:?}");
        }
    }
}
