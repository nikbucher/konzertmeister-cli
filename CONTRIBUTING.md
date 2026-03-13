# Contributing

Contributions are welcome! Please open an issue or pull request on [GitHub](https://github.com/nikbucher/konzertmeister-cli).

## Domain Terminology

Use Konzertmeister's own naming (see [Konzertmeister API documentation](https://konzertmeister.app/en/help/konzertmeister-api)):

| Term            | Not                        | Notes                                               |
|-----------------|----------------------------|-----------------------------------------------------|
| **Association** | organization, club, Verein | The entity that owns appointments                   |
| **Appointment** | event, Termin              | A scheduled entry                                   |
| **Template**    | —                          | Appointment templates, referenced by external ID    |
| **API key**     | —                          | Per association, sent via `X-KM-ORG-API-KEY` header |

## Development Methodology

[AIUP](https://aiup.dev) — requirements-driven, iterative. Specs, code, and tests improve together in short cycles.

Phases: Inception → Elaboration → Construction → Transition.

## Code Style

- Follow [`.editorconfig`](.editorconfig) / [rustfmt](https://github.com/rust-lang/rustfmt) (`cargo fmt`)
- Run `cargo fmt --check` before committing
- Run `cargo clippy -- -D warnings` for lint checks

## Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/).

Format: `<type>[optional scope]: <description>`

Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`, `ci`, `build`

Examples:

- `feat: add appointment list command`
- `fix: handle missing API key gracefully`
- `docs: update vision with distribution strategy`
- `ci: add commitlint to PR checks`

## CI Checks

Make sure CI passes before submitting:

```sh
cargo fmt --check
cargo clippy -- -D warnings
cargo test
```
