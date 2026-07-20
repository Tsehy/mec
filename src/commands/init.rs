use crate::cli::InitArgs;
use crate::domain::Season;
use crate::history::{History, HistoryError};
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Debug, thiserror::Error)]
pub enum InitError {
    #[error("Season already exists")]
    AlreadyExists,
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    History(#[from] HistoryError),
    #[error(transparent)]
    Serialize(#[from] serde_json::error::Error),
    #[error(transparent)]
    DateTime(#[from] chrono::format::ParseError),
}

pub fn run(args: &InitArgs) -> Result<(), InitError> {
    let file_name = format!("{}.json", args.name());
    if Path::new(&file_name).exists() {
        return Err(InitError::AlreadyExists);
    }

    let mut output_file = File::create_new(&file_name)?;

    let season = Season::try_from(args)?;
    let json = serde_json::to_string(&season)?;
    output_file.write_all(json.as_bytes())?;

    History::init(args.name())?;

    println!("Season created");
    Ok(())
}
