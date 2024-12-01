use std::{error::Error, fmt::Display, path::PathBuf};

use crate::time::{get_max_day, get_max_year};

#[derive(Debug)]
pub enum AocError {
    InvalidDay(i32, u8),
    InvalidYear(i32),
    InvalidPart(u8),

    FetchingFailed(String),
    LoadingFailed(String),
    LibNotFound(i32, String),
    EmptyTest(i32, u8),

    NoSolutionsFound,
    AmbiguousSolutions(Vec<PathBuf>),
}

impl Display for AocError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidDay(year, day) => write!(
                f,
                "Invalid day. Expected a day between 1-{} got: {}",
                get_max_day(*year),
                day
            ),
            Self::InvalidYear(year) => write!(
                f,
                "Invalid year. Expected a year between 2015-{} got: {}",
                get_max_year(),
                year
            ),
            Self::InvalidPart(part) => write!(f, "Invalid part. Expected 1 or 2 got: {}", part),

            Self::FetchingFailed(error) => {
                write!(f, "Fetching of the input online failed: {}", error)
            }
            Self::EmptyTest(year, day) => write!(
                f,
                "Test file for that day is empty. Year: {}, Day: {}",
                year, day
            ),
            Self::LoadingFailed(error) => {
                write!(f, "Dynamically loading the solver failed: {}", error)
            }
            Self::LibNotFound(year, path) => {
                write!(
                    f,
                    "No dynamic libary containing {} in the file name found at {}",
                    year, path
                )
            }
            Self::NoSolutionsFound => write!(f, "No valid libary found."),
            Self::AmbiguousSolutions(paths) => {
                writeln!(f, "Multiple solution files were found:")?;
                for path in paths {
                    writeln!(f, "  - {}", path.display())?;
                }
                write!(
                    f,
                    "Please ensure only one solution file exists for this year"
                )
            }
        }
    }
}

impl Error for AocError {}
