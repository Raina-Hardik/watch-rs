//! Terminal display handling for watch-rs

use std::collections::HashSet;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

use anyhow::Result;
use chrono::Local;
use crossterm::{
    ExecutableCommand, QueueableCommand,
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    style::{Attribute, Color, ResetColor, SetAttribute, SetForegroundColor},
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};

use crate::config::{Config, DiffMode};
use crate::diff::ChangedPosition;
use crate::runner::CommandResult;

/// Terminal display manager
pub struct Display {
    /// Terminal width
    width: u16,
    /// Terminal height
    height: u16,
    /// Whether we're in alternate screen mode
    in_alternate_screen: bool,
    /// Screenshot counter
    screenshot_counter: u32,
    /// Buffer for screenshot content
    screen_buffer: Vec<String>,
}

#[allow(dead_code)]
impl Display {
    /// Create a new display manager
    pub fn new() -> Result<Self> {
        let (width, height) = terminal::size()?;

        Ok(Display {
            width,
            height,
            in_alternate_screen: false,
            screenshot_counter: 0,
            screen_buffer: Vec::new(),
        })
    }

    /// Initialize the terminal for watch mode
    pub fn init(&mut self) -> Result<()> {
        let mut stdout = io::stdout();
        terminal::enable_raw_mode()?;
        stdout.execute(EnterAlternateScreen)?;
        stdout.execute(Hide)?;
        self.in_alternate_screen = true;
        Ok(())
    }

    /// Restore the terminal to normal mode
    pub fn cleanup(&mut self) -> Result<()> {
        let mut stdout = io::stdout();
        if self.in_alternate_screen {
            stdout.execute(Show)?;
            stdout.execute(LeaveAlternateScreen)?;
            self.in_alternate_screen = false;
        }
        terminal::disable_raw_mode()?;
        Ok(())
    }

    /// Update terminal dimensions
    pub fn update_size(&mut self) -> Result<bool> {
        let (new_width, new_height) = terminal::size()?;
        let changed = new_width != self.width || new_height != self.height;
        self.width = new_width;
        self.height = new_height;
        Ok(changed)
    }

    /// Get terminal dimensions
    pub fn size(&self) -> (u16, u16) {
        (self.width, self.height)
    }

    /// Clear the screen
    pub fn clear(&self) -> Result<()> {
        let mut stdout = io::stdout();
        stdout.queue(MoveTo(0, 0))?;
        stdout.queue(Clear(ClearType::All))?;
        stdout.flush()?;
        Ok(())
    }

    /// Render the header
    fn render_header(
        &self,
        stdout: &mut io::Stdout,
        config: &Config,
        result: &CommandResult,
        _iteration: u64,
    ) -> Result<u16> {
        if !config.show_title {
            return Ok(0);
        }

        let now = Local::now();
        let time_str = now.format("%a %b %d %H:%M:%S %Y").to_string();

        let interval_str = format!("{:.1}s", config.interval.as_secs_f64());
        let command_str = config.command_string();

        // First line: "Every Xs: command"
        let header_left = format!("Every {}: {}", interval_str, command_str);

        // Truncate if too long
        let max_left_width = self.width.saturating_sub(time_str.len() as u16 + 2) as usize;
        let header_left = if header_left.len() > max_left_width {
            format!("{}...", &header_left[..max_left_width.saturating_sub(3)])
        } else {
            header_left
        };

        // Calculate padding to right-align the time
        let padding = self.width as usize - header_left.len() - time_str.len();
        let padding_str = " ".repeat(padding.max(1));

        stdout.queue(MoveTo(0, 0))?;
        stdout.queue(SetAttribute(Attribute::Bold))?;
        write!(stdout, "{}{}{}", header_left, padding_str, time_str)?;
        stdout.queue(SetAttribute(Attribute::Reset))?;

        // Second line: exit status if non-zero
        let mut lines_used = 1;

        if !result.success {
            stdout.queue(MoveTo(0, 1))?;
            stdout.queue(SetForegroundColor(Color::Red))?;
            if let Some(code) = result.exit_code {
                write!(stdout, "Exit code: {}", code)?;
            } else {
                write!(stdout, "Command terminated by signal")?;
            }
            stdout.queue(ResetColor)?;
            lines_used = 2;
        }

        // Empty line after header
        stdout.queue(MoveTo(0, lines_used))?;
        writeln!(stdout)?;

        Ok(lines_used + 1)
    }

    /// Render the command output
    fn render_output(
        &mut self,
        stdout: &mut io::Stdout,
        output: &str,
        config: &Config,
        changes: &HashSet<ChangedPosition>,
        start_line: u16,
    ) -> Result<()> {
        self.screen_buffer.clear();

        let available_height = self.height.saturating_sub(start_line) as usize;
        let lines: Vec<&str> = output.lines().collect();

        for (line_idx, line) in lines.iter().take(available_height).enumerate() {
            stdout.queue(MoveTo(0, start_line + line_idx as u16))?;

            let display_line = if config.wrap {
                line.to_string()
            } else {
                // Truncate line if too long
                if line.len() > self.width as usize {
                    line[..self.width as usize].to_string()
                } else {
                    line.to_string()
                }
            };

            // Check if we need to highlight differences
            if config.diff_mode != DiffMode::None && !changes.is_empty() {
                self.render_line_with_diff(stdout, &display_line, line_idx, changes)?;
            } else {
                write!(stdout, "{}", display_line)?;
            }

            self.screen_buffer.push(display_line);
        }

        Ok(())
    }

    /// Render a line with difference highlighting
    fn render_line_with_diff(
        &self,
        stdout: &mut io::Stdout,
        line: &str,
        line_idx: usize,
        changes: &HashSet<ChangedPosition>,
    ) -> Result<()> {
        let chars: Vec<char> = line.chars().collect();
        let mut in_highlight = false;

        for (col_idx, ch) in chars.iter().enumerate() {
            let is_changed = changes.contains(&ChangedPosition {
                line: line_idx,
                col: col_idx,
            });

            if is_changed && !in_highlight {
                stdout.queue(SetAttribute(Attribute::Reverse))?;
                in_highlight = true;
            } else if !is_changed && in_highlight {
                stdout.queue(SetAttribute(Attribute::NoReverse))?;
                in_highlight = false;
            }

            write!(stdout, "{}", ch)?;
        }

        if in_highlight {
            stdout.queue(SetAttribute(Attribute::NoReverse))?;
        }

        Ok(())
    }

    /// Render the full screen
    pub fn render(
        &mut self,
        config: &Config,
        result: &CommandResult,
        output: &str,
        changes: &HashSet<ChangedPosition>,
        iteration: u64,
    ) -> Result<()> {
        let mut stdout = io::stdout();

        if !config.follow {
            self.clear()?;
        }

        let header_lines = self.render_header(&mut stdout, config, result, iteration)?;
        self.render_output(&mut stdout, output, config, changes, header_lines)?;

        stdout.flush()?;
        Ok(())
    }

    /// Take a screenshot
    pub fn take_screenshot(&mut self, shotsdir: Option<&PathBuf>) -> Result<PathBuf> {
        self.screenshot_counter += 1;

        let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
        let filename = format!("watch_{}_{}.txt", timestamp, self.screenshot_counter);

        let path = match shotsdir {
            Some(dir) => {
                fs::create_dir_all(dir)?;
                dir.join(filename)
            }
            None => PathBuf::from(filename),
        };

        let content = self.screen_buffer.join("\n");
        fs::write(&path, content)?;

        Ok(path)
    }

    /// Beep the terminal
    pub fn beep(&self) -> Result<()> {
        print!("\x07");
        io::stdout().flush()?;
        Ok(())
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        let _ = self.cleanup();
    }
}

/// Input event types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputEvent {
    /// Quit the program
    Quit,
    /// Force immediate refresh
    Refresh,
    /// Take a screenshot
    Screenshot,
    /// Terminal was resized
    Resize,
    /// No event (timeout)
    None,
}

/// Check for keyboard input with a timeout
pub fn poll_input(timeout: std::time::Duration) -> Result<InputEvent> {
    if event::poll(timeout)? {
        match event::read()? {
            Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::NONE,
                ..
            }) => Ok(InputEvent::Quit),

            Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
                ..
            }) => Ok(InputEvent::Quit),

            Event::Key(KeyEvent {
                code: KeyCode::Char(' '),
                ..
            }) => Ok(InputEvent::Refresh),

            Event::Key(KeyEvent {
                code: KeyCode::Char('s'),
                modifiers: KeyModifiers::NONE,
                ..
            }) => Ok(InputEvent::Screenshot),

            Event::Resize(_, _) => Ok(InputEvent::Resize),

            _ => Ok(InputEvent::None),
        }
    } else {
        Ok(InputEvent::None)
    }
}

/// Wait for any key press
pub fn wait_for_key() -> Result<()> {
    loop {
        if event::poll(std::time::Duration::from_millis(100))?
            && matches!(event::read()?, Event::Key(_))
        {
            break;
        }
    }
    Ok(())
}
