use crate::domain::{DomainError, Game, GameInfo, Season};
use crate::history::event::{EventAction, EventParseError};
use chrono::NaiveDate;
use std::fmt::{Display, Formatter};

pub struct GameCreated {
    date: NaiveDate,
    players: [String; 4],
}

impl GameCreated {
    pub fn new(date: NaiveDate, players: [String; 4]) -> Self {
        GameCreated { date, players }
    }
}

impl std::str::FromStr for GameCreated {
    type Err = EventParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!("implement game created event parsing")
    }
}

impl Display for GameCreated {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            format!(
                "GameCreated {} {}",
                self.date.format("%Y-%m-%d"),
                self.players.join(" ")
            )
        )
    }
}

impl EventAction for GameCreated {
    fn execute(&self, mut season: Season) -> Result<(), DomainError> {
        let mut game_infos = generate_game_infos(&mut season, &self.players);

        calculate_new_elo(&mut game_infos);
        for info in game_infos.iter() {
            season
                .players_mut()
                .iter_mut()
                .find(|player| player.name() == info.name())
                .expect("player should be present")
                .set_elo(info.elo_after());
        }

        season.games_mut().push(Game::new(self.date, game_infos));
        season.save_to_file()?;
        Ok(())
    }

    fn undo(&self, mut season: Season) -> Result<(), DomainError> {
        let removed_game = season
            .games_mut()
            .pop()
            .expect("there should be at leas one game in the season");

        for game_info in removed_game.players() {
            let player = season
                .players_mut()
                .iter_mut()
                .find(|player| player.name() == game_info.name())
                .expect("player should be in the season");

            player.set_elo(game_info.elo_before());
        }
        Ok(())
    }
}

fn generate_game_infos(season: &mut Season, players: &[String]) -> Vec<GameInfo> {
    let mut game_infos: Vec<GameInfo> = Vec::new();

    for player_name in players {
        let player = season
            .players()
            .iter()
            .find(|player| player.name() == player_name)
            .expect("player should be in the season");

        game_infos.push(GameInfo::new(player.name(), player.elo(), player.elo()));
    }
    game_infos
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
