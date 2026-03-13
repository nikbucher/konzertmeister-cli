# Requirements: km — Konzertmeister CLI

## Functional Requirements

| ID     | Title                  | User Story                                                                                                                                                                          | Priority | Status |
|--------|------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|----------|--------|
| FR-001 | API key config         | As an association admin, I want to configure my API key per association profile so that the CLI can authenticate with the Konzertmeister API.                                       | High     | Open   |
| FR-002 | Default association    | As an association admin, I want to set a default association profile so that I don't have to specify the association on every command.                                              | Medium   | Open   |
| FR-003 | List appointments      | As an association admin, I want to list upcoming appointments so that I can verify that all data is correct.                                                                        | High     | Open   |
| FR-004 | Filter appointments    | As an association admin, I want to filter appointments by date range, type, activation status, published status, and tags so that I can focus on relevant appointments.             | High     | Open   |
| FR-005 | Sorting                | As an association admin, I want to sort appointments by start date or deadline so that I can review them in a meaningful order.                                                     | Medium   | Open   |
| FR-006 | Create appointment     | As an association admin, I want to create an appointment from a template by providing a naive datetime so that the CLI resolves DST automatically using the association's timezone. | High     | Open   |
| FR-007 | Batch creation         | As an association admin, I want to create multiple appointments from a JSON file so that I can automate bulk event creation.                                                        | Medium   | Open   |
| FR-008 | JSON output            | As an association admin, I want the full API response as JSON output (default) so that I can pipe it to tools like jq or Miller for further processing.                             | High     | Open   |
| FR-009 | Table output           | As an association admin, I want a formatted table output with key fields (id, name, start, end, location, active, tags) so that I can quickly scan data in the terminal.            | Low      | Open   |
| FR-010 | Local datetime display | As an association admin, I want datetimes displayed in the appointment's local timezone by default so that I can verify times without manual UTC conversion.                        | High     | Open   |
| FR-011 | UTC output             | As an association admin, I want a `--utc` flag to display datetimes in raw UTC so that I can use the data in scripts that expect UTC.                                               | Low      | Open   |
| FR-012 | Creator mail config    | As an association admin, I want to configure the creator email per association profile so that appointment creation uses an authorized account.                                     | High     | Open   |
| FR-013 | Pagination             | As an association admin, I want the CLI to automatically fetch all pages of results (or let me specify `--page`) so that I see all matching appointments, not just the first 10.    | Medium   | Open   |

## Non-Functional Requirements

| ID      | Title          | Requirement                                                             | Category    | Priority | Status |
|---------|----------------|-------------------------------------------------------------------------|-------------|----------|--------|
| NFR-001 | Startup time   | CLI commands must start executing within 100ms on standard hardware.    | Performance | Medium   | Open   |
| NFR-002 | Cross-platform | CLI must compile and run on Linux, macOS, and Windows.                  | Portability | High     | Open   |
| NFR-003 | No runtime     | CLI must be a single static binary without runtime dependencies.        | Portability | High     | Open   |
| NFR-004 | Exit codes     | CLI must return exit code 0 on success and non-zero on error.           | Usability   | High     | Open   |
| NFR-005 | Error messages | Error messages must include the HTTP status code and API error message. | Usability   | High     | Open   |
| NFR-006 | Key security   | API keys must not be logged or included in error output.                | Security    | High     | Open   |

## Constraints

| ID    | Title         | Constraint                                                                            | Category  | Priority | Status |
|-------|---------------|---------------------------------------------------------------------------------------|-----------|----------|--------|
| C-001 | Language      | CLI must be implemented in Rust.                                                      | Technical | High     | Open   |
| C-002 | API version   | CLI targets the Konzertmeister API v4 endpoints.                                      | Technical | High     | Open   |
| C-003 | API endpoints | CLI is limited to the available M2M endpoints: list appointments, create appointment. | Technical | High     | Open   |
| C-004 | Open source   | Project should be publicly available on GitHub.                                       | Business  | High     | Open   |
