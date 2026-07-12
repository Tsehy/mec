mod cli;
mod commands;
mod models;

use clap::Parser;
use cli::{Cli, CliCommands};

fn main() -> anyhow::Result<()> {
    match Cli::parse().command() {
        CliCommands::Init(args) => commands::init::run(args)?,
        CliCommands::AddPlayer(_args) => todo!(),
        CliCommands::AddGame(args) => commands::add_game::run(args)?,
        CliCommands::RemoveGame(_args) => todo!(),
        CliCommands::Display(args) => commands::display::run(args)?,
        CliCommands::Export(args) => commands::export::run(args)?,
    }
    Ok(())
}
