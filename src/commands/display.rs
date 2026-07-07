use crate::cli::display::DisplayArgs;
use crate::models::game_info::GameInfo;
use crate::models::season::Season;
use std::fs::File;
use std::io::Read;

#[derive(Debug, thiserror::Error)]
pub enum DisplayError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Deserialize(#[from] serde_json::error::Error),
    #[error("The season contains no games")]
    EmptySeason,
}

pub fn run(args: &DisplayArgs) -> Result<(), DisplayError> {
    let file_path = format!("{}.json", args.season());
    let mut season_file = File::open(file_path)?;

    let mut json = String::new();
    season_file.read_to_string(&mut json)?;
    let mut season: Season = serde_json::from_str(&json)?;

    if season.players().len() == 0 {
        return Err(DisplayError::EmptySeason);
    }

    println!("{} - {}\n", season.name(), season.date());
    print_summary(&mut season);
    print_table(&season, args.count());

    Ok(())
}

fn print_summary(season: &mut Season) {
    season.players_mut().sort_by(|a, b| b.elo().cmp(&a.elo()));

    let max_name_length = season
        .players()
        .iter()
        .map(|player| player.name().chars().count())
        .max()
        .unwrap_or(0);

    let games: Vec<&GameInfo> = season
        .games()
        .iter()
        .flat_map(|game| game.players())
        .collect();

    let mut position = 1u8;
    for player in season.players().iter() {
        let spacing = " ".repeat(max_name_length + 1 - player.name().chars().count());
        let game_count = games
            .iter()
            .filter(|info| info.name() == player.name())
            .count();

        println!(
            "{:2}. {}:{}{} ({})",
            position,
            player.name(),
            spacing,
            player.elo(),
            game_count,
        );
        position += 1;
    }
}

fn print_table(season: &Season, count: u8) {
    let game_count = season.games().len();
    let skip = if count != 0 {
        game_count.saturating_sub(count as usize)
    } else {
        0
    };

    println!();
    println!("    Date    |      1st     |      2nd     |      3rd     |      4th     ");
    println!("------------|--------------|--------------|--------------|--------------");
    for game in season.games().iter().skip(skip) {
        let first = &game.players()[0];
        let second = &game.players()[1];
        let third = &game.players()[2];
        let fourth = &game.players()[3];
        println!(
            " {:10} | {:12} | {:12} | {:12} | {:12} ",
            game.date(),
            first.name(),
            second.name(),
            third.name(),
            fourth.name()
        );

        println!(
            "            | {} -> {} | {} -> {} | {} -> {} | {} -> {} ",
            first.elo_before(),
            first.elo_after(),
            second.elo_before(),
            second.elo_after(),
            third.elo_before(),
            third.elo_after(),
            fourth.elo_before(),
            fourth.elo_after()
        );

        println!("------------|--------------|--------------|--------------|--------------");
    }
}
