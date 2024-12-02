use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LoadingError {
    #[error("No solutions found for year {year}")]
    NoSolutionsFound { year: i32 },

    #[error("Failed to load solution library: {path}")]
    LibraryLoadFailed {
        path: PathBuf,
        #[source]
        source: libloading::Error,
    },

    #[error("Invalid solution library: {msg}")]
    InvalidLibrary { msg: String, path: PathBuf },

    #[error("Solution for day {day} not found")]
    SolutionNotFound { day: u8 },
}
