//! Command execution for watch-rs

use anyhow::{Context, Result};
use std::borrow::Cow;
use std::process::{Command, Output, Stdio};

use crate::config::CommandMode;

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
    pub fn combined_output(&self) -> Cow<'_, str> {
        if self.stderr.is_empty() {
            Cow::Borrowed(&self.stdout)
        } else if self.stdout.is_empty() {
            Cow::Borrowed(&self.stderr)
        } else {
            let mut combined = String::with_capacity(self.stdout.len() + 1 + self.stderr.len());
            combined.push_str(&self.stdout);
            combined.push('\n');
            combined.push_str(&self.stderr);
            Cow::Owned(combined)
        }
    }
}

/// Command runner that handles platform differences
pub struct Runner {
    /// Command execution strategy
    mode: CommandMode,
}

impl Runner {
    /// Create a new runner
    pub fn new(mode: CommandMode) -> Self {
        Runner { mode }
    }

    /// Run a command and return the result
    pub fn run(&self, command: &[String]) -> Result<CommandResult> {
        let output = match self.mode {
            CommandMode::Exec => self.run_exec(command)?,
            CommandMode::Shell => self.run_shell(command)?,
            CommandMode::Auto => {
                if should_use_shell(command) {
                    self.run_shell(command)?
                } else {
                    self.run_exec(command)?
                }
            }
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
        if command.is_empty() {
            anyhow::bail!("No command specified");
        }

        let command_str = build_shell_command(command);

        #[cfg(windows)]
        let output =
            Command::new(std::env::var("COMSPEC").unwrap_or_else(|_| "cmd.exe".to_string()))
                .args(["/C", &command_str])
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output()
                .with_context(|| format!("Failed to execute command: {}", command_str))?;

        #[cfg(not(windows))]
        let output = Command::new(std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string()))
            .args(["-c", &command_str])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .with_context(|| format!("Failed to execute command: {}", command_str))?;

        Ok(output)
    }
}

fn should_use_shell(command: &[String]) -> bool {
    if command.is_empty() {
        return false;
    }

    // A single argument frequently means the user intentionally passed a shell command
    // (e.g. "echo $HOME | sed ...") and expects shell features.
    if command.len() == 1 {
        return true;
    }

    command
        .iter()
        .any(|arg| is_shell_operator(arg) || looks_like_env_assignment(arg))
}

fn is_shell_operator(arg: &str) -> bool {
    matches!(
        arg,
        "|" | "||" | "&&" | ";" | "<" | ">" | ">>" | "2>" | "2>>" | "2>&1" | "(" | ")"
    )
}

fn looks_like_env_assignment(arg: &str) -> bool {
    let Some((name, value)) = arg.split_once('=') else {
        return false;
    };

    !name.is_empty()
        && !value.is_empty()
        && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn build_shell_command(command: &[String]) -> String {
    if command.len() == 1 {
        return command[0].clone();
    }

    #[cfg(windows)]
    {
        command
            .iter()
            .map(|arg| quote_for_cmd(arg))
            .collect::<Vec<_>>()
            .join(" ")
    }

    #[cfg(not(windows))]
    {
        command
            .iter()
            .map(|arg| quote_for_posix_sh(arg))
            .collect::<Vec<_>>()
            .join(" ")
    }
}

#[cfg(windows)]
fn quote_for_cmd(arg: &str) -> String {
    if arg.is_empty() {
        return "\"\"".to_string();
    }

    let mut escaped = String::with_capacity(arg.len() + 8);
    for ch in arg.chars() {
        match ch {
            '^' | '&' | '|' | '<' | '>' | '(' | ')' => {
                escaped.push('^');
                escaped.push(ch);
            }
            '%' => escaped.push_str("%%"),
            '!' => escaped.push_str("^!"),
            '"' => escaped.push_str("\\\""),
            _ => escaped.push(ch),
        }
    }

    let needs_quotes = arg.chars().any(char::is_whitespace);
    if needs_quotes {
        format!("\"{}\"", escaped)
    } else {
        escaped
    }
}

#[cfg(not(windows))]
fn quote_for_posix_sh(arg: &str) -> String {
    if arg.is_empty() {
        return "''".to_string();
    }

    let escaped = arg.replace('\'', "'\"'\"'");
    format!("'{}'", escaped)
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
    use crate::config::CommandMode;

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

    #[test]
    fn test_combined_output_borrows_stdout_when_stderr_empty() {
        let result = CommandResult {
            stdout: "out".to_string(),
            stderr: String::new(),
            exit_code: Some(0),
            success: true,
        };

        assert!(matches!(result.combined_output(), Cow::Borrowed("out")));
    }

    #[test]
    fn test_combined_output_borrows_stderr_when_stdout_empty() {
        let result = CommandResult {
            stdout: String::new(),
            stderr: "err".to_string(),
            exit_code: Some(1),
            success: false,
        };

        assert!(matches!(result.combined_output(), Cow::Borrowed("err")));
    }

    #[test]
    fn test_combined_output_allocates_when_both_present() {
        let result = CommandResult {
            stdout: "out".to_string(),
            stderr: "err".to_string(),
            exit_code: Some(1),
            success: false,
        };

        assert!(matches!(
            result.combined_output(),
            Cow::Owned(ref s) if s == "out\nerr"
        ));
    }

    #[test]
    fn test_auto_mode_prefers_exec_for_simple_commands() {
        assert!(!should_use_shell(&[
            "echo".to_string(),
            "hello world".to_string()
        ]));
    }

    #[test]
    fn test_auto_mode_uses_shell_for_operators() {
        assert!(should_use_shell(&[
            "echo".to_string(),
            "hello".to_string(),
            "|".to_string(),
            "grep".to_string(),
            "h".to_string()
        ]));
    }

    #[test]
    fn test_auto_mode_uses_shell_for_single_string() {
        assert!(should_use_shell(&["echo $HOME && date".to_string()]));
    }

    #[test]
    fn test_runner_construction() {
        let runner = Runner::new(CommandMode::Auto);
        assert!(matches!(runner.mode, CommandMode::Auto));
    }
}
