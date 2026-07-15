pub mod init;
pub mod add_game;
pub mod display;
pub mod export;
pub mod add_player;

#[derive(clap::Parser)]
#[command(about = "Mahjong Elo Calculator")]
pub struct Cli {
    #[command(subcommand)]
    command: CliCommands,
}

impl Cli {
    pub fn command(&self) -> &CliCommands {
        &self.command
    }
}

#[derive(clap::Subcommand)]
pub enum CliCommands {
    #[command(about = "Initialize a new season")]
    Init(init::InitArgs),
    #[command(alias = "ap", about = "Add a player to a season")]
    AddPlayer(add_player::AddPlayerArgs),
    #[command(alias = "ag", about = "Add game to a season")]
    AddGame(add_game::AddGameArgs),
    #[command(alias = "dsp", about = "Display information for a season")]
    Display(display::DisplayArgs),
    #[command(alias = "ex", about = "Generate embed json template for a season")]
    Export(export::ExportArgs),
}
