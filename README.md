# km — Konzertmeister CLI

[![CI](https://github.com/nikbucher/konzertmeister-cli/actions/workflows/ci.yml/badge.svg)](https://github.com/nikbucher/konzertmeister-cli/actions/workflows/ci.yml)

A command-line tool for the [Konzertmeister](https://konzertmeister.app) API. Manage appointments and members, and view replies and attendance for your music association.

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

The config file is stored at `~/.config/km/config.toml` on Linux (or under `$XDG_CONFIG_HOME`), and at `~/Library/Application Support/km/config.toml` on macOS. Run `km config path` to see the exact location. On Unix, the file has restricted permissions (600).

## Usage

### List appointments

```sh
# Upcoming appointments (default output: JSON)
km appointment list

# Filter by date range
km appointment list --from 2026-01-01 --to 2026-06-30

# Only active, published appointments
km appointment list --active --published

# Filter by tag
km appointment list --tag rehearsal

# Filter by appointment type ID
km appointment list --type 1 --type 5

# Sort by deadline instead of start date
km appointment list --sort deadline

# Table output
km appointment list --format table

# Show times in UTC
km appointment list --format table --utc
```

JSON output is designed to be pipeable, e.g. with [jq](https://jqlang.github.io/jq/):

```sh
km appointment list | jq '.[].name'
```

### Create an appointment

Appointments are created from templates. You can find template external IDs in the Konzertmeister web UI.

```sh
# Create from a template
km appointment create --template tmpl-abc --start 2026-06-15T19:30

# With a custom name and description
km appointment create --template tmpl-abc --start 2026-06-15T19:30 --name "Summer Concert" --description "Annual open-air event"

# Preview the request without sending it
km appointment create --template tmpl-abc --start 2026-06-15T19:30 --dry-run
```

Naive datetimes (without timezone offset) are interpreted as your local timezone. You can also provide an explicit offset:

```sh
km appointment create --template tmpl-abc --start "2026-06-15T19:30:00+02:00"
```

### List members

```sh
km member list --format table
km member list --mail person@example.com
```

The default JSON output includes member data fields. Email matching ignores case. An empty result is a successful empty list.

### Add or update a member

```sh
km member add --mail person@example.com --firstname Alex --prop-string section=brass
km member update --mail person@example.com --mobile-phone 123 --prop-bool active=true
km member add --mail person@example.com --prop-number score=1.5 --dry-run
```

`--mail` is required. Updates also need at least one changed detail or data field. Member data fields use their external IDs: `--prop-string EXT=VALUE`, `--prop-number EXT=VALUE`, `--prop-date EXT=VALUE`, and `--prop-bool EXT=true|false`. Repeat a flag to set multiple fields. Date values accept a local datetime or an explicit offset. The API input has no dedicated select value field. `--dry-run` prints the request JSON without sending it. Successful live requests report to stderr and exit with code 0; the API returns no body.

### List replies

```sh
km reply list 123
km reply list 123 --reply positive --format table
```

Use an appointment ID from `km appointment list`. Reply filters are `positive`, `maybe`, `negative`, and `unanswered`. The default output is JSON.

### List attendance

```sh
km attendance list 123
km attendance list 123 --attending --format table
km attendance list 123 --absent
```

`--attending` and `--absent` are mutually exclusive. Recorded attendance uses the API's v2 endpoint. The default output is JSON.

### Exit codes

All commands use these exit codes:

| Code | Meaning |
|------|---------|
| `0` | Command succeeded or help was displayed |
| `1` | Runtime error, such as a missing association profile or API failure |
| `2` | Argument error, such as an unknown option or missing required argument |

### Manage configuration

```sh
km config path      # Print config file location
km config edit      # Open config in $EDITOR
```

### Use a specific profile

Override the default profile for any command with `--association`:

```sh
km appointment list --association other-association
km appointment create --association other-association --template tmpl-abc --start 2026-06-15T19:30
```

## Releasing

Releases are automated via GitHub Actions. To create a new release:

```sh
git tag -a v0.2.0 -m "v0.2.0"
git push origin v0.2.0
```

This triggers a build for all supported platforms and creates a GitHub Release with the binaries attached.

## Documentation

- [Vision](docs/vision.md) — project goals and scope
- [Requirements](docs/requirements.md) — functional and non-functional requirements
- [Use cases](docs/use_cases/use_cases.md) — feature overview, specifications, and test traceability
- [API spec](docs/openapi.json) — OpenAPI 3.1 (source: `https://rest.konzertmeister.app/v3/api-docs/m2m`)

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for domain terminology, coding conventions, and commit message guidelines.

## License

[MIT](LICENSE)
