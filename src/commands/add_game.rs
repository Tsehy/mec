use crate::cli::AddGameArgs;
use crate::domain::{DomainError, Game, GameInfo, Player, Season};
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
}

// TODO: event creation
pub fn run(args: &AddGameArgs) -> Result<(), AddGameError> {
    let mut season = Season::load(args.season())?;

    let date = match args.date() {
        Some(date) => NaiveDate::parse_from_str(date, "%Y-%m-%d")?,
        None => Local::now().date_naive(),
    };

    check_duplicates(args.players())?;
    let mut game_infos = generate_game_infos(&mut season, &args)?;

    calculate_new_elo(&mut game_infos);
    for info in game_infos.iter() {
        season
            .players_mut()
            .iter_mut()
            .find(|player| player.name() == info.name())
            .expect("player should be present")
            .set_elo(info.elo_after());
    }

    season.games_mut().push(Game::new(date, game_infos));

    season.save_to_file()?;

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

fn generate_game_infos(
    season: &mut Season,
    args: &AddGameArgs,
) -> Result<Vec<GameInfo>, AddGameError> {
    let mut game_infos: Vec<GameInfo> = Vec::new();

    for player_name in args.players() {
        let find_result = season
            .players()
            .iter()
            .find(|player| player.name() == player_name);

        match find_result {
            Some(player) => {
                game_infos.push(GameInfo::new(player.name(), player.elo(), player.elo()))
            }
            None => {
                return Err(AddGameError::MissingPlayer(player_name.clone()));
            }
        }
    }
    Ok(game_infos)
}

fn calculate_new_elo(game_infos: &mut [GameInfo]) {
    let len = game_infos.len();

    let indexes = (0..len - 1).flat_map(|i| (i + 1..len).map(move |j| (i, j)));
    for (winner_index, loser_index) in indexes {
        let delta = elo_change(
            game_infos[winner_index].elo_before(),
            game_infos[loser_index].elo_before(),
        );

        game_infos[winner_index].elo_after_add(delta);
        game_infos[loser_index].elo_after_sub(delta);
    }
}

fn elo_change(winner_elo: u16, loser_elo: u16) -> u16 {
    // delta = 1 / (1 + 10 ^ ((loser - winner) / 400)
    let diff = f32::from(loser_elo) - f32::from(winner_elo);
    let exponent = diff / 400f32;
    let ratio = 1f32 / (1f32 + 10f32.powf(exponent)); // E
    let delta = 20f32 * (1f32 - ratio); // K * (S - E); K = 20; S = 1:winner, 0:loser 
    delta.round() as u16
}
