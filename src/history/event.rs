use crate::domain::{DomainError, Season};
use crate::history::{game_created::GameCreated, player_created::PlayerCreated};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, thiserror::Error)]
pub enum EventParseError {
    #[error("Unknown event in history: `{0}`")]
    UnknownEvent(String),
    #[error("The event has no identifier")]
    NoIdentifier,
    #[error("Expected number of arguments is `{0}` but found `{0}`")]
    ArgumentCount(u32, u32),
    #[error(transparent)]
    DateTime(#[from] chrono::ParseError),
}

pub trait EventAction: Display + FromStr {
    fn execute(&self, season: Season) -> Result<(), DomainError>;
    fn undo(&self, season: Season) -> Result<(), DomainError>;
}

pub enum Event {
    GameCreated(GameCreated),
    PlayerCreated(PlayerCreated),
}

impl Display for Event {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Event::GameCreated(event) => event.fmt(f),
            Event::PlayerCreated(event) => event.fmt(f),
        }
    }
}

impl FromStr for Event {
    type Err = EventParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().position(|c| c == ' ') {
            None => Err(EventParseError::NoIdentifier),
            Some(word_end) => match &s[..word_end] {
                "GameCreated" => Ok(Event::GameCreated(GameCreated::from_str(s)?)),
                "PlayerCreated" => Ok(Event::PlayerCreated(PlayerCreated::from_str(s)?)),
                _ => Err(EventParseError::UnknownEvent(s.to_owned())),
            },
        }
    }
}

impl EventAction for Event {
    fn execute(&self, season: Season) -> Result<(), DomainError> {
        match self {
            Event::GameCreated(event) => event.execute(season),
            Event::PlayerCreated(event) => event.execute(season),
        }
    }

    fn undo(&self, season: Season) -> Result<(), DomainError> {
        match self {
            Event::GameCreated(event) => event.undo(season),
            Event::PlayerCreated(event) => event.undo(season),
        }
    }
}
