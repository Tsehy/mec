use crate::cli::remove_player::RemovePlayerArgs;
use crate::models::player::Player;
use crate::models::season::{Season, SeasonError};

#[derive(Debug, thiserror::Error)]
pub enum RemovePlayerError {
    #[error(transparent)]
    Season(#[from] SeasonError),
    #[error("Player `{0}` is not present in the season")]
    MissingPlayer(String),
    #[error("Player `{0}` is referenced by {1} game(s)")]
    PlayerReference(String, u32),
}

pub fn run(args: &RemovePlayerArgs) -> Result<(), RemovePlayerError> {
    let mut season = Season::load(&args.season())?;

    let player_predicament = |player: &Player| player.name() == args.name();

    if !season.players().iter().any(player_predicament) {
        return Err(RemovePlayerError::MissingPlayer(args.name().to_string()));
    }

    if let Some(&count) = season.get_game_counts().get(args.name()) {
        return Err(RemovePlayerError::PlayerReference(
            args.name().to_string(),
            count,
        ));
    }

    let index = season
        .players()
        .iter()
        .position(player_predicament)
        .expect("player should be present in the season");

    season.players_mut().remove(index);

    season.save_to_file()?;

    println!(
        "Removed ˙{}˙ from `{}`",
        args.name(),
        args.season()
    );
    Ok(())
}
