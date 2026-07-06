use crate::cli::add_game::AddGameArgs;
use crate::models::game::Game;
use crate::models::game_info::GameInfo;
use crate::models::player::Player;
use crate::models::season::Season;
use chrono::{Local, NaiveDate};
use std::fs::File;
use std::io::{Read, Write};

#[derive(Debug, thiserror::Error)]
pub enum AddGameError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Deserialize(#[from] serde_json::error::Error),
    #[error(transparent)]
    DateTime(#[from] chrono::ParseError),
    #[error("Player `{0}` in not present in this season")]
    MissingPlayers(String),
}

pub fn run(args: &AddGameArgs) -> Result<(), AddGameError> {
    let file_name = format!("{}.json", args.season());
    let mut season_file = File::open(&file_name)?;

    let mut json = String::new();
    season_file.read_to_string(&mut json)?;
    let mut season: Season = serde_json::from_str(&json)?;

    let mut game_infos = generate_game_infos(&mut season, &args)?;

    calculate_new_elo(&mut game_infos);
    for info in game_infos.iter() {
        let find_result = season
            .players_mut()
            .iter_mut()
            .find(|player| player.name() == info.name());
        if let Some(player) = find_result {
            player.set_elo(info.elo_after())
        }
    }

    let date = match args.date() {
        Some(date) => NaiveDate::parse_from_str(date, "%Y-%m-%d")?,
        None => Local::now().date_naive(),
    };
    season.games_mut().push(Game::new(date, game_infos));

    json = serde_json::to_string(&season)?;
    let mut season_file = File::create(&file_name)?;
    season_file.write_all(json.as_bytes())?;

    println!("Game registered successfully");
    Ok(())
}

fn generate_game_infos(season: &mut Season, args: &AddGameArgs) -> Result<Vec<GameInfo>, AddGameError> {
    let starting_elo = *season.start_elo();
    let mut game_infos: Vec<GameInfo> = Vec::new();
    for player_name in args.players() {
        let search_result = season
            .players()
            .iter()
            .find(|player| player.name() == player_name);
        if let Some(player) = search_result {
            game_infos.push(GameInfo::new(player.name(), player.elo(), player.elo()));
        } else {
            if *args.force() {
                season
                    .players_mut()
                    .push(Player::new(player_name, starting_elo));
                game_infos.push(GameInfo::new(player_name, starting_elo, starting_elo));
            } else {
                return Err(AddGameError::MissingPlayers(player_name.clone()));
            }
        }
    }
    Ok(game_infos)
}

fn calculate_new_elo(game_infos: &mut [GameInfo]) {
    for winner_index in 0..game_infos.len() - 1 {
        for loser_index in winner_index + 1..game_infos.len() {
            let delta = elo_change(
                game_infos[winner_index].elo_before(),
                game_infos[loser_index].elo_before(),
            );
            game_infos[winner_index].elo_after_add(delta);
            game_infos[loser_index].elo_after_sub(delta);
        }
    }
}

fn elo_change(winner_elo: u16, loser_elo: u16) -> u16 {
    // delta = 1 / (1 + 10 ^ ((loser - winner) / 400)
    let diff = f32::from(loser_elo) - f32::from(winner_elo);
    let exponent = diff / 400f32;
    let ratio = 1f32 / (1f32 + 10f32.powf(exponent));
    let delta = 20f32 * (1f32 - ratio);
    delta.round() as u16
}
