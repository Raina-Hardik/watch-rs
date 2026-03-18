# justfile for watch-rs
# Cross-platform build and development tasks

set shell := ["bash", "-uc"]
set windows-shell := ["powershell.exe", "-NoProfile", "-Command"]

# Default recipe - show help
default:
    @just --list

# Format code (Rust standard formatting)
[unix]
format:
    cargo fmt --all

[windows]
format:
    cargo fmt --all

# Lint code with clippy and apply fixes where possible
[unix]
lint:
    cargo clippy --all-targets --all-features -- -D warnings

[windows]
lint:
    cargo clippy --all-targets --all-features -- -D warnings

# Lint with automatic fixes
[unix]
lint-fix:
    cargo clippy --all-targets --all-features --fix --allow-dirty --allow-staged

[windows]
lint-fix:
    cargo clippy --all-targets --all-features --fix --allow-dirty --allow-staged

# Compile release binary
[unix]
compile:
    @echo "Setting up Rust toolchain..."
    rustup update stable
    cargo build --release
    @echo "Binary available at: target/release/watch-rs"

[windows]
compile:
    Write-Host "Setting up Rust toolchain..."
    rustup update stable
    cargo build --release
    Write-Host "Binary available at: target/release/watch-rs.exe"

# Compile with all features and debug info
[unix]
compile-debug:
    cargo build --all-features
    @echo "Debug binary available at: target/debug/watch-rs"

[windows]
compile-debug:
    cargo build --all-features
    Write-Host "Debug binary available at: target/debug/watch-rs.exe"

# Run tests
[unix]
test:
    cargo test --all-features --verbose

[windows]
test:
    cargo test --all-features --verbose

# Clean build artifacts
[unix]
clean:
    cargo clean
    @echo "Build artifacts cleaned"

[windows]
clean:
    cargo clean
    Write-Host "Build artifacts cleaned"

# Full development workflow (format, lint-fix, compile, test)
[unix]
dev: format lint-fix compile test
    @echo "✓ Development workflow complete"

[windows]
dev: format lint-fix compile test
    Write-Host "✓ Development workflow complete"

# Check everything without modifying (format, lint, compile, test)
[unix]
check: 
    cargo fmt --all -- --check
    cargo clippy --all-targets --all-features -- -D warnings
    cargo test --all-features

[windows]
check:
    cargo fmt --all -- --check
    cargo clippy --all-targets --all-features -- -D warnings
    cargo test --all-features

# Build release binary for current platform
[unix]
release:
    cargo build --release --all-features
    @echo "Release binary: target/release/watch-rs"
    @echo "Size: $(du -h target/release/watch-rs | cut -f1)"

[windows]
release:
    cargo build --release --all-features
    Write-Host "Release binary: target/release/watch-rs.exe"
    $size = (Get-Item target/release/watch-rs.exe).Length
    Write-Host "Size: $([Math]::Round($size / 1MB, 2)) MB"

# Build for specific target (usage: just build-target x86_64-pc-windows-msvc)
build-target target:
    rustup target add {{target}}
    cargo build --release --target {{target}}
    @echo "Built for {{target}}"

# Install locally for testing
[unix]
install:
    cargo install --path .
    @echo "watch-rs installed locally"
    @echo "Try: watch-rs --version"

[windows]
install:
    cargo install --path .
    Write-Host "watch-rs installed locally"
    Write-Host "Try: watch-rs --version"

# Show version
[unix]
version:
    @cargo run -- --version

[windows]
version:
    cargo run -- --version
