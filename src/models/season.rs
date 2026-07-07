use chrono::{Local, NaiveDate};
use serde::{Deserialize, Serialize};
use crate::cli::init::InitArgs;
use crate::models::game::Game;
use crate::models::player::Player;

#[derive(Serialize, Deserialize)]
pub struct Season {
    name: String,
    date: NaiveDate,
    start_elo: u16,
    players: Vec<Player>,
    games: Vec<Game>,
}

impl Season {
    pub fn name(&self) -> &str {
        &self.name
    }
    
    pub fn date(&self) -> NaiveDate {
        self.date
    }
    
    pub fn players(&self) -> &[Player] {
        &self.players
    }

    pub fn players_mut(&mut self) -> &mut Vec<Player> {
        &mut self.players
    }

    pub fn games(&self) -> &[Game] {
        &self.games
    }
    
    pub fn games_mut(&mut self) -> &mut Vec<Game> {
        &mut self.games
    }

    pub fn start_elo(&self) -> &u16 {
        &self.start_elo
    }
}

impl TryFrom<&InitArgs> for Season {
    type Error = chrono::format::ParseError;
    fn try_from(value: &InitArgs) -> Result<Self, Self::Error> {
        let date = match value.date() {
            Some(date) => NaiveDate::parse_from_str(&date, "%Y-%m-%d")?,
            None => Local::now().date_naive(),
        };

        Ok(Season {
            name: value.name().to_string(),
            date,
            start_elo: *value.elo(),
            players: Vec::new(),
            games: Vec::new(),
        })
    }
}