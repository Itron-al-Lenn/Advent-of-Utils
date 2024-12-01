mod loader;
mod runner;

use advent_of_utils::Parts;
use clap::Parser;
use runner::Config;
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
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let config = Config {
        year: cli.year,
        day: cli.day,
        part: match cli.part {
            Some(num) => match Parts::new(num) {
                Ok(part) => Some(part),
                Err(error) => {
                    println!("Error: {}", error);
                    process::exit(1);
                }
            },
            None => None,
        },
        workspace_dir: cli.workspace_dir + "/target/release",
        test_mode: cli.test,
        input_dir: cli.input_dir,
    };

    if let Err(error) = runner::run(&config).await {
        println!("Error: {}", error);
        process::exit(1);
    };
}
