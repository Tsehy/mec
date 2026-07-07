use clap::Args;

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