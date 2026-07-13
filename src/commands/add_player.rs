use crate::cli::add_player::AddPlayerArgs;
use crate::models::player::Player;
use crate::models::season::{Season, SeasonError};

#[derive(Debug, thiserror::Error)]
pub enum AddPlayerError {
    #[error(transparent)]
    Season(#[from] SeasonError),
    #[error("Player `{0}` already exists")]
    PlayerExists(String),
}

pub fn run(args: &AddPlayerArgs) -> Result<(), AddPlayerError> {
    let mut season = Season::load(args.season())?;

    let player_exists = season
        .players()
        .iter()
        .any(|player| player.name() == args.name());

    if player_exists {
        return Err(AddPlayerError::PlayerExists(args.name().to_string()));
    }

    let new_player = Player::new(args.name(), *season.start_elo());
    season.players_mut().push(new_player);
    season.save_to_file()?;

    println!("Player `{}` added to `{}`", args.name(), season.name());
    Ok(())
}
