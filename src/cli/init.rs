use clap::Args;

#[derive(Args)]
pub struct InitArgs {
    #[arg(long, short)]
    name: String,
    #[arg(long, short)]
    date: Option<String>,
    #[arg(long, short, default_value_t = 1500, required = false)]
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