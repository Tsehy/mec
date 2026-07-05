use clap::Args;

#[derive(Args)]
pub struct AddGameArgs {
    #[arg(long, short)]
    season: String,
    #[arg(long, short)]
    date: Option<String>,
    #[arg(default_value_t = false)]
    force: bool,
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

    pub fn players(&self) -> &Vec<String> {
        &self.players
    }
}
