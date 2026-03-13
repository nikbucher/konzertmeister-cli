# Use Case: Create Appointment

## Overview

**Use Case ID:** UC-003  
**Use Case Name:** Create Appointment  
**Primary Actor:** Association Admin  
**Goal:** Create an appointment from a template by providing a datetime so that the CLI resolves the timezone automatically  
**Status:** Implemented

## Preconditions

- At least one association profile is configured with API key and creator email (see UC-001)
- A default association is set or `--association` is provided
- The admin knows the external ID of the appointment template (from the Konzertmeister web UI)

## Main Success Scenario

1. Admin runs `km create --template <ext-id> --start "2026-06-15T19:30:00"`.
2. System resolves the association profile (default or via `--association`).
3. System reads the creator email from the profile.
4. System interprets the naive datetime as local machine timezone and converts it to a zoned datetime (see BR-009).
5. System sends a POST request to the Konzertmeister API with the template ID, zoned datetime, and creator email.
6. System receives the created appointment from the API.
7. System outputs the full API response as JSON to stdout.

## Alternative Flows

### A1: Zoned Datetime Provided

**Trigger:** Admin provides a datetime with timezone offset or `Z` suffix (step 1)
**Flow:**

1. System passes the datetime through unchanged (no timezone conversion).
2. Use case continues at step 5.

### A2: Override Name

**Trigger:** Admin provides `--name` flag (step 1)
**Flow:**

1. System includes the provided name in the API request.
2. Use case continues at step 5.

### A3: Override Description

**Trigger:** Admin provides `--description` flag (step 1)
**Flow:**

1. System includes the provided description in the API request.
2. Use case continues at step 5.

### A4: Dry Run

**Trigger:** Admin provides `--dry-run` flag (step 1)
**Flow:**

1. System resolves all parameters (profile, timezone, template ID, creator email).
2. System outputs the request payload that would be sent as JSON to stdout.
3. System does not send any request to the API.
4. Use case ends.

### A5: API Error

**Trigger:** API returns a non-200 response (step 6)
**Flow:**

1. System displays an error message including the HTTP status code and API error message (NFR-005).
2. System exits with a non-zero exit code (NFR-004).

### A6: Creator Email Not Configured

**Trigger:** The resolved profile has no creator email (step 3)
**Flow:**

1. System displays an error: "Creator email not configured for profile '<name>'. Run 'km config set <name>' to add it."
2. System exits with a non-zero exit code.

### A7: Association Not Found

**Trigger:** Provided `--association` does not match any configured profile (step 2)
**Flow:**

1. System displays an error: "Profile '<name>' not found."
2. System lists available profiles.
3. System exits with a non-zero exit code.

### A8: No Association Configured

**Trigger:** No default set and no `--association` provided (step 2)
**Flow:**

1. System displays an error: "No association specified. Use --association or set a default with 'km config default'."
2. System lists available profiles (if any).
3. System exits with a non-zero exit code.

### A9: Missing Required Flags

**Trigger:** Admin omits `--template` or `--start` (step 1)
**Flow:**

1. System displays an error indicating the missing required flag(s).
2. System exits with a non-zero exit code.

## Postconditions

### Success Postconditions

- The appointment is created in Konzertmeister
- The full API response (created appointment) is output as JSON to stdout
- Exit code is 0

### Failure Postconditions

- No appointment is created
- An error message is displayed to stderr
- Exit code is non-zero

## Business Rules

### BR-009: Naive Datetime Timezone Resolution

`--start` accepts three formats:

- **Naive datetime** (`2026-06-15T19:30:00`): interpreted as the local machine timezone, converted to a zoned datetime for the API.
- **UTC datetime** (`2026-06-15T17:30:00Z`): passed through unchanged.
- **Zoned datetime** (`2026-06-15T19:30:00+02:00`): passed through unchanged.

### BR-010: Template External ID

The `--template` value is the external ID of an appointment template as shown in the Konzertmeister web UI. The CLI passes it directly to the API without validation.

### BR-011: Creator Email Source

The creator email is always read from the association profile configuration. There is no command-line override.

### BR-012: Dry Run Output

`--dry-run` outputs the resolved request payload as JSON to stdout without sending a request. This allows the admin to verify timezone conversion and parameter resolution before creating an appointment.
