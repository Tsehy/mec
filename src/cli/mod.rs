pub mod init;
pub mod add_game;
pub mod display;
pub mod export;

#[derive(clap::Parser)]
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
    Init(init::InitArgs),
    AddGame(add_game::AddGameArgs),
    Display(display::DisplayArgs),
    Export(export::ExportArgs),
}
