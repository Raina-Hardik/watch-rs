//! Core watch loop for watch-rs

use std::time::{Duration, Instant};

use anyhow::Result;

use crate::config::{Config, DiffMode};
use crate::diff::DiffEngine;
use crate::display::{Display, InputEvent, poll_input, wait_for_key};
use crate::runner::{Runner, strip_non_printable};

/// Exit status codes
pub const EXIT_SUCCESS: i32 = 0;
#[allow(dead_code)]
pub const EXIT_ERROR: i32 = 1;
pub const EXIT_COMMAND_ERROR: i32 = 2;

/// State for the watch loop
struct WatchState {
    /// Current iteration number
    iteration: u64,
    /// Previous output for change detection
    previous_output: String,
    /// Number of consecutive cycles with unchanged output
    unchanged_cycles: u32,
    /// Last command exit code
    last_exit_code: Option<i32>,
    /// Whether the command is currently running
    command_running: bool,
    /// Whether a screenshot was requested
    screenshot_requested: bool,
}

impl WatchState {
    fn new() -> Self {
        WatchState {
            iteration: 0,
            previous_output: String::new(),
            unchanged_cycles: 0,
            last_exit_code: None,
            command_running: false,
            screenshot_requested: false,
        }
    }
}

/// Run the main watch loop
pub fn run(config: Config) -> Result<i32> {
    let mut display = Display::new()?;
    let mut diff_engine = DiffEngine::new();
    let runner = Runner::new(config.exec);
    let mut state = WatchState::new();

    // Initialize display
    display.init()?;

    // Main loop
    let exit_code = watch_loop(&config, &mut display, &mut diff_engine, &runner, &mut state)?;

    // Cleanup is handled by Display::drop
    Ok(exit_code)
}

/// The main watch loop
fn watch_loop(
    config: &Config,
    display: &mut Display,
    diff_engine: &mut DiffEngine,
    runner: &Runner,
    state: &mut WatchState,
) -> Result<i32> {
    let poll_interval = Duration::from_millis(50);
    let mut next_run = Instant::now();
    let mut force_refresh = true;

    loop {
        let now = Instant::now();

        // Check for input events
        let remaining = if force_refresh {
            Duration::ZERO
        } else {
            next_run.saturating_duration_since(now)
        };

        let poll_time = remaining.min(poll_interval);
        let event = poll_input(poll_time)?;

        match event {
            InputEvent::Quit => {
                return Ok(EXIT_SUCCESS);
            }
            InputEvent::Refresh => {
                force_refresh = true;
            }
            InputEvent::Screenshot => {
                state.screenshot_requested = true;
            }
            InputEvent::Resize => {
                if display.update_size()? {
                    // Terminal was resized
                    diff_engine.reset();
                    if config.rerun_on_resize {
                        force_refresh = true;
                    }
                }
            }
            InputEvent::None => {}
        }

        // Check if it's time to run the command
        let should_run = force_refresh || Instant::now() >= next_run;

        if should_run {
            force_refresh = false;
            state.iteration += 1;
            state.command_running = true;

            let run_start = Instant::now();

            // Execute the command
            let result = runner.run(&config.command)?;
            state.command_running = false;
            state.last_exit_code = result.exit_code;

            // Process output
            let output = result.combined_output();
            let processed_output = strip_non_printable(&output, config.color);

            // Calculate differences if needed
            let changes = if config.diff_mode != DiffMode::None {
                let permanent = config.diff_mode == DiffMode::Permanent;
                diff_engine.calculate_diff(&processed_output, permanent)
            } else {
                std::collections::HashSet::new()
            };

            // Render the display
            display.render(
                config,
                &result,
                &processed_output,
                &changes,
                state.iteration,
            )?;

            // Handle screenshot request
            if state.screenshot_requested {
                state.screenshot_requested = false;
                if let Ok(path) = display.take_screenshot(config.shotsdir.as_ref()) {
                    // Could show a brief notification, but for simplicity we'll skip it
                    let _ = path; // Suppress unused warning
                }
            }

            // Beep on error if requested
            if config.beep && !result.success {
                display.beep()?;
            }

            // Check exit conditions
            if config.errexit && !result.success {
                // Wait for key press before exiting
                wait_for_key()?;
                return Ok(result.exit_code.unwrap_or(EXIT_COMMAND_ERROR));
            }

            // Check if output changed
            let output_changed = processed_output != state.previous_output;

            if config.chgexit && output_changed && state.iteration > 1 {
                return Ok(EXIT_SUCCESS);
            }

            // Handle equexit
            if let Some(cycles) = config.equexit {
                if output_changed {
                    state.unchanged_cycles = 0;
                } else {
                    state.unchanged_cycles += 1;
                    if state.unchanged_cycles >= cycles {
                        return Ok(EXIT_SUCCESS);
                    }
                }
            }

            state.previous_output = processed_output;

            // Calculate next run time
            if config.precise {
                // Precise mode: interval from start of previous run
                next_run = run_start + config.interval;
                // If we're already past the next run time, run immediately
                if next_run < Instant::now() {
                    next_run = Instant::now();
                }
            } else {
                // Normal mode: interval from end of previous run
                next_run = Instant::now() + config.interval;
            }
        }
    }
}
