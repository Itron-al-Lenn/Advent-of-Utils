mod config;
mod loader;
mod runner;

use advent_of_utils::Parts;
use clap::Parser;
use config::Config;
use std::process;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg()]
    year: i32,

    #[arg(short, long)]
    day: Option<u8>,

    #[arg(short, long)]
    part: Option<u8>,

    #[arg(long, default_value = "inputs")]
    input_dir: String,

    #[arg(long, default_value = ".")]
    workspace_dir: String,

    #[arg(short, long)]
    test: bool,

    #[arg(short, long)]
    exclude_parse_time: bool,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let part = cli.part.map(|num| match Parts::new(num) {
        Ok(part) => part,
        Err(error) => {
            println!("Error: {}", error);
            process::exit(1);
        }
    });

    let config = Config::new(
        cli.year,
        cli.day,
        part,
        cli.test,
        cli.input_dir,
        cli.workspace_dir,
        cli.exclude_parse_time,
    );

    if let Err(error) = runner::run(&config).await {
        println!("Error: {}", error);
        process::exit(1);
    };
}
