use crate::{
    error::{AocError, InputError},
    input::fetch_input,
    time::AocTime,
};
use std::path::{Path, PathBuf};

pub struct PuzzleInput {
    pub input: String,
}

impl PuzzleInput {
    pub fn new(year: i32, day: u8, input_dir: &String, is_test: bool) -> Result<Self, AocError> {
        let source_path: PathBuf = match is_test {
            false => format!("{}/{}/day_{}.txt", input_dir, year, day).into(),
            true => format!("{}/tests/{}/day_{}.txt", input_dir, year, day).into(),
        };

        match is_test {
            false => match std::fs::read_to_string(&source_path) {
                Ok(input) => Ok(Self { input }),
                Err(err)
                    if err.kind() == std::io::ErrorKind::NotFound
                        && AocTime::now().is_puzzle_available(year, day) =>
                {
                    match fetch_input(year, day) {
                        Err(e) => Err(AocError::Input(e)),
                        Ok(input) => {
                            if let Some(parent) = Path::new(&source_path).parent() {
                                if let Err(e) = std::fs::create_dir_all(parent) {
                                    return Err(AocError::Input(InputError::FileSaveError {
                                        path: source_path,
                                        reason: "Failed creating dir.".to_string(),
                                        source: Some(e),
                                    }));
                                };
                            }

                            if let Err(e) = std::fs::write(&source_path, &input) {
                                return Err(AocError::Input(InputError::FileSaveError {
                                    path: source_path,
                                    reason: "Failed writing file.".to_string(),
                                    source: Some(e),
                                }));
                            }
                            Ok(Self { input })
                        }
                    }
                }
                Err(e) => Err(AocError::Input(InputError::FileReadError {
                    path: source_path,
                    reason: "Failed reading file.".to_string(),
                    source: Some(e),
                })),
            },

            true => match std::fs::read_to_string(&source_path) {
                Ok(input) => Ok(Self { input }),
                Err(e) => Err(AocError::Input(InputError::FileReadError {
                    path: source_path,
                    reason: "Failed reading file.".to_string(),
                    source: Some(e),
                })),
            },
        }
    }

    pub fn input(&self) -> String {
        self.input.clone()
    }
}
