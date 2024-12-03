use advent_of_utils::{
    error::{AocError, SolutionError},
    types::{AocResult, PuzzleInput},
    Parts,
};
use std::time::Instant;
use tokio::task::JoinSet;

use crate::{config::Config, loader};

pub struct ExecutionResult {
    pub results: Vec<(AocResult, ExecutionMetrics)>,
}

pub struct ExecutionMetrics {
    pub total_time: std::time::Duration,
    pub solution_time: std::time::Duration,
}

pub(crate) async fn run_solutions(
    config: &Config,
    solutions: &loader::Solutions,
) -> Result<ExecutionResult, AocError> {
    let mut tasks: JoinSet<Result<(AocResult, ExecutionMetrics), AocError>> = JoinSet::new();
    let year = config.year;
    let test_mode = config.test_mode;

    match config.day {
        Some(day) => {
            let solver = solutions
                .get(day)
                .ok_or(AocError::Solution(SolutionError::NotImplemented))?;

            schedule_day_tasks(
                &mut tasks,
                day,
                &config.part,
                solver,
                year,
                test_mode,
                config.input_dir.clone(),
            );
        }
        None => {
            for (day, solver) in solutions.iter() {
                schedule_day_tasks(
                    &mut tasks,
                    day,
                    &config.part,
                    solver,
                    year,
                    test_mode,
                    config.input_dir.clone(),
                );
            }
        }
    }

    collect_results(tasks).await
}

fn schedule_day_tasks(
    tasks: &mut JoinSet<Result<(AocResult, ExecutionMetrics), AocError>>,
    day: u8,
    part: &Option<Parts>,
    solver: &dyn advent_of_utils::Solution,
    year: i32,
    test_mode: bool,
    input_dir: String,
) {
    let part = match part {
        Some(part) => part.clone() as u8,
        None => 0,
    };
    // Schedule part 1
    if part != 2 {
        schedule_part_task(tasks, day, 1, solver, year, test_mode, input_dir.clone());
    }
    // Schedule part 2
    if part != 1 {
        schedule_part_task(tasks, day, 2, solver, year, test_mode, input_dir);
    }
}

fn schedule_part_task(
    tasks: &mut JoinSet<Result<(AocResult, ExecutionMetrics), AocError>>,
    day: u8,
    part: u8,
    solver: &dyn advent_of_utils::Solution,
    year: i32,
    test_mode: bool,
    input_dir: String,
) {
    let solver = solver.clone_box();

    tasks.spawn_blocking(move || {
        let total_start = Instant::now();

        let input = PuzzleInput::new(year, day, &input_dir, test_mode)?;

        let solution_start = Instant::now();
        let result = if part == 1 {
            solver.part1(input)
        } else {
            solver.part2(input)
        };
        let solution_time = solution_start.elapsed();
        let total_time = total_start.elapsed();

        Ok((
            AocResult::new(day, part, result),
            ExecutionMetrics {
                total_time,
                solution_time,
            },
        ))
    });
}

async fn collect_results(
    mut tasks: JoinSet<Result<(AocResult, ExecutionMetrics), AocError>>,
) -> Result<ExecutionResult, AocError> {
    let mut results = Vec::new();

    while let Some(result) = tasks.join_next().await {
        match result {
            Ok(Ok((aoc_result, metrics))) => {
                results.push((aoc_result, metrics));
            }
            Ok(Err(e)) => return Err(e),
            Err(_) => return Err(AocError::Solution(SolutionError::NotImplemented)),
        }
    }

    results.sort_by_key(|(r, _)| (r.day(), r.part() as u8));

    Ok(ExecutionResult { results })
}
