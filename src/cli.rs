use clap::Args;

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
    Init(InitArgs),
    #[command(alias = "ap", about = "Add a player to a season")]
    AddPlayer(AddPlayerArgs),
    #[command(alias = "ag", about = "Add game to a season")]
    AddGame(AddGameArgs),
    #[command(alias = "dsp", about = "Display information for a season")]
    Display(DisplayArgs),
    #[command(alias = "ex", about = "Generate embed json template for a season")]
    Export(ExportArgs),
}

#[derive(Args)]
pub struct InitArgs {
    #[arg(help = "New season's name")]
    name: String,
    #[arg(long, short, help = "Starting date (yyyy-mm-dd) [default: today]")]
    date: Option<String>,
    #[arg(long, short, default_value_t = 1500, required = false, help = "Starting ELO of the new players")]
    elo: u16,
}

impl InitArgs {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn date(&self) -> &Option<String> {
        &self.date
    }

    pub fn elo(&self) -> &u16 {
        &self.elo
    }
}

#[derive(Args)]
pub struct AddPlayerArgs {
    #[arg(help = "Season's name")]
    season: String,
    #[arg(help = "Player's name")]
    name: String
}

impl AddPlayerArgs {
    pub fn season(&self) -> &str {
        &self.season
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}

#[derive(Args)]
pub struct AddGameArgs {
    #[arg(help = "Season's name")]
    season: String,
    #[arg(long, short, help = "Game date (yyyy-mm-dd) [default: today]")]
    date: Option<String>,
    #[arg(num_args = 4, help = "Players in order, first to last")]
    players: Vec<String>,
}

impl AddGameArgs {
    pub fn season(&self) -> &String {
        &self.season
    }

    pub fn date(&self) -> &Option<String> {
        &self.date
    }

    pub fn players(&self) -> &[String] {
        &self.players
    }
}

#[derive(Args)]
pub struct DisplayArgs {
    #[arg(help = "Season's name")]
    season: String,
    #[arg(long, short, default_value_t = 0, help = "Last N games to display, 0 to display all")]
    count: u8,
}

impl DisplayArgs {
    pub fn season(&self) -> &str {
        &self.season
    }

    pub fn count(&self) -> u8 {
        self.count
    }
}

#[derive(Args)]
pub struct ExportArgs {
    #[arg(help = "Season's name")]
    season: String,
    #[arg(long, short = 'n', default_value_t = 1, help = "Last N games to export, 0 to get only the summary")]
    count: u8,
    #[arg(long, short, default_value_t = false, help = "Make game files inline")]
    inline: bool,
    #[arg(long, short, default_value_t = 0x8D0404, help = "Color of the margin (HEX)")]
    color: u32,
}

impl ExportArgs {
    pub fn season(&self) -> &str {
        &self.season
    }

    pub fn count(&self) -> u8 {
        self.count
    }

    pub fn inline(&self) -> bool {
        self.inline
    }

    pub fn color(&self) -> u32 {
        self.color
    }
}
