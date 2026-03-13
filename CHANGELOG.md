# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
