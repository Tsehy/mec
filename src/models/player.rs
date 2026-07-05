use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Player {
    name: String,
    elo: u16,
}