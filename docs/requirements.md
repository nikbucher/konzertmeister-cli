# Requirements: km — Konzertmeister CLI

## Functional Requirements

| ID     | Title                  | User Story                                                                                                                                                                          | Priority | Status |
|--------|------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|----------|--------|
| FR-001 | API key config         | As an association admin, I want to configure my API key per association profile so that the CLI can authenticate with the Konzertmeister API.                                       | High     | Implemented |
| FR-002 | Default association    | As an association admin, I want to set a default association profile so that I don't have to specify the association on every command.                                              | Medium   | Implemented |
| FR-003 | List appointments      | As an association admin, I want to list upcoming appointments so that I can verify that all data is correct.                                                                        | High     | Implemented |
| FR-004 | Filter appointments    | As an association admin, I want to filter appointments by date range, type, activation status, published status, and tags so that I can focus on relevant appointments.             | High     | Implemented |
| FR-005 | Sorting                | As an association admin, I want to sort appointments by start date or deadline so that I can review them in a meaningful order.                                                     | Medium   | Implemented |
| FR-006 | Create appointment     | As an association admin, I want to create an appointment from a template using a naive datetime so that the CLI applies my machine's timezone when the local time is unambiguous. | High     | Implemented |
| FR-007 | Batch creation         | As an association admin, I want to create multiple appointments from a JSON file so that I can automate bulk event creation.                                                        | Medium   | Open   |
| FR-008 | List JSON output       | As an association admin, I want list commands to output their results as JSON by default so that I can pipe them to tools like jq or Miller for further processing.                  | High     | Implemented |
| FR-009 | Appointment table      | As an association admin, I want an appointment table with key fields (id, name, start, end, location, deadlines, active, tags) so that I can quickly scan appointments in the terminal. | Low | Implemented |
| FR-010 | Local datetime display | As an association admin, I want datetimes displayed in the appointment's local timezone by default so that I can verify times without manual UTC conversion.                        | High     | Implemented |
| FR-011 | UTC output             | As an association admin, I want a `--utc` flag to display datetimes in raw UTC so that I can use the data in scripts that expect UTC.                                               | Low      | Implemented |
| FR-012 | Creator mail config    | As an association admin, I want to configure the creator email per association profile so that appointment creation uses an authorized account.                                     | High     | Implemented |
| FR-013 | Pagination             | As an association admin, I want the CLI to automatically fetch all pages of results (or let me specify `--page`) so that I see all matching appointments, not just the first 10.    | Medium   | Implemented |
| FR-014 | List members | As an association admin, I want to list and filter members by email so that I can inspect their basic data and data fields. | High | Implemented |
| FR-015 | Add member | As an association admin, I want to add a member by email and set data fields so that the association roster is current. | High | Implemented |
| FR-016 | Update member | As an association admin, I want to update a member and their data fields by email so that records stay current. | High | Implemented |
| FR-017 | List replies | As an association admin, I want to list and filter replies for an appointment so that I can plan participation. | High | Implemented |
| FR-018 | List attendances | As an association admin, I want to list and filter recorded attendance for an appointment so that I can review actual participation. | High | Implemented |
| FR-019 | Member table | As an association admin, I want a member table with name, email, and phone so that I can scan the association roster in the terminal. | Low | Implemented |
| FR-020 | Reply table | As an association admin, I want a reply table with name, email, reply, comment, and reply time so that I can scan responses in the terminal. | Low | Implemented |
| FR-021 | Attendance table | As an association admin, I want an attendance table with name, email, attendance state, and comment so that I can scan recorded participation in the terminal. | Low | Implemented |

## Non-Functional Requirements

| ID      | Title          | Requirement                                                             | Category    | Priority | Status |
|---------|----------------|-------------------------------------------------------------------------|-------------|----------|--------|
| NFR-001 | Startup time   | CLI commands must start executing within 100ms on standard hardware.    | Performance | Medium   | Open   |
| NFR-002 | Cross-platform | CLI must compile and run on Linux, macOS, and Windows.                  | Portability | High     | Open   |
| NFR-003 | No runtime     | CLI must be a single static binary without runtime dependencies.        | Portability | High     | Open   |
| NFR-004 | Exit codes     | CLI must return exit code 0 for success and help, 1 for runtime errors, and 2 for argument errors. | Usability | High | Implemented |
| NFR-005 | Error messages | Error messages must include the HTTP status code and API error message. | Usability   | High     | Open   |
| NFR-006 | Key security   | API keys must not be logged or included in error output.                | Security    | High     | Open   |

## Constraints

| ID    | Title         | Constraint                                                                            | Category  | Priority | Status |
|-------|---------------|---------------------------------------------------------------------------------------|-----------|----------|--------|
| C-001 | Language      | CLI must be implemented in Rust.                                                      | Technical | High     | Open   |
| C-002 | API version   | CLI targets the documented Konzertmeister M2M endpoints (v4 and attendance v2).                                      | Technical | High     | Open   |
| C-003 | API endpoints | CLI supports the documented M2M endpoints for appointments, members, replies, and attendance. | Technical | High     | Open   |
| C-004 | Open source   | Project should be publicly available on GitHub.                                       | Business  | High     | Open   |
