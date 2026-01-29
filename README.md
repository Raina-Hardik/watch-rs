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

### From Source

```bash
git clone https://github.com/Raina-Hardik/watch-rs.git
cd watch-rs
cargo build --release
```

The binary will be available at `target/release/watch-rs` (or `watch-rs.exe` on Windows).

### From Cargo

```bash
cargo install watch-rs
```

## Usage

```bash
watch-rs [OPTIONS] <COMMAND>...
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
| `-x, --exec` | Pass command to exec instead of shell |
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
cargo build --release --target x86_64-pc-windows-msvc
```

### Linux
```bash
cargo build --release --target x86_64-unknown-linux-gnu
```

### macOS
```bash
cargo build --release --target x86_64-apple-darwin
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
