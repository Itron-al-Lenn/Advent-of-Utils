use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LoadingError {
    #[error("No solution library found for year {year}")]
    LibraryNotFound {
        year: i32,
        search_path: PathBuf,
        #[source]
        source: Option<std::io::Error>,
    },

    #[error("Multiple solution libraries found: {}", .paths.iter().map(|p| p.display().to_string()).collect::<Vec<_>>().join(", "))]
    AmbiguousLibraries { paths: Vec<PathBuf> },

    #[error("Failed to load solution library: {reason}")]
    LibraryLoadFailed {
        reason: String,
        #[source]
        source: Option<libloading::Error>,
    },

    #[error("No solutions found in library for year {year}")]
    NoSolutions { year: i32 },

    #[error("Invalid solution library: {reason}")]
    InvalidLibrary { reason: String },
}
