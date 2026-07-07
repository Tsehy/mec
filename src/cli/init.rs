use clap::Args;

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