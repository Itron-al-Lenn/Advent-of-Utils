use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum InputError {
    #[error("Missing session token: {reason}")]
    MissingToken { reason: String },

    #[error("Failed to fetch input for year {year} day {day}: {reason}")]
    FetchFailed {
        year: i32,
        day: u8,
        reason: String,
        #[source]
        source: Option<reqwest::Error>,
    },

    #[error("Failed to read input file {path}: {reason}")]
    FileReadError {
        path: PathBuf,
        reason: String,
        #[source]
        source: Option<std::io::Error>,
    },

    #[error("Failed to save input to {path}: {reason}")]
    FileSaveError {
        path: PathBuf,
        reason: String,
        #[source]
        source: Option<std::io::Error>,
    },
}
