use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use crate::models::game_info::GameInfo;

#[derive(Serialize, Deserialize)]
pub struct Game {
    date: NaiveDate,
    players: Vec<GameInfo>
}