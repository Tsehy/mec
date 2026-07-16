use crate::domain::{Player, Season, DomainError};

pub fn parse(text: &str) -> Option<Box<dyn Event>> {
    todo!();
}

pub trait Event {
    fn execute(&self, season: Season) -> Result<(), DomainError>;
    fn undo(&self, season: Season) -> Result<(), DomainError>;
    fn to_string(&self) -> String;
}

pub struct AddPlayerEvent {
    name: String,
}

impl AddPlayerEvent {
    pub fn new(name: &str) -> Self {
        AddPlayerEvent {
            name: name.to_string(),
        }
    }
}

impl Event for AddPlayerEvent {
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

    fn to_string(&self) -> String {
        format!("AddPlayer {}", self.name)
    }
}