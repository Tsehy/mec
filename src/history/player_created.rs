use crate::domain::{DomainError, Player, Season};
use crate::history::event::{Event, EventAction, EventParseError};
use std::fmt::Display;

pub struct PlayerCreated {
    name: String,
}

impl PlayerCreated {
    pub fn new(name: &str) -> Event {
        Event::PlayerCreated(PlayerCreated {
            name: name.to_owned(),
        })
    }
}

impl std::str::FromStr for PlayerCreated {
    type Err = EventParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split(' ').collect();
        if parts.len() != 2 {
            Err(EventParseError::ArgumentCount(2, parts.len() as u32))
        } else {
            Ok(PlayerCreated {
                name: parts[1].to_owned(),
            })
        }
    }
}

impl Display for PlayerCreated {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("PlayerCreated {}", self.name))
    }
}

impl EventAction for PlayerCreated {
    fn execute(&self, mut season: Season) -> Result<(), DomainError> {
        let new_player = Player::new(&self.name, *season.start_elo());
        season.players_mut().push(new_player);
        season.save_to_file()?;
        Ok(())
    }

    fn undo(&self, mut season: Season) -> Result<(), DomainError> {
        let index = season
            .players()
            .iter()
            .position(|player| player.name() == self.name)
            .expect("player should be present in the season");

        season.players_mut().remove(index);
        season.save_to_file()?;

        Ok(())
    }
}
