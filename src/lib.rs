use chrono::{Datelike, FixedOffset};
use error::AocError;
use input::PuzzleInput;

mod error;
mod input;
pub mod parser;

fn get_ets() -> chrono::DateTime<FixedOffset> {
    chrono::DateTime::from_naive_utc_and_offset(
        chrono::Local::now().naive_utc(),
        FixedOffset::west_opt(5 * 3600).unwrap(),
    )
}

pub enum Parts {
    Part1,
    Part2,
}

pub enum ValidInput {
    Valid(PuzzleInput),
    Invalid,
}

pub trait Table {
    fn table(&self);
    fn table_with_width(&self, part1_width: usize, part2_width: usize);
}

pub struct Puzzle {
    day: u8,
    year: i32,
    puzzle_input: ValidInput,
    part_1: Option<String>,
    part_2: Option<String>,
}

impl std::fmt::Display for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Day {} ({})", self.day, self.year)?;
        match &self.part_1 {
            Some(solution) => writeln!(f, "Part 1: {}", solution)?,
            None => writeln!(f, "Part 1: Not solved")?,
        }
        match &self.part_2 {
            Some(solution) => writeln!(f, "Part 2: {}", solution)?,
            None => writeln!(f, "Part 2: Not solved")?,
        }
        Ok(())
    }
}

impl Table for Puzzle {
    fn table(&self) {
        let part1_width = match self.part_1.as_ref() {
            Some(solution) => solution.chars().count(),
            None => 4,
        };

        let part2_width = match self.part_2.as_ref() {
            Some(solution) => solution.chars().count(),
            None => 4,
        };
        self.table_with_width(part1_width, part2_width)
    }

    fn table_with_width(&self, part1_width: usize, part2_width: usize) {
        let part1 = match self.part_1.as_ref() {
            Some(solution) => solution,
            None => &String::from("None"),
        };

        let part2 = match self.part_2.as_ref() {
            Some(solution) => solution,
            None => &String::from("None"),
        };

        println!(
            "┃ {:^2} │ {:^part1_width$} │ {:^part2_width$} ┃",
            self.day, part1, part2
        )
    }
}

impl Puzzle {
    pub fn new(day: u8, year: i32) -> Self {
        Self {
            day,
            year,
            puzzle_input: ValidInput::Invalid,
            part_1: Option::None,
            part_2: Option::None,
        }
    }

    fn get_max_day(year: i32) -> u8 {
        let ets = get_ets();
        match year {
            year if year == ets.year() && 12 == ets.month() => {
                let day = ets.day();
                day as u8
            }
            _ => 25,
        }
    }

    fn fetch_input(day: u8, year: i32) -> ValidInput {
        match day {
            day if day <= Self::get_max_day(year) => ValidInput::Valid(PuzzleInput::new(day, year)),
            _ => ValidInput::Invalid,
        }
    }

    pub fn solve<T>(
        &mut self,
        part: Parts,
        parser: fn(&input::PuzzleInput) -> T,
        solver: fn(T) -> String,
    ) -> Result<(), AocError> {
        match self.puzzle_input {
            ValidInput::Invalid => self.puzzle_input = Self::fetch_input(self.day, self.year),
            ValidInput::Valid(_) => (),
        }
        match &self.puzzle_input {
            ValidInput::Invalid => Err(AocError::InvalidDate),
            ValidInput::Valid(input) => {
                let parsed_input = parser(input);
                let solution = solver(parsed_input);
                match part {
                    Parts::Part1 => self.part_1 = Some(solution),
                    Parts::Part2 => self.part_2 = Some(solution),
                }
                Ok(())
            }
        }
    }
}

pub struct Year {
    year: i32,
    pub puzzles: [Puzzle; 25],
}

impl Table for Year {
    fn table(&self) {
        let part1_width = self
            .puzzles
            .iter()
            .map(|puzzle| puzzle.part_1.as_deref().unwrap_or("None").chars().count())
            .max()
            .unwrap_or(4);

        let part2_width = self
            .puzzles
            .iter()
            .map(|puzzle| puzzle.part_1.as_deref().unwrap_or("None").chars().count())
            .max()
            .unwrap_or(4);

        self.table_with_width(part1_width, part2_width)
    }

    fn table_with_width(&self, part1_width: usize, part2_width: usize) {
        println!("┏━━━━━━┓");
        println!("┃ {} ┃", self.year);
        println!("┣━━━━━┯┺{0:━>part1_width$}━┯━{0:━>part2_width$}━┓", "");
        for puzzle in self.puzzles.iter() {
            puzzle.table_with_width(part1_width, part2_width);
        }
        println!("┗━━━━━┷━{0:━>part1_width$}━┷━{0:━>part2_width$}━┛", "");
    }
}

impl Year {
    pub fn new(year: i32) -> std::result::Result<Year, error::AocError> {
        if year < 2015 || year > Self::get_max_year() {
            return Err(error::AocError::WrongYear);
        }
        let puzzles = std::array::from_fn(|i| -> Puzzle { Puzzle::new((i + 1) as u8, year) });
        Ok(Self { year, puzzles })
    }

    fn get_max_year() -> i32 {
        let ets = get_ets();
        match ets.month() {
            12 => ets.year(),
            _ => ets.year() - 1,
        }
    }
}

#[cfg(test)]
mod tests {}
