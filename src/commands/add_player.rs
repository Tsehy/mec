use crate::cli::AddPlayerArgs;
use crate::domain::{Season, DomainError};
use crate::history::event::EventAction;
use crate::history::player_created::PlayerCreated;

#[derive(Debug, thiserror::Error)]
pub enum AddPlayerError {
    #[error(transparent)]
    Season(#[from] DomainError),
    #[error("Player `{0}` already exists")]
    PlayerExists(String),
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
    
    // TODO:
    // "clear" redo stack
    // append to history

    println!("Player `{}` added to `{}`", args.name(), args.season());
    Ok(())
}
