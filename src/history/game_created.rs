use crate::domain::{Game, GameInfo, Season};
use crate::history::HistoryError;
use crate::history::event::{Event, EventAction, EventParseError};
use chrono::NaiveDate;
use std::fmt::{Display, Formatter};

pub struct GameCreated {
    date: NaiveDate,
    players: [String; 4],
}

impl GameCreated {
    pub fn new(date: NaiveDate, players: [String; 4]) -> Event {
        Event::GameCreated(GameCreated { date, players })
    }
}

impl std::str::FromStr for GameCreated {
    type Err = EventParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split(' ').collect();
        if parts.len() != 6 {
            Err(EventParseError::ArgumentCount(5, parts.len() as u32))
        } else {
            let date = NaiveDate::parse_from_str(parts[1], "%Y-%m-%d")?;
            let players: [String; 4] = std::array::from_fn(|i| parts[i + 2].to_owned());
            Ok(GameCreated { date, players })
        }
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
    fn execute(&self, mut season: Season) -> Result<(), HistoryError> {
        let mut game_infos = generate_game_infos(&mut season, &self.players);

        calculate_new_elo(&mut game_infos);
        for info in game_infos.iter() {
            match season
                .players_mut()
                .iter_mut()
                .find(|player| player.name() == info.name())
            {
                Some(player) => player.set_elo(info.elo_after()),
                None => {
                    return Err(HistoryError::CorruptedHistory(format!(
                        "Player `{}` is in the game, but is missing from the season",
                        info.name()
                    )));
                }
            }
        }

        season.games_mut().push(Game::new(self.date, game_infos));
        season.save_to_file()?;
        Ok(())
    }

    fn undo(&self, mut season: Season) -> Result<(), HistoryError> {
        let removed_game = match season.games_mut().pop() {
            Some(last_game) => last_game,
            None => {
                return Err(HistoryError::CorruptedHistory(
                    "The last event is a game, but the season is empty".to_owned(),
                ));
            }
        };

        for game_info in removed_game.players() {
            match season
                .players_mut()
                .iter_mut()
                .find(|player| player.name() == game_info.name())
            {
                Some(player) => player.set_elo(game_info.elo_before()),
                None => {
                    return Err(HistoryError::CorruptedHistory(format!(
                        "Player `{}` is in the game, but is missing from the season",
                        game_info.name()
                    )));
                }
            }
        }
        season.save_to_file()?;
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
