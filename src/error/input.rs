use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum InputError {
    #[error("Failed to read input file: {path}")]
    FileNotFound {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to fetch input: {msg}")]
    FetchFailed {
        msg: String,
        #[source]
        source: Option<reqwest::Error>,
    },

    #[error("Input parsing failed on line {line:?}: {msg}")]
    ParseError { msg: String, line: Option<usize> },

    #[error("Missing session token for input fetching")]
    MissingToken,
}

impl InputError {
    pub fn parse_error(msg: impl Into<String>) -> Self {
        Self::ParseError {
            msg: msg.into(),
            line: None,
        }
    }

    pub fn with_line(self, line_number: usize) -> Self {
        match self {
            Self::ParseError { msg, .. } => Self::ParseError {
                msg,
                line: Some(line_number),
            },
            other => other,
        }
    }
}
