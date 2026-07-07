mod cli;
mod commands;
mod models;

use clap::Parser;
use cli::{Cli, CliCommands};

fn main() -> anyhow::Result<()> {
    match Cli::parse().command() {
        CliCommands::Init(args) => commands::init::run(args)?,
        CliCommands::AddGame(args) => commands::add_game::run(args)?,
        CliCommands::Display(_args) => todo!(),
        CliCommands::Export(_args) => todo!(),
    }
    Ok(())
}
