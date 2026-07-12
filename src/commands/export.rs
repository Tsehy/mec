use crate::cli::export::ExportArgs;
use crate::models::embed::{Body, EmbedError};
use crate::models::season::{Season, SeasonLoadError};
use chrono::Local;
use std::fmt::Debug;

#[derive(Debug, thiserror::Error)]
pub enum ExportError {
    #[error(transparent)]
    Load(#[from] SeasonLoadError),
    #[error(transparent)]
    Serde(#[from] serde_json::error::Error),
    #[error("The season contains no players")]
    NoPlayers,
    #[error("The season contains no games")]
    NoGames,
    #[error(transparent)]
    Embed(#[from] EmbedError),
}

pub fn run(args: &ExportArgs) -> Result<(), ExportError> {
    let mut season = load_and_validate_season(args.season())?;

    let body_title = format!("{} - {}", Local::now().date_naive(), season.name());
    let mut embed_body = Body::new(body_title, args.color())?;

    if args.count() > 0 {
        add_game_fields(
            &mut embed_body,
            &season,
            args.count() as usize,
            args.inline(),
        );
    }

    add_season_summary(&mut embed_body, &mut season);

    let json = serde_json::to_string(&embed_body)?;
    println!("{json}");

    Ok(())
}

fn load_and_validate_season(season: &str) -> Result<Season, ExportError> {
    let season = Season::load(season)?;

    if season.players().len() == 0 {
        return Err(ExportError::NoPlayers);
    }

    if season.games().len() == 0 {
        return Err(ExportError::NoGames);
    }

    Ok(season)
}

fn add_game_fields(embed_body: &mut Body, season: &Season, count: usize, inline: bool) {
    let skip = season.games().len().saturating_sub(count);
    let mut game_index = 1;
    for game in season.games().iter().skip(skip).take(count) {
        let field_title = format!("__Game {} - {}__", game_index, game.date());
        game_index += 1;

        let field_value = game
            .players()
            .iter()
            .map(|player| {
                format!(
                    "1. {} ({} -> {})",
                    player.name(),
                    player.elo_before(),
                    player.elo_after()
                )
            })
            .collect::<Vec<String>>()
            .join("\n");

        embed_body.add_field(field_title, field_value, inline);
    }
}

fn add_season_summary(embed_body: &mut Body, season: &mut Season) {
    season.sort_players();
    let game_count = season.get_game_counts();

    let field_value = season
        .players()
        .iter()
        .map(|player| {
            format!(
                "1. {}: {} ({})",
                player.name(),
                player.elo(),
                game_count.get(player.name()).unwrap_or(&0)
            )
        })
        .collect::<Vec<String>>()
        .join("\n");

    embed_body.add_field("__Leaderboard__".to_string(), field_value, false);
}
