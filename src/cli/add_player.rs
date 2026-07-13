use clap::Args;

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