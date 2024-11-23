use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub enum AocError {
    WrongYear,
    InvalidDate,
}

impl Display for AocError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AocError::WrongYear => write!(f, "Invalid year for AoC"),
            AocError::InvalidDate => write!(f, "Invalid date. It's too early"),
        }
    }
}

impl Error for AocError {}
