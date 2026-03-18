# Development Instructions & Best Practices

This document serves as a guide for developing and maintaining the watch-rs project.

## Git Workflow

### Branch Strategy

- `main` - Stable, release-ready code only
- `develop` - Integration branch for features
- `feature/*` - Feature branches (e.g., `feature/difference-highlighting`)
- `fix/*` - Bug fix branches (e.g., `fix/interval-parsing`)
- `release/*` - Release preparation branches

### Commit Message Format

Follow the Conventional Commits specification:

```
<type>(<scope>): <description>

[optional body]

[optional footer(s)]
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

**Examples:**
```
feat(cli): add --precise flag for timing mode
fix(display): correct ANSI escape sequence handling
docs(readme): update installation instructions
```

### Versioning (Semantic Versioning)

Format: `MAJOR.MINOR.PATCH`

- **MAJOR**: Breaking changes
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes (backward compatible)

**Tags:**
- Use annotated tags: `git tag -a v1.0.0 -m "Release version 1.0.0"`
- Tag format: `v<MAJOR>.<MINOR>.<PATCH>`

## Project Structure

```
watch-rs/
├── Cargo.toml          # Project manifest
├── Cargo.lock          # Dependency lock file
├── README.md           # User-facing documentation
├── INSTRUCTIONS.md     # This file (development guide)
├── CHANGELOG.md        # Version history
├── LICENSE             # MIT License
├── src/
│   ├── main.rs         # Entry point
│   ├── lib.rs          # Library root (optional)
│   ├── cli.rs          # Command-line argument parsing
│   ├── watch.rs        # Core watch loop logic
│   ├── display.rs      # Terminal display handling
│   ├── diff.rs         # Difference highlighting
│   ├── runner.rs       # Command execution
│   └── config.rs       # Configuration structures
└── tests/
    └── integration/    # Integration tests
```

## Key Dependencies

- `clap` - Command-line argument parsing (derive feature)
- `crossterm` - Cross-platform terminal manipulation
- `chrono` - Date and time handling
- `anyhow` - Error handling

## Development Checklist

### Before Each Commit

1. [ ] Run `cargo fmt` to format code
2. [ ] Run `cargo clippy` to check for lints
3. [ ] Run `cargo test` to ensure tests pass
4. [ ] Run `cargo build` to verify compilation
5. [ ] Write meaningful commit message

### Before Release

1. [ ] Update version in `Cargo.toml`
2. [ ] Update `CHANGELOG.md`
3. [ ] Run full test suite
4. [ ] Test on multiple platforms if possible
5. [ ] Create annotated tag
6. [ ] Push tags: `git push origin --tags`

## Implementation Notes

### Feature Priority

1. **Core functionality** - Basic watch loop with interval
2. **Display** - Header, clear screen, output display
3. **CLI** - All command-line options
4. **Key controls** - Interactive keyboard input
5. **Advanced features** - Diff highlighting, colors, screenshots

### Platform Considerations

**Windows:**
- Use `cmd.exe /C` for shell execution
- Handle different line endings (CRLF vs LF)
- Console API differences

**Unix (Linux/macOS):**
- Use `sh -c` for shell execution
- Handle signals properly (SIGINT, SIGTERM)

### Error Handling

- Use `anyhow::Result` for application errors
- Provide meaningful error messages
- Follow exit code conventions from man page

## Testing Strategy

### Unit Tests
- Test individual functions in isolation
- Focus on edge cases (intervals, parsing)

### Integration Tests
- Test CLI argument combinations
- Test actual command execution
- Test keyboard input handling

### Manual Testing Commands

```bash
# Basic functionality
cargo run -- date

# With interval
cargo run -- -n 1 date

# With differences
cargo run -- -d ls -la

# Error exit
cargo run -- -e false

# Change exit
cargo run -- -g "cat somefile"
```

## Code Style

- Follow Rust standard style (rustfmt)
- Use descriptive variable names
- Document public APIs with doc comments
- Prefer `impl Trait` for return types where appropriate
- Use `?` operator for error propagation

## Resources

- [watch(1) man page](https://man7.org/linux/man-pages/man1/watch.1.html)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Conventional Commits](https://www.conventionalcommits.org/)
- [Semantic Versioning](https://semver.org/)
