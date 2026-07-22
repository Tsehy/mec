mod cli;
mod commands;
mod domain;
mod embed;
mod history;

use clap::Parser;
use cli::{Cli, CliCommands};

fn main() -> anyhow::Result<()> {
    match Cli::parse().command() {
        CliCommands::Init(args) => commands::init::run(args)?,
        CliCommands::AddPlayer(args) => commands::add_player::run(&args)?,
        CliCommands::AddGame(args) => commands::add_game::run(args)?,
        CliCommands::Display(args) => commands::display::run(args)?,
        CliCommands::Export(args) => commands::export::run(args)?,
        CliCommands::Undo(args) => commands::undo::run(args)?,
        CliCommands::Redo(args) => commands::redo::run(args)?,
    }
    Ok(())
}
