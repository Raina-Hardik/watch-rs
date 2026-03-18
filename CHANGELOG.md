# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2026-03-18

### Added
- GitHub Actions workflow for cross-platform CI/CD builds
  - Builds for: Linux (GNU, musl), Windows (MSVC, GNU), macOS (x86_64, ARM64)
  - Automatic release creation with pre-built binaries
- `.pre-commit-config.yaml` for local code quality checks
  - Rust formatting via `rustfmt`
  - Linting via `clippy`
  - General pre-commit hooks (trailing whitespace, YAML validation, etc.)
- `just publish` recipe for streamlined version bumping and release automation
  - Updates Cargo.toml version
  - Updates CHANGELOG.md with release date
  - Creates annotated git tag
  - Pushes to GitHub (triggers CI/CD)

### Changed
- `.github/CONTRIBUTING.md` now references pre-commit setup

## [Released]

### Added

- `.github/CONTRIBUTING.md` with PR guidelines and review process
- `justfile` for cross-platform development workflows
  - Format, lint, compile, test, and dev targets
  - Platform-specific recipes using bash (Unix) and PowerShell (Windows)
  - Build configuration for multiple targets
  - Install and release tasks

## [0.1.0] - 2026-01-29

### Added

- Initial release
- Core watch loop functionality
- Configurable update interval (`-n, --interval`)
- Difference highlighting (`-d, --differences`)
- ANSI color support (`-c, --color`, `-C, --no-color`)
- Header toggle (`-t, --no-title`)
- Beep on error (`-b, --beep`)
- Exit on error (`-e, --errexit`)
- Exit on change (`-g, --chgexit`)
- Exit on equal (`-q, --equexit`)
- Precise timing mode (`-p, --precise`)
- Direct exec mode (`-x, --exec`)
- Line wrap toggle (`-w, --no-wrap`)
- No rerun on resize (`-r, --no-rerun`)
- Follow mode (`-f, --follow`)
- Screenshot feature (`-s, --shotsdir`)
- Interactive key controls (q, Space, s)
- Cross-platform support (Windows, Linux, macOS)
- Environment variable support (WATCH_INTERVAL, COLUMNS, LINES)
