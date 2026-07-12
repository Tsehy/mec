use crate::cli::init::InitArgs;
use crate::models::game::Game;
use crate::models::player::Player;
use chrono::{Local, NaiveDate};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

#[derive(Debug, thiserror::Error)]
pub enum SeasonLoadError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Deserialize(#[from] serde_json::error::Error),
}

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
    
    pub fn load(name: &str) -> Result<Season, SeasonLoadError> {
        let file_path = format!("{}.json", name);
        let mut season_file = File::open(file_path)?;
        
        let mut json = String::new();
        season_file.read_to_string(&mut json)?;
        let season: Season = serde_json::from_str(&json)?;
        
        Ok(season)
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
