use crate::cli::HistoryArgs;
use crate::domain::Season;
use crate::history::event::EventAction;
use crate::history::{History, HistoryError};

pub fn run(args: &HistoryArgs) -> Result<(), HistoryError> {
    let season = Season::load(args.season())?;
    let mut history = History::load(args.season())?;
    let event = history.undo()?;
    event.undo(season)?;
    history.save_to_file()?;

    println!("Previous command undone");
    Ok(())
}
