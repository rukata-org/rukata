use camino::Utf8PathBuf;
use clap::{Command, Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct GeneralArguments {
    /// Puzzle ID to use
    pub puzzle_id: u16,
}

#[derive(Parser, Debug)]
pub struct SettingsArguments {
    /// Directory for Rukata to use
    #[arg(short, long)]
    pub directory: Option<Utf8PathBuf>,
}

#[derive(Subcommand, Debug)]
pub enum SubCommands {
    /// Generates the specified Puzzle ID
    Generate(GeneralArguments),
    /// Checks the specified Puzzle ID
    Check(GeneralArguments),
    /// Generates the solution to the specified Puzzle ID
    Solution(GeneralArguments),
    /// Modify settings used by Rukata
    Settings(SettingsArguments),
}

pub fn generate_command() -> Command {
    let cli = Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .arg_required_else_help(true);

    SubCommands::augment_subcommands(cli)
}
