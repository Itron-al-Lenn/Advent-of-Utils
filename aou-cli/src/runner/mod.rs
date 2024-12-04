mod executor;

use advent_of_utils::{time::AocTime, types::display::Table};
use std::error::Error;

use crate::{config::Config, loader};

pub async fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    // Validate input parameters
    match config.day {
        Some(day) => AocTime::now().validate_date(config.year, day)?,
        None => AocTime::now().validate_year(config.year)?,
    }

    // Load solutions using the new loader API
    let solutions = loader::load_solutions(config).await?;

    // Execute solutions
    let execution_result = executor::run_solutions(config, &solutions).await?;

    // Display results with metrics
    execution_result.table();

    Ok(())
}
