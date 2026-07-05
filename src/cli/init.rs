use chrono::{DateTime, Local, NaiveDate};
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

    pub fn date(&self) -> NaiveDate {
        match &self.date {
            Some(date) => DateTime::parse_from_rfc3339(date).unwrap().date_naive(),
            None => Local::now().date_naive(),
        }
    }

    pub fn elo(&self) -> &u16 {
        &self.elo
    }
}