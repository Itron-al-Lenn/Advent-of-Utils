use crate::{error::AocError, input};
use std::{
    fmt::{Debug, Display},
    path::Path,
};

pub enum AocOption {
    Str(String),
    Int(i32),
    None,
}
impl Debug for AocOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AocOption::Str(s) => write!(f, "Str: {}", s),
            AocOption::Int(i) => write!(f, "Int: {}", i),
            AocOption::None => write!(f, "None"),
        }
    }
}

impl Display for AocOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AocOption::Str(s) => write!(f, "{}", s),
            AocOption::Int(i) => write!(f, "{}", i),
            AocOption::None => write!(f, "None"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Parts {
    Part1 = 1,
    Part2 = 2,
}

impl Display for Parts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Part1 => write!(f, "Part 1"),
            Self::Part2 => write!(f, "Part 2"),
        }
    }
}

impl TryFrom<u8> for Parts {
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Parts::new(value)
    }

    type Error = AocError;
}

impl Parts {
    pub fn new(num: u8) -> Result<Self, AocError> {
        match num {
            1 => Ok(Parts::Part1),
            2 => Ok(Parts::Part2),
            num => Err(AocError::InvalidPart(num)),
        }
    }
}

pub struct AocResult {
    day: u8,
    part: Parts,
    result: AocOption,
}

impl AocResult {
    pub fn new(day: u8, part: u8, result: AocOption) -> Self {
        Self {
            day,
            part: Parts::new(part).unwrap(),
            result,
        }
    }

    pub fn day(&self) -> u8 {
        self.day
    }

    pub fn part(&self) -> &Parts {
        &self.part
    }

    pub fn result(&self) -> &AocOption {
        &self.result
    }
}

pub struct PuzzleInput {
    pub input: String,
    pub test_result: AocOption,
}

impl PuzzleInput {
    pub fn new(year: i32, day: u8, input_dir: &String, test_mode: bool) -> Result<Self, AocError> {
        let input_path = match test_mode {
            false => format!("{}/{}/day_{}.txt", input_dir, year, day),
            true => format!("{}/tests/{}/day_{}.txt", input_dir, year, day),
        };

        match test_mode {
            false => match std::fs::read_to_string(&input_path) {
                Ok(input) => Ok(Self {
                    input,
                    test_result: AocOption::None,
                }),
                Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                    match input::fetch_input(year, day) {
                        Err(e) => Err(AocError::FetchingFailed(e.to_string())),
                        Ok(input) => {
                            if let Some(parent) = Path::new(&input_path).parent() {
                                if let Err(e) = std::fs::create_dir_all(parent) {
                                    return Err(AocError::FetchingFailed(e.to_string()));
                                };
                            }

                            if let Err(e) = std::fs::write(&input_path, &input) {
                                return Err(AocError::FetchingFailed(e.to_string()));
                            }
                            Ok(Self {
                                input,
                                test_result: AocOption::None,
                            })
                        }
                    }
                }
                Err(e) => Err(AocError::FetchingFailed(e.to_string())),
            },

            true => match std::fs::read_to_string(&input_path) {
                Ok(input) => {
                    let test_result = match input.lines().next() {
                        Some(result) => match result.parse::<i32>() {
                            Ok(int) => AocOption::Int(int),
                            Err(_) => AocOption::Str(result.to_string()),
                        },
                        None => {
                            return Err(AocError::FetchingFailed(
                                AocError::EmptyTest(year, day).to_string(),
                            ))
                        }
                    };
                    Ok(Self { test_result, input })
                }
                Err(e) => Err(AocError::FetchingFailed(e.to_string() + &input_path)),
            },
        }
    }

    pub fn input(&self) -> String {
        self.input.clone()
    }

    pub fn test_result(&self) -> String {
        self.test_result.to_string()
    }
}
