use std::{error::Error, sync::Arc};

use advent_of_utils::{
    error::AocError,
    time::{validate_date, validate_year},
    types::AocResult,
    Parts, PuzzleInput,
};
use tokio::{
    fs::{read_dir, DirEntry},
    task::JoinSet,
};

use crate::loader::load_year;

pub struct Config {
    pub year: i32,
    pub day: Option<u8>,
    pub part: Option<Parts>,
    pub test_mode: bool,
    pub input_dir: String,
    pub workspace_dir: String,
}

impl Config {
    pub async fn loader_paths(&self) -> Result<Vec<DirEntry>, Box<dyn Error>> {
        let mut matching_files = Vec::new();

        let mut dir = read_dir(&self.workspace_dir).await?;
        while let Some(entry) = dir.next_entry().await? {
            if let Some(file_name) = entry.file_name().to_str() {
                if file_name.contains(&self.year.to_string()) {
                    matching_files.push(entry);
                }
            }
        }

        Ok(matching_files)
    }
}

pub async fn run<'a>(config: &Config) -> Result<(), Box<dyn Error>> {
    match config.day {
        Some(day) => validate_date(config.year, day)?,
        None => validate_year(config.year)?,
    }

    let solvers = load_year(config).await?;

    let mut tasks: JoinSet<Result<AocResult, AocError>> = JoinSet::new();

    let year = config.year;
    let test_mode = config.test_mode;

    if let Some(day) = config.day {
        let solver = match solvers.get(&day) {
            Some(solution) => solution.clone_box(),
            None => return Err(Box::new(AocError::NoSolutionsFound)),
        };
        let solver2 = solver.clone_box();

        let input_dir = config.input_dir.clone();
        tasks.spawn_blocking(move || {
            let input = PuzzleInput::new(year, day, &input_dir, test_mode);
            match input {
                Err(error) => Err(error),
                Ok(puzzle_input) => Ok(AocResult::new(day, 1, solver.part1(puzzle_input))),
            }
        });

        let input_dir = config.input_dir.clone();
        tasks.spawn_blocking(move || {
            let input = PuzzleInput::new(year, day, &input_dir, test_mode);
            match input {
                Err(error) => Err(error),
                Ok(puzzle_input) => Ok(AocResult::new(day, 2, solver2.part2(puzzle_input))),
            }
        });
    } else {
        let solver_pairs: Vec<_> = solvers.iter().collect();

        for (&day, solver) in solver_pairs {
            let solver = Arc::new(solver.clone_box());
            let solver2 = Arc::clone(&solver);
            let input_dir = config.input_dir.clone();
            tasks.spawn_blocking(move || {
                let input = PuzzleInput::new(year, day, &input_dir, test_mode);
                match input {
                    Err(error) => Err(error),
                    Ok(puzzle_input) => Ok(AocResult::new(day, 1, solver.part1(puzzle_input))),
                }
            });

            let input_dir = config.input_dir.clone();
            tasks.spawn_blocking(move || {
                let input = PuzzleInput::new(year, day, &input_dir, test_mode);
                match input {
                    Err(error) => Err(error),
                    Ok(puzzle_input) => Ok(AocResult::new(day, 2, solver2.part2(puzzle_input))),
                }
            });
        }
    }

    let mut results: Vec<AocResult> = Vec::new();
    while let Some(result) = tasks.join_next().await {
        results.push(result??);
    }

    results.sort_by_key(|r| (r.day(), r.part().clone() as u8));
    for result in results {
        println!(
            "Day: {}, Part: {}, Result: {}",
            result.day(),
            result.part(),
            result.result()
        );
    }

    Ok(())
}
