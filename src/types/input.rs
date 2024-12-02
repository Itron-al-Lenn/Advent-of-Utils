use crate::{error::AocError, input::fetch_input};
use std::path::{Path, PathBuf};

pub struct PuzzleInput {
    pub input: String,
    pub metadata: InputMetadata,
}

#[derive(Debug)]
struct InputMetadata {
    year: i32,
    day: u8,
    is_test: bool,
    source_path: PathBuf,
}

impl PuzzleInput {
    pub fn new(year: i32, day: u8, input_dir: &String, is_test: bool) -> Result<Self, AocError> {
        let source_path: PathBuf = match is_test {
            false => format!("{}/{}/day_{}.txt", input_dir, year, day).into(),
            true => format!("{}/tests/{}/day_{}.txt", input_dir, year, day).into(),
        };

        match is_test {
            false => match std::fs::read_to_string(&source_path) {
                Ok(input) => Ok(Self {
                    input,
                    metadata: InputMetadata {
                        year,
                        day,
                        is_test,
                        source_path,
                    },
                }),
                Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                    match fetch_input(year, day) {
                        Err(e) => Err(AocError::FetchingFailed(e.to_string())),
                        Ok(input) => {
                            if let Some(parent) = Path::new(&source_path).parent() {
                                if let Err(e) = std::fs::create_dir_all(parent) {
                                    return Err(AocError::FetchingFailed(e.to_string()));
                                };
                            }

                            if let Err(e) = std::fs::write(&source_path, &input) {
                                return Err(AocError::FetchingFailed(e.to_string()));
                            }
                            Ok(Self {
                                input,
                                metadata: InputMetadata {
                                    year,
                                    day,
                                    is_test,
                                    source_path,
                                },
                            })
                        }
                    }
                }
                Err(e) => Err(AocError::FetchingFailed(e.to_string())),
            },

            true => match std::fs::read_to_string(&source_path) {
                Ok(input) => Ok(Self {
                    input,
                    metadata: InputMetadata {
                        year,
                        day,
                        is_test,
                        source_path,
                    },
                }),
                Err(e) => Err(AocError::FetchingFailed(e.to_string())),
            },
        }
    }

    pub fn input(&self) -> String {
        self.input.clone()
    }
}
