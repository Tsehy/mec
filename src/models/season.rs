use chrono::{NaiveDate};
use serde::{Deserialize, Serialize};
use crate::cli::init::InitArgs;
use crate::models::game::Game;
use crate::models::player::Player;

#[derive(Serialize, Deserialize)]
pub struct Season {
    name: String,
    date: NaiveDate,
    players: Vec<Player>,
    games: Vec<Game>,
}

impl From<&InitArgs> for Season {
    fn from(value: &InitArgs) -> Self {
        Season {
            name: value.name().to_string(),
            date: value.date(),
            players: Vec::new(),
            games: Vec::new(),
        }
    }
}