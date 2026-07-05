use chrono::{Local, NaiveDate, NaiveDateTime};
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
            Some(date) => NaiveDateTime::parse_from_str(&date, "%Y-%m-%d").unwrap().date(),
            None => Local::now().date_naive(),
        }
    }

    pub fn elo(&self) -> &u16 {
        &self.elo
    }
}