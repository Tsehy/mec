use clap::Args;

#[derive(Args)]
pub struct AddGameArgs {
    #[arg(help = "Season's name")]
    season: String,
    #[arg(long, short, help = "Game date (yyyy-mm-dd) [default: today]")]
    date: Option<String>,
    #[arg(long, short, default_value_t = false, help = "Create not existing players")]
    force: bool,
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

    pub fn force(&self) -> &bool {
        &self.force
    }

    pub fn players(&self) -> &[String] {
        &self.players
    }
}
