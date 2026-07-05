mod cli;

use clap::Parser;
use cli::CliCommands;

fn main() {
    match cli::Cli::parse().command() {
        CliCommands::Init(args) => {},
        CliCommands::AddGame(args) => {},
        CliCommands::Display(args) => {},
        CliCommands::Export(args) => {},
    }
}