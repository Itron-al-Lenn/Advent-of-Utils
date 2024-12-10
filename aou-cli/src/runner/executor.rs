use advent_of_utils::{
    error::{AocError, SolutionError},
    time::AocTime,
    types::{AocResult, AocYear, PuzzleInput},
    Parts,
};
use std::time::Instant;
use tokio::task::JoinSet;

use crate::{
    config::{self, Config},
    loader,
};

pub(crate) async fn run_solutions(
    config: &Config,
    solutions: &loader::Solutions,
) -> Result<AocYear, AocError> {
    let mut tasks: JoinSet<Result<AocResult, AocError>> = JoinSet::new();
    let year = config.year;

    match config.day {
        Some(day) => {
            let solver = solutions
                .get(day)
                .ok_or(AocError::Solution(SolutionError::NotImplemented))?;

            schedule_day_tasks(&mut tasks, solver, day, config);
        }
        None => {
            let time = AocTime::now();
            for (day, solver) in solutions
                .iter()
                .filter(|(day, _)| time.is_puzzle_available(year, *day))
            {
                schedule_day_tasks(&mut tasks, solver, day, config);
            }
        }
    }

    collect_results(tasks).await
}

fn schedule_day_tasks(
    tasks: &mut JoinSet<Result<AocResult, AocError>>,
    solver: &dyn advent_of_utils::Solution,
    day: u8,
    config: &Config,
) {
    let part = match config.part {
        Some(part) => part as u8,
        None => 0,
    };
    // Schedule part 1
    if part != 2 {
        schedule_part_task(tasks, 1, day, solver, config.clone());
    }
    // Schedule part 2
    if part != 1 {
        schedule_part_task(tasks, 2, day, solver, config.clone());
    }
}

fn schedule_part_task(
    tasks: &mut JoinSet<Result<AocResult, AocError>>,
    part: u8,
    day: u8,
    solver: &dyn advent_of_utils::Solution,
    config: Config,
) {
    let solver = solver.clone_box();
    tasks.spawn_blocking(move || {
        let mut time_start = Instant::now();

        let input = PuzzleInput::new(config.year, day, &config.input_dir, config.test_mode)?;

        if config.exclude_parse_time {
            time_start = Instant::now();
        }
        let result = if part == 1 {
            solver.part1(input)
        } else {
            solver.part2(input)
        };
        let time = time_start.elapsed();

        Ok(AocResult::new(day, part, result, time))
    });
}

async fn collect_results(
    mut tasks: JoinSet<Result<AocResult, AocError>>,
) -> Result<AocYear, AocError> {
    let mut results = Vec::new();

    while let Some(result) = tasks.join_next().await {
        match result {
            Ok(Ok(aoc_result)) => {
                results.push(aoc_result);
            }
            Ok(Err(e)) => return Err(e),
            Err(_) => return Err(AocError::Solution(SolutionError::NotImplemented)),
        }
    }

    results.sort_by_key(|r| (r.day(), r.part() as u8));

    Ok(AocYear::from_vec(results))
}
