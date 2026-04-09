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
        Ok(code) => ExitCode::from(normalize_exit_code(code)),
        Err(e) => {
            eprintln!("watch-rs: {}", e);
            ExitCode::from(1)
        }
    }
}

fn normalize_exit_code(code: i32) -> u8 {
    if code < 0 {
        1
    } else {
        code.min(u8::MAX as i32) as u8
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

#[cfg(test)]
mod tests {
    use super::normalize_exit_code;

    #[test]
    fn test_normalize_exit_code_bounds() {
        assert_eq!(normalize_exit_code(-1), 1);
        assert_eq!(normalize_exit_code(0), 0);
        assert_eq!(normalize_exit_code(255), 255);
        assert_eq!(normalize_exit_code(512), 255);
    }
}
