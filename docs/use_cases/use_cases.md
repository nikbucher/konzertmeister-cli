# Use Cases — Konzertmeister CLI

Filenames start with the feature name and end with the stable use case ID, so the specifications sort by feature. The ID inside each document remains unchanged. Use `docs/use_cases/*-UC-*.md` when running the AIUP validator; the template's `docs/use_cases/UC-*.md` glob does not match these files.

## Actors

| Actor | Description |
|-------|-------------|
| Association Admin | User with an API key for their association, managing appointments via CLI |

## Use Case Overview

| Feature | Use Case | Actor | Status | ID |
|---------|----------|-------|--------|----|
| Association | [Configure Association Profile](association-profile-configure-UC-001.md) | Association Admin | Implemented | UC-001 |
| Appointment | Batch Create Appointments | Association Admin | Planned | UC-004 |
| Appointment | [Create Appointment](appointment-create-UC-003.md) | Association Admin | Implemented | UC-003 |
| Appointment | [List Appointments](appointment-list-UC-002.md) | Association Admin | Implemented | UC-002 |
| Appointment | [List Attendances](appointment-attendances-list-UC-009.md) | Association Admin | Implemented | UC-009 |
| Appointment | [List Replies](appointment-replies-list-UC-008.md) | Association Admin | Implemented | UC-008 |
| Member | [Add Member](member-add-UC-006.md) | Association Admin | Implemented | UC-006 |
| Member | [List Members](member-list-UC-005.md) | Association Admin | Implemented | UC-005 |
| Member | [Update Member](member-update-UC-007.md) | Association Admin | Implemented | UC-007 |

The [use case diagram](../use_cases.puml) shows actors and use cases. The [test traceability matrix](traceability.md) records existing test coverage.
