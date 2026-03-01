mod cli;

use clap::Parser;
use tracing::Level;

use cli::{Args, Mode};

fn main() {
    let args = Args::parse();

    // for verbose flag, DEBUG level logs shows the resolution steps
    // WARN will show WARN and below logs
    let level = if args.verbose {
        Level::DEBUG
    } else {
        Level::WARN
    };
    tracing_subscriber::fmt()
        .with_max_level(level)
        .with_target(false)
        .without_time()
        .init();

    let result = match args.mode {
        Mode::Stub => {
            let resolver = String::from("Stub placeholder");
            resolver
        }
        Mode::Recursive => {
            let resolver = String::from("Recursive placeholder");
            resolver
        }
    };

    println!("Level: {level}");
    println!("Result: {result}");
}
