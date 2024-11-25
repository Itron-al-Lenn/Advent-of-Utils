use std::time::Duration;

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
    fn table_with_width(&self, part1_widths: &Widths, part2_widths: &Widths);
}

struct Solution {
    total_time: Option<Duration>,
    parse_time: Option<Duration>,
    solution_time: Option<Duration>,
    solution: Option<i32>,
}

impl Solution {
    pub fn new() -> Self {
        Self {
            total_time: None,
            parse_time: None,
            solution_time: None,
            solution: None,
        }
    }
}

pub struct Widths {
    total_time: usize,
    parse_time: usize,
    solution_time: usize,
    solution: usize,
}

impl Widths {
    fn new(solution: &Solution) -> Self {
        let total_time = solution
            .total_time
            .map(|t| t.as_micros().to_string().len() + 3)
            .unwrap_or(0)
            .max(5);
        let parse_time = solution
            .parse_time
            .map(|t| t.as_micros().to_string().len() + 3)
            .unwrap_or(0)
            .max(5);
        let solution_time = solution
            .solution_time
            .map(|t| t.as_micros().to_string().len() + 3)
            .unwrap_or(0)
            .max(6);

        let solution = solution
            .solution
            .map(|s| s.to_string().len())
            .unwrap_or(0)
            .max(6);

        Widths {
            total_time,
            parse_time,
            solution_time,
            solution,
        }
    }
    fn get_max(widths: Vec<Self>) -> Self {
        Self {
            total_time: widths
                .iter()
                .map(|widths| widths.total_time)
                .max()
                .unwrap_or(0),
            solution_time: widths
                .iter()
                .map(|widths| widths.solution_time)
                .max()
                .unwrap_or(0),
            parse_time: widths
                .iter()
                .map(|widths| widths.parse_time)
                .max()
                .unwrap_or(0),
            solution: widths
                .iter()
                .map(|widths| widths.solution)
                .max()
                .unwrap_or(0),
        }
    }
}

pub struct Puzzle {
    day: u8,
    year: i32,
    puzzle_input: ValidInput,
    part1: Solution,
    part2: Solution,
}

impl std::fmt::Display for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Day {} ({})", self.day, self.year)?;
        match &self.part1.solution {
            Some(solution) => writeln!(f, "Part 1: {}", solution)?,
            None => writeln!(f, "Part 1: Not solved")?,
        }
        match &self.part2.solution {
            Some(solution) => writeln!(f, "Part 2: {}", solution)?,
            None => writeln!(f, "Part 2: Not solved")?,
        }
        Ok(())
    }
}

impl Table for Puzzle {
    fn table(&self) {
        let part1_widths = Widths::new(&self.part1);
        let part2_widths = Widths::new(&self.part2);

        self.table_with_width(&part1_widths, &part2_widths)
    }

    fn table_with_width(&self, part1_width: &Widths, part2_width: &Widths) {
        let part1_solution = self
            .part1
            .solution
            .map(|s| s.to_string())
            .unwrap_or(String::from("None"));

        let part2_solution = self
            .part2
            .solution
            .map(|s| s.to_string())
            .unwrap_or(String::from("None"));

        let part1_time = self
            .part1
            .total_time
            .map(|t| t.as_micros().to_string() + " μs")
            .unwrap_or(String::from("None"));

        let part1_solution_time = self
            .part1
            .solution_time
            .map(|t| t.as_micros().to_string() + " μs")
            .unwrap_or(String::from("None"));

        let part2_time = self
            .part2
            .total_time
            .map(|t| t.as_micros().to_string() + " μs")
            .unwrap_or(String::from("None"));

        let part2_solution_time = self
            .part2
            .solution_time
            .map(|t| t.as_micros().to_string() + " μs")
            .unwrap_or(String::from("None"));

        println!(
            "┃ {0:>3} │ {1:^2$} │ {3:^4$} │ {5:^6$} │ {7:^6$} │ {8:^9$} │ {10:^9$} ┃",
            self.day,
            part1_solution,
            part1_width.solution,
            part2_solution,
            part2_width.solution,
            part1_time,
            part1_width.total_time.max(part2_width.total_time),
            part2_time,
            part1_solution_time,
            part1_width.solution_time.max(part2_width.solution_time),
            part2_solution_time,
        );
    }
}

impl Puzzle {
    pub fn new(day: u8, year: i32) -> Self {
        Self {
            day,
            year,
            puzzle_input: ValidInput::Invalid,
            part1: Solution::new(),
            part2: Solution::new(),
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
        solver: fn(T) -> i32,
    ) -> Result<(), AocError> {
        match self.puzzle_input {
            ValidInput::Invalid => self.puzzle_input = Self::fetch_input(self.day, self.year),
            ValidInput::Valid(_) => (),
        }
        match &self.puzzle_input {
            ValidInput::Invalid => Err(AocError::InvalidDate),
            ValidInput::Valid(input) => {
                let start = std::time::Instant::now();
                let parsed_input = parser(input);
                let parse_time = start.elapsed();
                let start_solution = std::time::Instant::now();
                let solution = solver(parsed_input);
                let solution_time = start_solution.elapsed();
                let total_time = start.elapsed();
                match part {
                    Parts::Part1 => {
                        self.part1.solution = Some(solution);
                        self.part1.total_time = Some(total_time);
                        self.part1.solution_time = Some(solution_time);
                        self.part1.parse_time = Some(parse_time);
                    }
                    Parts::Part2 => {
                        self.part2.solution = Some(solution);
                        self.part2.total_time = Some(total_time);
                        self.part2.solution_time = Some(solution_time);
                        self.part2.parse_time = Some(parse_time);
                    }
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
        let part1_widths = self
            .puzzles
            .iter()
            .map(|puzzle| -> Widths { Widths::new(&puzzle.part1) })
            .collect();

        let part2_widths = self
            .puzzles
            .iter()
            .map(|puzzle| -> Widths { Widths::new(&puzzle.part2) })
            .collect();

        self.table_with_width(
            &Widths::get_max(part1_widths),
            &Widths::get_max(part2_widths),
        )
    }

    fn table_with_width(&self, part1_width: &Widths, part2_width: &Widths) {
        println!("┏━━━━━━┓");
        println!("┃ {} ┃", self.year);
        println!(
            "┣━━━━━┳┻{0:━>1$}━┳━{0:━>2$}━┳━{0:━>3$}━━━{0:━>3$}━┳━{0:━>4$}━━━{0:━>4$}━┓",
            "",
            part1_width.solution,
            part2_width.solution,
            part1_width.total_time.max(part2_width.total_time),
            part1_width.solution_time.max(part2_width.solution_time),
        );
        println!(
            "┃ Day ┃ {0:^1$} ┃ {2:^3$} ┃ {4:^5$} ┃ {6:^7$} ┃",
            "Part 1",
            part1_width.solution,
            "Part 2",
            part2_width.solution,
            "Total Time",
            2 * part1_width.total_time.max(part2_width.total_time) + 3,
            "Solution Time",
            2 * part1_width.solution_time.max(part2_width.solution_time) + 3,
        );
        println!(
            "┣━━━━━╇━{0:━>1$}━╇━{0:━>2$}━╇━{0:━>3$}━┯━{0:━>3$}━╇━{0:━>4$}━┯━{0:━>4$}━┫",
            "",
            part1_width.solution,
            part2_width.solution,
            part1_width.total_time.max(part2_width.total_time),
            part1_width.solution_time.max(part2_width.solution_time),
        );
        for puzzle in self.puzzles.iter() {
            puzzle.table_with_width(part1_width, part2_width);
        }
        println!(
            "┗━━━━━┷━{0:━>1$}━┷━{0:━>2$}━┷━{0:━>3$}━┷━{0:━>3$}━┷━{0:━>4$}━┷━{0:━>4$}━┛",
            "",
            part1_width.solution,
            part2_width.solution,
            part1_width.total_time.max(part2_width.total_time),
            part1_width.solution_time.max(part2_width.solution_time),
        );
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
mod tests {
    use super::*;
    use chrono::FixedOffset;

    #[test]
    fn test_get_ets() {
        let ets = get_ets();
        let fixed_offset = FixedOffset::west_opt(5 * 3600).unwrap();
        assert_eq!(ets.offset(), &fixed_offset);
    }

    #[test]
    fn test_solution_new() {
        let solution = Solution::new();
        assert!(solution.total_time.is_none());
        assert!(solution.parse_time.is_none());
        assert!(solution.solution_time.is_none());
        assert!(solution.solution.is_none());
    }

    #[test]
    fn test_widths_new() {
        let solution = Solution::new();
        let widths = Widths::new(&solution);
        assert_eq!(widths.total_time, 5);
        assert_eq!(widths.parse_time, 5);
        assert_eq!(widths.solution_time, 6);
        assert_eq!(widths.solution, 6);
    }

    #[test]
    fn test_fetch_input_valid() {
        let day = 1;
        let year = 2023;
        let input = Puzzle::fetch_input(day, year);
        if let ValidInput::Valid(_) = input {
        } else {
            panic!("Expected ValidInput::Valid");
        }
    }

    #[test]
    fn test_fetch_input_invalid() {
        let day = 26;
        let year = 2024;
        let input = Puzzle::fetch_input(day, year);
        if let ValidInput::Invalid = input {
        } else {
            panic!("Expected ValidInput::Invalid");
        }
    }

    #[test]
    fn test_puzzle_new() {
        let puzzle = Puzzle::new(1, 2024);
        assert_eq!(puzzle.day, 1);
        assert_eq!(puzzle.year, 2024);
        assert!(matches!(puzzle.puzzle_input, ValidInput::Invalid));
        assert!(puzzle.part1.solution.is_none());
        assert!(puzzle.part2.solution.is_none());
    }

    #[test]
    fn test_year_new() {
        let year = Year::new(2023).unwrap();
        assert_eq!(year.year, 2023);
        assert_eq!(year.puzzles.len(), 25);
    }

    #[test]
    fn test_year_new_invalid_year() {
        let year = Year::new(2014);
        assert!(year.is_err());
    }
}
