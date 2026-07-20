use crate::cli::DisplayArgs;
use crate::domain::{DomainError, Season};

#[derive(Debug, thiserror::Error)]
pub enum DisplayError {
    #[error(transparent)]
    Season(#[from] DomainError),
    #[error("The season contains no players")]
    NoPlayers,
    #[error("The season contains no games")]
    NoGames,
}

pub fn run(args: &DisplayArgs) -> Result<(), DisplayError> {
    let mut season = Season::load(args.season())?;

    if season.players().len() == 0 {
        return Err(DisplayError::NoPlayers);
    }

    if season.games().len() == 0 {
        return Err(DisplayError::NoGames);
    }

    println!("{} - {}\n", season.name(), season.date());
    print_summary(&mut season);
    print_table(&season, args.count());

    Ok(())
}

fn print_summary(season: &mut Season) {
    season.sort_players();
    let max_name_length = season.get_max_name_length();
    let game_count = season.get_game_counts();

    let mut position = 1u8;
    for player in season.players().iter() {
        let spacing = " ".repeat(max_name_length + 1 - player.name().chars().count());

        println!(
            "{:2}. {}:{}{} ({})",
            position,
            player.name(),
            spacing,
            player.elo(),
            game_count.get(player.name()).unwrap_or(&0),
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
