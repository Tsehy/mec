use clap::Args;

#[derive(Args)]
pub struct RemovePlayerArgs {
    #[arg(help = "Season's name")]
    season: String,
    #[arg(help = "Player's name")]
    name: String,
}

impl RemovePlayerArgs {
    pub fn season(&self) -> &str {
        &self.season
    }
    
    pub fn name(&self) -> &str {
        &self.name
    }
}