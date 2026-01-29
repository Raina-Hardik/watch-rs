//! Command execution for watch-rs

use anyhow::{Context, Result};
use std::process::{Command, Output, Stdio};

/// Result of running a command
#[derive(Debug, Clone)]
pub struct CommandResult {
    /// Standard output
    pub stdout: String,
    /// Standard error
    pub stderr: String,
    /// Exit code (None if killed by signal)
    pub exit_code: Option<i32>,
    /// Whether the command succeeded (exit code 0)
    pub success: bool,
}

impl CommandResult {
    /// Get the combined output (stdout + stderr)
    pub fn combined_output(&self) -> String {
        if self.stderr.is_empty() {
            self.stdout.clone()
        } else if self.stdout.is_empty() {
            self.stderr.clone()
        } else {
            format!("{}\n{}", self.stdout, self.stderr)
        }
    }
}

/// Command runner that handles platform differences
pub struct Runner {
    /// Whether to use exec mode (direct execution)
    exec_mode: bool,
}

impl Runner {
    /// Create a new runner
    pub fn new(exec_mode: bool) -> Self {
        Runner { exec_mode }
    }

    /// Run a command and return the result
    pub fn run(&self, command: &[String]) -> Result<CommandResult> {
        let output = if self.exec_mode {
            self.run_exec(command)?
        } else {
            self.run_shell(command)?
        };

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let exit_code = output.status.code();
        let success = output.status.success();

        Ok(CommandResult {
            stdout,
            stderr,
            exit_code,
            success,
        })
    }

    /// Run command through exec (direct execution)
    fn run_exec(&self, command: &[String]) -> Result<Output> {
        if command.is_empty() {
            anyhow::bail!("No command specified");
        }

        let program = &command[0];
        let args = &command[1..];

        Command::new(program)
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .with_context(|| format!("Failed to execute command: {}", program))
    }

    /// Run command through shell
    fn run_shell(&self, command: &[String]) -> Result<Output> {
        let command_str = command.join(" ");

        #[cfg(windows)]
        let output = Command::new("cmd")
            .args(["/C", &command_str])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .with_context(|| format!("Failed to execute command: {}", command_str))?;

        #[cfg(not(windows))]
        let output = Command::new("sh")
            .args(["-c", &command_str])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .with_context(|| format!("Failed to execute command: {}", command_str))?;

        Ok(output)
    }
}

/// Strip non-printable characters from output (except ANSI escape sequences)
pub fn strip_non_printable(s: &str, preserve_ansi: bool) -> String {
    if preserve_ansi {
        // Keep ANSI escape sequences, remove other non-printable chars
        let mut result = String::with_capacity(s.len());
        let mut chars = s.chars().peekable();

        while let Some(c) = chars.next() {
            if c == '\x1b' {
                // Start of ANSI escape sequence
                result.push(c);
                // Consume until we hit a letter (end of sequence)
                while let Some(&next) = chars.peek() {
                    result.push(chars.next().unwrap());
                    if next.is_ascii_alphabetic() {
                        break;
                    }
                }
            } else if c.is_control() && c != '\n' && c != '\r' && c != '\t' {
                // Skip non-printable control characters (except newline, carriage return, tab)
                continue;
            } else {
                result.push(c);
            }
        }

        result
    } else {
        // Remove all ANSI escape sequences and non-printable chars
        let mut result = String::with_capacity(s.len());
        let mut chars = s.chars().peekable();

        while let Some(c) = chars.next() {
            if c == '\x1b' {
                // Skip ANSI escape sequence
                while let Some(&next) = chars.peek() {
                    chars.next();
                    if next.is_ascii_alphabetic() {
                        break;
                    }
                }
            } else if c.is_control() && c != '\n' && c != '\r' && c != '\t' {
                // Skip non-printable control characters
                continue;
            } else {
                result.push(c);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_non_printable_basic() {
        let input = "Hello\x00World";
        let result = strip_non_printable(input, false);
        assert_eq!(result, "HelloWorld");
    }

    #[test]
    fn test_strip_preserves_newlines() {
        let input = "Hello\nWorld";
        let result = strip_non_printable(input, false);
        assert_eq!(result, "Hello\nWorld");
    }

    #[test]
    fn test_strip_ansi_when_disabled() {
        let input = "Hello \x1b[31mRed\x1b[0m World";
        let result = strip_non_printable(input, false);
        assert_eq!(result, "Hello Red World");
    }

    #[test]
    fn test_preserve_ansi_when_enabled() {
        let input = "Hello \x1b[31mRed\x1b[0m World";
        let result = strip_non_printable(input, true);
        assert_eq!(result, "Hello \x1b[31mRed\x1b[0m World");
    }
}
