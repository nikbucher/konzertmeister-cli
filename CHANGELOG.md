# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2026-09-29

### Added

- `appointment list` / `appointment create` — appointment commands grouped under `appointment`
- `member list` — list association members with data fields, filter by `--mail`
- `member add` / `member update` — add or update members, including data fields via `--prop-string`, `--prop-number`, `--prop-date`, `--prop-bool`, with `--dry-run` preview
- `reply list` — list replies to an appointment, filter by `--reply positive|maybe|negative|unanswered`
- `attendance list` — list recorded attendance for an appointment, filter by `--attending` or `--absent`

### Removed

- **BREAKING:** top-level `list` and `create` commands — use `km appointment list` and `km appointment create` instead

## [0.1.0] - 2026-03-12

### Added

- `config set` — create and manage association profiles with API key and creator email
- `config default` — set a default profile for all commands
- `config edit` — open config file in `$EDITOR`
- `config path` — print config file location
- `list` — list appointments with filters (date range, type, active/cancelled, published/unpublished, tags)
- `list` — auto-pagination, sorting by start date or deadline, JSON and table output
- `create` — create appointments from templates with `--dry-run` preview
- Local timezone handling for naive datetime inputs
- Secure config storage with restricted file permissions (600)
