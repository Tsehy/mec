use crate::cli::InitArgs;
use chrono::{Local, NaiveDate};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Serde(#[from] serde_json::error::Error),
}

#[derive(serde::Serialize, serde::Deserialize)]
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

    pub fn sort_players(&mut self) {
        self.players_mut().sort_by(|a, b| b.elo().cmp(&a.elo()));
    }

    pub fn get_game_counts(&self) -> HashMap<&str, u32> {
        let player_names = self
            .games()
            .iter()
            .flat_map(|game| game.players())
            .map(|player| player.name());

        let mut game_count: HashMap<&str, u32> = HashMap::new();
        for player_name in player_names {
            *game_count.entry(player_name).or_insert(0) += 1;
        }
        game_count
    }

    pub fn get_max_name_length(&self) -> usize {
        self.players()
            .iter()
            .map(|player| player.name().chars().count())
            .max()
            .unwrap_or(0)
    }

    pub fn load(name: &str) -> Result<Season, DomainError> {
        let file_path = format!("{}.json", name);
        let json = std::fs::read_to_string(file_path)?;
        let season: Season = serde_json::from_str(&json)?;
        Ok(season)
    }

    pub fn save_to_file(self) -> Result<(), DomainError> {
        let file_path = format!("{}.json", self.name);
        let json = serde_json::to_string(&self)?;
        std::fs::write(file_path, &json)?;
        Ok(())
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
            name: value.name().to_owned(),
            date,
            start_elo: *value.elo(),
            players: Vec::new(),
            games: Vec::new(),
        })
    }
}

#[derive(Serialize, Deserialize)]
pub struct Player {
    name: String,
    elo: u16,
}

impl Player {
    pub fn new(name: &str, elo: u16) -> Self {
        Player {
            name: name.to_owned(),
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

#[derive(Serialize, Deserialize)]
pub struct GameInfo {
    name: String,
    elo_before: u16,
    elo_after: u16,
}

impl GameInfo {
    pub fn new(name: &str, elo_before: u16, elo_after: u16) -> Self {
        GameInfo {
            name: name.to_owned(),
            elo_before,
            elo_after,
        }
    }

    pub fn name(&self) -> &str {
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
