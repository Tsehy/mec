use crate::cli::AddPlayerArgs;
use crate::domain::{DomainError, Season};
use crate::history::event::EventAction;
use crate::history::player_created::PlayerCreated;
use crate::history::{History, HistoryError};

#[derive(Debug, thiserror::Error)]
pub enum AddPlayerError {
    #[error(transparent)]
    Season(#[from] DomainError),
    #[error("Player `{0}` already exists")]
    PlayerExists(String),
    #[error(transparent)]
    History(#[from] HistoryError),
}

pub fn run(args: &AddPlayerArgs) -> Result<(), AddPlayerError> {
    let season = Season::load(args.season())?;

    let player_exists = season
        .players()
        .iter()
        .any(|player| player.name() == args.name());

    if player_exists {
        return Err(AddPlayerError::PlayerExists(args.name().to_string()));
    }

    let event = PlayerCreated::new(args.name());
    event.execute(season)?;

    let mut history = History::load(args.season())?;
    history.append(event)?;
    history.save_to_file()?;

    println!("Player `{}` added to `{}`", args.name(), args.season());
    Ok(())
}
