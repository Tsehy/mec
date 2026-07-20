use crate::history::event::Event;
use std::io::Write;

pub mod event;
pub mod game_created;
pub mod player_created;

#[derive(Debug, thiserror::Error)]
pub enum HistoryError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    IntParse(#[from] std::num::ParseIntError),
    #[error("No event to undo")]
    EmptyHistory,
    #[error("Cannot undo `Init` event")]
    UndoInit,
    #[error("No event to redo")]
    EmptyFuture,
    #[error("History is corrupted: {0}")]
    CorruptedHistory(String),
}

pub struct History {
    events: Vec<String>,
    state: usize,
    season: String,
}

impl History {
    pub fn init(season_name: &str) -> Result<(), HistoryError> {
        std::fs::create_dir_all(format!(".mec/{season_name}"))?;

        std::fs::File::create_new(format!(".mec/{season_name}/history"))?
            .write_all("Init".as_bytes())?;

        std::fs::File::create_new(format!(".mec/{season_name}/state"))?
            .write_all("0".as_bytes())?;

        Ok(())
    }

    pub fn load(season_name: &str) -> Result<Self, HistoryError> {
        let state: usize = std::fs::read_to_string(format!(".mec/{season_name}/state"))?.parse()?;
        let events: Vec<String> = std::fs::read_to_string(format!(".mec/{season_name}/history"))?
            .lines()
            .map(|line| line.to_string())
            .collect();

        Ok(History {
            events,
            state,
            season: season_name.to_string(),
        })
    }

    pub fn append(&mut self, event: Event) -> Result<(), HistoryError> {
        if self.events.len() <= self.state {
            return Err(HistoryError::CorruptedHistory(
                "State pointer is outside of history".to_string(),
            ));
        }
        self.events = self.events[..=self.state].to_vec();

        self.events.push(event.to_string());
        self.state += 1;

        Ok(())
    }

    pub fn undo(&mut self) -> Result<Event, HistoryError> {
        todo!("implement undo action")
        // get event at state pointer
        // decrease state pointer
        // return event
    }

    pub fn redo(&mut self) -> Result<Event, HistoryError> {
        todo!("implement redo action")
        // increase state pointer
        // get event at state pointer
        // return event
    }

    pub fn save_to_file(self) -> Result<(), HistoryError> {
        std::fs::write(
            format!(".mec/{}/history", self.season),
            self.events.join("\n"),
        )?;

        std::fs::write(
            format!(".mec/{}/state", self.season),
            self.state.to_string(),
        )?;

        Ok(())
    }
}
