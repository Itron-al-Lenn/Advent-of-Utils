
use chrono::{Datelike, FixedOffset};
use error::AocError;
use input::PuzzleInput;

mod error;
mod input;
mod parser;

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

pub enum ValidDate {
    Valid(PuzzleInput),
    Invalid,
}


pub struct Puzzle {
    day: u8,
    year: i32,
    puzzle_input: ValidDate,
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

}

impl Puzzle {
    pub fn new(day: u8, year: i32) -> Self {
        Self {
            day,
            year,
            puzzle_input: Self::fetch_input(day, year),
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

    fn fetch_input(day: u8, year: i32) -> ValidDate {
        match day {
            day if day <= Self::get_max_day(year) => ValidDate::Valid(PuzzleInput::new(day, year)),
            _ => ValidDate::Invalid,
        }
    }

    pub fn solve<T>(
        &mut self,
        part: Parts,
        parser: fn(&input::PuzzleInput) -> T,
        solver: fn(T) -> String,
    ) -> Result<(), AocError> {
        match self.puzzle_input {
            ValidDate::Invalid => self.puzzle_input = Self::fetch_input(self.day, self.year),
            ValidDate::Valid(_) => (),
        }
        match &self.puzzle_input {
            ValidDate::Invalid => Err(AocError::InvalidDate),
            ValidDate::Valid(input) => {
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

    }

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
