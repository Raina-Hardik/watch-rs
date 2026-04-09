//! Configuration structures for watch-rs

use std::path::PathBuf;
use std::time::Duration;

use crate::cli::Args;

/// Command execution mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandMode {
    /// Decide automatically between direct exec and shell execution
    Auto,
    /// Always execute via a shell
    Shell,
    /// Always execute directly without a shell
    Exec,
}

/// Difference highlighting mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiffMode {
    /// No difference highlighting
    None,
    /// Highlight differences from previous update only
    Normal,
    /// Highlight all differences since first update
    Permanent,
}

/// Runtime configuration derived from command-line arguments
#[derive(Debug, Clone)]
pub struct Config {
    /// Update interval
    pub interval: Duration,
    /// Difference highlighting mode
    pub diff_mode: DiffMode,
    /// Whether to interpret ANSI colors
    pub color: bool,
    /// Whether to show the header
    pub show_title: bool,
    /// Whether to beep on error
    pub beep: bool,
    /// Whether to exit on command error
    pub errexit: bool,
    /// Whether to exit on output change
    pub chgexit: bool,
    /// Number of unchanged cycles before exit (None = disabled)
    pub equexit: Option<u32>,
    /// Whether to use precise timing
    pub precise: bool,
    /// Whether to use exec instead of shell
    pub command_mode: CommandMode,
    /// Whether to wrap long lines
    pub wrap: bool,
    /// Whether to rerun on terminal resize
    pub rerun_on_resize: bool,
    /// Whether to use follow mode (scroll instead of clear)
    pub follow: bool,
    /// Directory for screenshots
    pub shotsdir: Option<PathBuf>,
    /// The command to execute
    pub command: Vec<String>,
}

impl Config {
    /// Create a new Config from command-line arguments
    pub fn from_args(args: &Args) -> Self {
        let diff_mode = match &args.differences {
            None => DiffMode::None,
            Some(mode) if mode == "permanent" || mode == "1" => DiffMode::Permanent,
            Some(_) => DiffMode::Normal,
        };

        // Determine color mode: --color wins, --no-color disables, default is auto
        let color = if args.no_color { false } else { args.color };
        let command_mode = if args.exec {
            CommandMode::Exec
        } else if args.shell {
            CommandMode::Shell
        } else {
            CommandMode::Auto
        };

        Config {
            interval: Duration::from_secs_f64(args.interval),
            diff_mode,
            color,
            show_title: !args.no_title,
            beep: args.beep,
            errexit: args.errexit,
            chgexit: args.chgexit,
            equexit: args.equexit,
            precise: args.precise,
            command_mode,
            wrap: !args.no_wrap,
            rerun_on_resize: !args.no_rerun,
            follow: args.follow,
            shotsdir: args.shotsdir.clone(),
            command: args.command.clone(),
        }
    }

    /// Get the command as a single string
    pub fn command_string(&self) -> String {
        self.command.join(" ")
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            interval: Duration::from_secs(2),
            diff_mode: DiffMode::None,
            color: false,
            show_title: true,
            beep: false,
            errexit: false,
            chgexit: false,
            equexit: None,
            precise: false,
            command_mode: CommandMode::Auto,
            wrap: true,
            rerun_on_resize: true,
            follow: false,
            shotsdir: None,
            command: vec![],
        }
    }
}
