# watch-rs

A cross-platform Rust implementation of the Unix `watch` command. Execute a program periodically, showing output fullscreen.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Overview

`watch-rs` runs a command repeatedly, displaying its output and errors on the terminal. This allows you to watch the program output change over time. By default, the command is run every 2 seconds and watch will run until interrupted.

**Primary Goal:** Enable Windows, macOS, and Linux users to have the utility of the `watch` command through Rust's cross-compilation capabilities.

## Features

- **Cross-platform:** Works on Windows, Linux, and macOS
- **Interval control:** Configurable update intervals (default: 2 seconds)
- **Difference highlighting:** Highlight changes between successive updates
- **Color support:** Interpret ANSI color and style sequences
- **Precise timing:** Execute commands at precise intervals
- **Exit conditions:** Exit on command error, output change, or output stability
- **Key controls:** Interactive controls (quit, immediate refresh, screenshot)
- **Screenshots:** Save terminal output to files
- **Header display:** Shows command, time, and exit status

## Installation

### From GitHub Releases (Recommended)

Pre-built binaries are available for:
- **Linux:** x86_64 (musl)
- **Windows:** x86_64 (GNU)

Note: Release assets are intentionally minimal. Other platforms can build from source.

Download the latest release from [GitHub Releases](https://github.com/Raina-Hardik/watch-rs/releases).

### From Source with Cargo

If you have Rust installed:

```bash
cargo install --git https://github.com/Raina-Hardik/watch-rs.git
```

Or build locally:

```bash
git clone https://github.com/Raina-Hardik/watch-rs.git
cd watch-rs
cargo build --release
```

The binary will be available at `target/release/watch-rs` (or `watch-rs.exe` on Windows).

### From Cargo (crates.io)

Coming soon! Once published to crates.io:

```bash
cargo install watch-rs
```

## Usage

```bash
watch-rs [OPTIONS] <COMMAND>...
```

### GNU watch Compatibility

`watch-rs` aims to be a best-effort stand-in for the Unix `watch` command, built with Windows in mind. It strives to match the behavior of the original:

- Same default interval (2s)
- Same flags (`-n`, `-d`, `-t`, etc.)
- Same fullscreen output behavior

Command execution mode is best-effort and cross-platform:

- Auto mode (default): execute directly for simple commands, use shell when shell syntax is detected
- `--shell`: always execute through shell (`$SHELL -c` on Unix, `%COMSPEC% /C` on Windows)
- `-x, --exec`: always execute directly (shell-independent, predictable argument handling)

**Note:** Some edge cases may not be fully covered. If you encounter differences or issues, please [open an issue](https://github.com/Raina-Hardik/watch-rs/issues) or submit a [pull request](https://github.com/Raina-Hardik/watch-rs/pulls). That's what they're here for!

### Windows Users: Create an Alias

To use `watch-rs` as a drop-in replacement for `watch`, add this to your PowerShell profile:

```powershell
Set-Alias -Name watch -Value watch-rs
```

Then you can use `watch` just like you would on Unix:

```powershell
watch dir
watch -n 1 "Get-Process | Select-Object Name, CPU"
```

### Options

| Option | Description |
|--------|-------------|
| `-n, --interval <SECS>` | Update interval in seconds (default: 2.0) |
| `-d, --differences[=permanent]` | Highlight differences between updates |
| `-c, --color` | Interpret ANSI color sequences |
| `-C, --no-color` | Do not interpret ANSI color sequences |
| `-t, --no-title` | Turn off the header |
| `-b, --beep` | Beep on command error (non-zero exit) |
| `-e, --errexit` | Exit on command error |
| `-g, --chgexit` | Exit when output changes |
| `-q, --equexit <CYCLES>` | Exit when output is unchanged for N cycles |
| `-p, --precise` | Precise timing mode |
| `-x, --exec` | Always execute directly (without shell) |
| `--shell` | Always execute through shell |
| `-w, --no-wrap` | Disable line wrapping |
| `-r, --no-rerun` | Don't rerun on terminal resize |
| `-f, --follow` | Scroll output like tail -f |
| `-s, --shotsdir <DIR>` | Directory to save screenshots |
| `-h, --help` | Display help |
| `-v, --version` | Display version |

### Key Controls

| Key | Action |
|-----|--------|
| `q` | Quit watch-rs |
| `Space` | Immediately run command |
| `s` | Take a screenshot |

### Examples

Watch directory contents:
```bash
watch-rs -d ls -l
```

Monitor with 1-second interval:
```bash
watch-rs -n 1 date
```

Watch with precise timing:
```bash
watch-rs -n 10 -p "date; echo Hello"
```

Exit when output changes:
```bash
watch-rs -g cat /etc/passwd
```

## Environment Variables

| Variable | Description |
|----------|-------------|
| `WATCH_INTERVAL` | Default update interval |
| `COLUMNS` | Override terminal width |
| `LINES` | Override terminal height |

## Exit Status

| Code | Description |
|------|-------------|
| 0 | Success |
| 1 | General errors |
| 2 | Command execution errors |
| N | With `--errexit`, returns command's exit code |

## Building for Different Platforms

### Windows
```bash
cargo build --release --target x86_64-pc-windows-gnu
```

### Linux
```bash
cargo build --release --target x86_64-unknown-linux-musl
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Inspired by the original `watch` command from [procps-ng](https://gitlab.com/procps-ng/procps)
- Built with Rust for cross-platform compatibility

## Version History

See [CHANGELOG.md](CHANGELOG.md) for version history.
