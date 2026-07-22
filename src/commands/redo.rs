use crate::cli::HistoryArgs;
use crate::domain::Season;
use crate::history::{History, HistoryError};
use crate::history::event::EventAction;

pub fn run(args: &HistoryArgs) -> Result<(), HistoryError> {
    let season = Season::load(args.season())?;
    let mut history = History::load(args.season())?;
    let event = history.redo()?;
    event.execute(season)?;
    history.save_to_file()?;
    
    println!("Next command executed");
    Ok(())
}