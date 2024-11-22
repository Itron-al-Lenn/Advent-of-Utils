use std::{error::Error, fmt::Display};

use chrono::{Datelike, FixedOffset};

mod input;
mod parser;

#[derive(Debug)]
enum AocError {
    WrongDate,
}

pub enum Parts {
    Part1,
    Part2,
}

impl Display for AocError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AocError::WrongDate => write!(f, "Invalid date for AoC"),
        }
    }
}

impl Error for AocError {}

pub struct Puzzle<R> {
    day: Day,
    puzzle_input: input::PuzzleInput,
    part_1: Option<R>,
    part_2: Option<R>,
}

impl<R: Display> Display for Puzzle<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Day {} ({})", self.day.day(), self.day.year())?;
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

impl<R> Puzzle<R> {
    pub fn new(day: Day) -> Self {
        Self {
            puzzle_input: input::PuzzleInput::new(&day),
            part_1: Option::None,
            part_2: Option::None,
            day,
        }
    }

    pub fn solve<T>(
        &mut self,
        part: Parts,
        parser: fn(&input::PuzzleInput) -> T,
        solver: fn(T) -> R,
    ) {
        let parsed_input = parser(&self.puzzle_input);
        let solution = solver(parsed_input);
        match part {
            Parts::Part1 => self.part_1 = Some(solution),
            Parts::Part2 => self.part_2 = Some(solution),
        }
    }
}

pub struct Day {
    day: u8,
    year: i32,
}

impl Day {
    fn new(day: u8, year: i32) -> Result<Self, AocError> {
        if day <= Self::get_max_day(year) && year > 2014 && year <= Self::get_max_year() {
            Ok(Self { day, year })
        } else {
            Err(AocError::WrongDate)
        }
    }

    fn get_max_day(year: i32) -> u8 {
        let ets = Self::get_ets();
        match year {
            year if year == ets.year() && 12 == ets.month() => {
                let day = ets.day();
                day as u8
            }
            _ => 25,
        }
    }

    fn get_max_year() -> i32 {
        let ets = Self::get_ets();
        match ets.month() {
            12 => ets.year(),
            _ => ets.year() - 1,
        }
    }

    fn get_ets() -> chrono::DateTime<FixedOffset> {
        chrono::DateTime::from_naive_utc_and_offset(
            chrono::Local::now().naive_utc(),
            chrono::FixedOffset::east_opt(5 * 3600).unwrap(),
        )
    }

    pub fn day(&self) -> u8 {
        self.day
    }

    pub fn year(&self) -> i32 {
        self.year
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_day_successfull() {
        let result = Day::new(1, 2015).expect("Test failed");
        assert_eq!(result.day, 1);
        assert_eq!(result.year, 2015);
    }

    #[test]
    #[should_panic]
    fn new_day_failed() {
        let _wrong_day = Day::new(27, 2016).expect("This should panic: Wrong day");
    }
    #[test]
    #[should_panic]
    fn new_day_failed2() {
        let _wrong_year = Day::new(13, 2054).expect("This should panic: Wrong year");
    }
    #[test]
    fn test_puzzle_display() {
        let puzzle = Puzzle::<i32>::new(Day::new(1, 2015).unwrap());
        println!("{}", puzzle);
        // This would print:
        // Day 1 (2015)
        // Part 1: Not solved
        // Part 2: Not solved
    }
}
