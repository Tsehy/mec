use crate::cli::AddGameArgs;
use crate::domain::{DomainError, Season};
use crate::history::event::EventAction;
use crate::history::game_created::GameCreated;
use crate::history::{History, HistoryError};
use chrono::{Local, NaiveDate};

#[derive(Debug, thiserror::Error)]
pub enum AddGameError {
    #[error(transparent)]
    SeasonLoad(#[from] DomainError),
    #[error(transparent)]
    DateTime(#[from] chrono::ParseError),
    #[error("Player `{0}` in not present in this season")]
    MissingPlayer(String),
    #[error("Player `{0}` appears more than one in the game")]
    DuplicatePlayer(String),
    #[error(transparent)]
    History(#[from] HistoryError),
}

pub fn run(args: &AddGameArgs) -> Result<(), AddGameError> {
    let season = Season::load(args.season())?;

    let date = match args.date() {
        Some(date) => NaiveDate::parse_from_str(date, "%Y-%m-%d")?,
        None => Local::now().date_naive(),
    };

    check_duplicates(args.players())?;
    check_player_presence(args.players(), &season)?;

    let event = GameCreated::new(date, args.players_arr());
    event.execute(season)?;

    let mut history = History::load(args.season())?;
    history.append(event)?;
    history.save_to_file()?;

    println!("Game registered");
    Ok(())
}

fn check_duplicates(players: &[String]) -> Result<(), AddGameError> {
    for current_player in players {
        let current_player_count = players
            .iter()
            .filter(|player| player == &current_player)
            .count();

        if current_player_count != 1 {
            return Err(AddGameError::DuplicatePlayer(current_player.clone()));
        }
    }
    Ok(())
}

fn check_player_presence(players: &[String], season: &Season) -> Result<(), AddGameError> {
    for player_name in players {
        if season
            .players()
            .iter()
            .find(|player| player.name() == player_name)
            .is_none()
        {
            return Err(AddGameError::MissingPlayer(player_name.to_string()));
        }
    }
    Ok(())
}
