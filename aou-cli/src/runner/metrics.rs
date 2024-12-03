use super::ExecutionResult;
use std::time::Duration;

pub struct Metrics {
    pub total_execution_time: Duration,
    pub total_solution_time: Duration,
    pub average_solution_time: Duration,
}

impl Metrics {
    fn new(execution_result: &ExecutionResult) -> Self {
        let total_execution_time: Duration = execution_result
            .results
            .iter()
            .map(|(_, metrics)| metrics.total_time)
            .sum();

        let total_solution_time: Duration = execution_result
            .results
            .iter()
            .map(|(_, metrics)| metrics.solution_time)
            .sum();

        let count = execution_result.results.len();
        let average_solution_time = if count > 0 {
            Duration::from_nanos(total_solution_time.as_nanos() as u64 / count as u64)
        } else {
            Duration::from_secs(0)
        };

        Self {
            total_execution_time,
            total_solution_time,
            average_solution_time,
        }
    }
}

pub(crate) fn display_results(execution_result: &ExecutionResult) {
    let metrics = Metrics::new(execution_result);

    println!("\nResults:");
    println!("========");

    for (result, metrics) in &execution_result.results {
        println!(
            "Day: {}, Part: {}, Result: {} (Solution time: {:.2}ms, Total time: {:.2}ms)",
            result.day(),
            result.part(),
            result.result(),
            metrics.solution_time.as_secs_f64() * 1000.0,
            metrics.total_time.as_secs_f64() * 1000.0,
        );
    }

    println!("\nPerformance Metrics:");
    println!("===================");
    println!(
        "Total Execution Time: {:.2}ms",
        metrics.total_execution_time.as_secs_f64() * 1000.0
    );
    println!(
        "Total Solution Time: {:.2}ms",
        metrics.total_solution_time.as_secs_f64() * 1000.0
    );
    println!(
        "Average Solution Time: {:.2}ms",
        metrics.average_solution_time.as_secs_f64() * 1000.0
    );
}
