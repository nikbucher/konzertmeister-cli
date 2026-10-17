# List available recipes
default:
    @just --list

# Format the code
fmt:
    cargo fmt

# Check formatting without changing files
fmt-check:
    cargo fmt --check

# Run clippy lints, failing on warnings
lint:
    cargo clippy -- -D warnings

# Run tests, optionally filtered (e.g. `just test uc001`)
test *filter:
    cargo test {{filter}}

# Run the same checks as CI
ci: fmt-check lint test

# Build a debug binary
build:
    cargo build

# Build an optimized release binary
release:
    cargo build --release

# Run the CLI with arguments (e.g. `just run appointment list`)
run *args:
    cargo run -- {{args}}

# Install the CLI from the local checkout
install:
    cargo install --path .

# Remove build artifacts
clean:
    cargo clean
