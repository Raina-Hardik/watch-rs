//! watch-rs: A cross-platform Rust implementation of the Unix watch command
//!
//! Execute a program periodically, showing output fullscreen.

mod cli;
mod config;
mod diff;
mod display;
mod runner;
mod watch;

use anyhow::Result;
use std::process::ExitCode;

use cli::Args;
use config::Config;

fn main() -> ExitCode {
    match run() {
        Ok(code) => ExitCode::from(code as u8),
        Err(e) => {
            eprintln!("watch-rs: {}", e);
            ExitCode::from(1)
        }
    }
}

fn run() -> Result<i32> {
    // Parse command-line arguments
    let args = Args::parse_args();

    // Create configuration
    let config = Config::from_args(&args);

    // Run the watch loop
    watch::run(config)
}
