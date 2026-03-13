# km — Konzertmeister CLI

[![CI](https://github.com/nikbucher/konzertmeister-cli/actions/workflows/ci.yml/badge.svg)](https://github.com/nikbucher/konzertmeister-cli/actions/workflows/ci.yml)

A command-line tool for the [Konzertmeister](https://konzertmeister.app) API. List and create appointments for your music association.

## Prerequisites

- A [Konzertmeister](https://konzertmeister.app) account with API access
- Your association's API key (see [Konzertmeister API documentation](https://konzertmeister.app/en/help/konzertmeister-api))

## Installation

### Homebrew (macOS & Linux)

```sh
brew install nikbucher/tap/konzertmeister-cli
```

### Pre-built binaries

Download the latest binary for your platform from [GitHub Releases](https://github.com/nikbucher/konzertmeister-cli/releases):

| Platform | Architecture  | Download                              |
|----------|---------------|---------------------------------------|
| Linux    | x86_64        | `km-x86_64-unknown-linux-gnu.tar.gz`  |
| Linux    | aarch64       | `km-aarch64-unknown-linux-gnu.tar.gz` |
| macOS    | Intel         | `km-x86_64-apple-darwin.tar.gz`       |
| macOS    | Apple Silicon | `km-aarch64-apple-darwin.tar.gz`      |
| Windows  | x86_64        | `km-x86_64-pc-windows-msvc.zip`       |
| Windows  | aarch64       | `km-aarch64-pc-windows-msvc.zip`      |

Extract the archive and place the `km` binary somewhere on your `PATH`.

### Build from source

Requires [Rust](https://rustup.rs/) (edition 2024):

```sh
cargo install --git https://github.com/nikbucher/konzertmeister-cli
```

## Setup

Configure your association profile. You will be prompted for the API key and creator email if not provided as flags:

```sh
km config set my-association
```

Or pass them directly:

```sh
km config set my-association --api-key YOUR_API_KEY --creator-mail admin@example.com
```

If you manage multiple associations, add more profiles and set a default:

```sh
km config set other-association
km config default my-association
```

The config file is stored at `~/.config/km/config.toml` (Linux/macOS) with restricted file permissions (600).

## Usage

### List appointments

```sh
# Upcoming appointments (default output: JSON)
km list

# Filter by date range
km list --from 2026-01-01 --to 2026-06-30

# Only active, published appointments
km list --active --published

# Filter by tag
km list --tag rehearsal

# Filter by appointment type ID
km list --type 1 --type 5

# Sort by deadline instead of start date
km list --sort deadline

# Table output
km list --format table

# Show times in UTC
km list --format table --utc
```

JSON output is designed to be pipeable, e.g. with [jq](https://jqlang.github.io/jq/):

```sh
km list | jq '.[].name'
```

### Create an appointment

Appointments are created from templates. You can find template external IDs in the Konzertmeister web UI.

```sh
# Create from a template
km create --template tmpl-abc --start 2026-06-15T19:30

# With a custom name and description
km create --template tmpl-abc --start 2026-06-15T19:30 --name "Summer Concert" --description "Annual open-air event"

# Preview the request without sending it
km create --template tmpl-abc --start 2026-06-15T19:30 --dry-run
```

Naive datetimes (without timezone offset) are interpreted as your local timezone. You can also provide an explicit offset:

```sh
km create --template tmpl-abc --start "2026-06-15T19:30:00+02:00"
```

### Manage configuration

```sh
km config path      # Print config file location
km config edit      # Open config in $EDITOR
```

### Use a specific profile

Override the default profile for any command with `--association`:

```sh
km list --association other-association
km create --association other-association --template tmpl-abc --start 2026-06-15T19:30
```

## Releasing

Releases are automated via GitHub Actions. To create a new release:

```sh
git tag -a v0.1.0 -m "Initial release"
git push origin v0.1.0
```

This triggers a build for all supported platforms and creates a GitHub Release with the binaries attached.

## Documentation

- [Vision](docs/vision.md) — project goals and scope
- [Requirements](docs/requirements.md) — functional and non-functional requirements
- [API spec](docs/openapi.json) — OpenAPI 3.0 (source: `https://rest.konzertmeister.app/v3/api-docs/m2m`)

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for domain terminology, coding conventions, and commit message guidelines.

## License

[MIT](LICENSE)
