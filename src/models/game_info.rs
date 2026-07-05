use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GameInfo {
    name: String,
    elo_before: u16,
    elo_after: u16,
}