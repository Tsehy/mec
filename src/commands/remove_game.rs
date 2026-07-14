use crate::cli::remove_game::RemoveGameArgs;
use crate::models::season::{Season, SeasonError};
use std::collections::HashMap;

#[derive(Debug, thiserror::Error)]
pub enum RemoveGameError {
    #[error(transparent)]
    Season(#[from] SeasonError),
    #[error("There are only `{0}` games in the season")]
    NotEnoughGames(usize),
}

pub fn run(args: &RemoveGameArgs) -> Result<(), RemoveGameError> {
    let mut season = Season::load(args.season())?;

    if args.count() == 0 || args.count() as usize > season.games().len() {
        return Err(RemoveGameError::NotEnoughGames(season.games().len()));
    }

    let mut player_scores: HashMap<String, u16> = HashMap::new();
    for _ in 0..args.count() {
        if let Some(removed_game) = season.games_mut().pop() {
            for game_info in removed_game.players() {
                player_scores.insert(game_info.name().to_string(), game_info.elo_before());
            }
        }
    }

    for player in season.players_mut() {
        if let Some(elo) = player_scores.get(player.name()) {
            player.set_elo(*elo);
        }
    }

    season.save_to_file()?;

    println!(
        "Last `{}` games removed from `{}`",
        args.count(),
        args.season()
    );
    Ok(())
}
