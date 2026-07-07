use crate::models::game_info::GameInfo;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Game {
    date: NaiveDate,
    players: Vec<GameInfo>,
}

impl Game {
    pub fn new(date: NaiveDate, players: Vec<GameInfo>) -> Self {
        Game { date, players }
    }
    
    pub fn date(&self) -> NaiveDate {
        self.date
    }
    
    pub fn players(&self) -> &[GameInfo] {
        &self.players
    }
}
