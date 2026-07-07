use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Player {
    name: String,
    elo: u16,
}

impl Player {
    pub fn new(name: &String, elo: u16) -> Self {
        Player {
            name: name.clone(),
            elo,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn elo(&self) -> u16 {
        self.elo
    }

    pub fn set_elo(&mut self, new_elo: u16) {
        self.elo = new_elo;
    }
}
