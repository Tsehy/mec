use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GameInfo {
    name: String,
    elo_before: u16,
    elo_after: u16,
}

impl GameInfo {
    pub fn new(name: &String, elo_before: u16, elo_after: u16) -> Self {
        GameInfo {
            name: name.clone(),
            elo_before,
            elo_after,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn elo_before(&self) -> u16 {
        self.elo_before
    }

    pub fn elo_after(&self) -> u16 {
        self.elo_after
    }

    pub fn elo_after_add(&mut self, delta: u16) {
        self.elo_after = self.elo_after.saturating_add(delta);
    }

    pub fn elo_after_sub(&mut self, delta: u16) {
        self.elo_after = self.elo_after.saturating_sub(delta);
    }
}