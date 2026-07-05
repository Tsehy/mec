mod cli;
mod commands;
pub mod models;

use clap::Parser;
use cli::{Cli, CliCommands};

fn main() -> anyhow::Result<()> {
    match Cli::parse().command() {
        CliCommands::Init(args) => commands::init::run(args)?,
        CliCommands::AddGame(args) => todo!(),
        CliCommands::Display(args) => todo!(),
        CliCommands::Export(args) => todo!(),
    }
    Ok(())
}
