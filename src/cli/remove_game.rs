use clap::Args;

#[derive(Args)]
pub struct RemoveGameArgs {
    #[arg(help = "Season's name")]
    season: String,
    #[arg(default_value_t = 1, help = "Last N game to remove")]
    count: u32,
}

impl RemoveGameArgs {
    pub fn season(&self) -> &str {
        &self.season
    }
    
    pub fn count(&self) -> u32 {
        self.count
    }
}