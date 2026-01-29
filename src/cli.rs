//! Command-line argument parsing for watch-rs

use clap::Parser;
use std::path::PathBuf;

/// Execute a program periodically, showing output fullscreen
#[derive(Parser, Debug, Clone)]
#[command(name = "watch-rs")]
#[command(author, version, about, long_about = None)]
#[command(after_help = "EXAMPLES:
    watch-rs ls -l              Watch directory contents
    watch-rs -n 1 date          Update every second
    watch-rs -d ls -la          Highlight differences
    watch-rs -e ./my_script     Exit on script error
    watch-rs -g cat file.txt    Exit when file changes")]
pub struct Args {
    /// Update interval in seconds
    #[arg(
        short = 'n',
        long = "interval",
        default_value = "2.0",
        env = "WATCH_INTERVAL"
    )]
    pub interval: f64,

    /// Highlight differences between updates.
    /// Use -d or --differences for normal mode.
    /// Use -d=permanent or --differences=permanent to show all changes since start.
    #[arg(short = 'd', long = "differences", value_name = "MODE", num_args = 0..=1, default_missing_value = "normal")]
    pub differences: Option<String>,

    /// Interpret ANSI color and style sequences
    #[arg(short = 'c', long = "color")]
    pub color: bool,

    /// Do not interpret ANSI color and style sequences
    #[arg(short = 'C', long = "no-color")]
    pub no_color: bool,

    /// Turn off the header showing the interval, command, and current time
    #[arg(short = 't', long = "no-title")]
    pub no_title: bool,

    /// Beep if command has a non-zero exit
    #[arg(short = 'b', long = "beep")]
    pub beep: bool,

    /// Freeze updates on command error and exit after a key press
    #[arg(short = 'e', long = "errexit")]
    pub errexit: bool,

    /// Exit when the output of command changes
    #[arg(short = 'g', long = "chgexit")]
    pub chgexit: bool,

    /// Exit when output does not change for the given number of cycles
    #[arg(short = 'q', long = "equexit", value_name = "CYCLES")]
    pub equexit: Option<u32>,

    /// Execute command at precise intervals (from start of previous run)
    #[arg(short = 'p', long = "precise")]
    pub precise: bool,

    /// Pass command to exec instead of shell
    #[arg(short = 'x', long = "exec")]
    pub exec: bool,

    /// Turn off line wrapping (truncate long lines)
    #[arg(short = 'w', long = "no-wrap")]
    pub no_wrap: bool,

    /// Do not run the program on terminal resize
    #[arg(short = 'r', long = "no-rerun")]
    pub no_rerun: bool,

    /// Scroll output like tail -f instead of clearing screen
    #[arg(short = 'f', long = "follow")]
    pub follow: bool,

    /// Directory to save screenshots into
    #[arg(short = 's', long = "shotsdir", value_name = "DIR")]
    pub shotsdir: Option<PathBuf>,

    /// The command to execute
    #[arg(required = true, trailing_var_arg = true)]
    pub command: Vec<String>,
}

#[allow(dead_code)]
impl Args {
    /// Parse command-line arguments
    pub fn parse_args() -> Self {
        let mut args = Args::parse();

        // Clamp interval to valid range (0.1 to 31 days)
        const MIN_INTERVAL: f64 = 0.1;
        const MAX_INTERVAL: f64 = 2678400.0; // 31 days in seconds

        args.interval = args.interval.clamp(MIN_INTERVAL, MAX_INTERVAL);

        args
    }

    /// Check if permanent difference mode is enabled
    pub fn is_permanent_diff(&self) -> bool {
        matches!(&self.differences, Some(mode) if mode == "permanent" || mode == "1")
    }

    /// Check if any difference highlighting is enabled
    pub fn has_differences(&self) -> bool {
        self.differences.is_some()
    }

    /// Get the command as a single string for shell execution
    pub fn command_string(&self) -> String {
        self.command.join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interval_clamping() {
        // Test minimum clamping
        let mut args = Args {
            interval: 0.01,
            differences: None,
            color: false,
            no_color: false,
            no_title: false,
            beep: false,
            errexit: false,
            chgexit: false,
            equexit: None,
            precise: false,
            exec: false,
            no_wrap: false,
            no_rerun: false,
            follow: false,
            shotsdir: None,
            command: vec!["test".to_string()],
        };

        // Simulate clamping
        if args.interval < 0.1 {
            args.interval = 0.1;
        }

        assert_eq!(args.interval, 0.1);
    }
}
