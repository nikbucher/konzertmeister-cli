# Use Case: List Appointments

## Overview

**Use Case ID:** UC-002  
**Use Case Name:** List Appointments  
**Primary Actor:** Association Admin  
**Goal:** List upcoming appointments so that the admin can verify that all data is correct  
**Status:** Implemented

## Preconditions

- At least one association profile is configured (see UC-001)
- A default association is set or `--association` is provided (see BR-003 in UC-001)

## Main Success Scenario

1. Admin runs `km list`.
2. System resolves the association profile (default or via `--association`).
3. System sends a request to the Konzertmeister API with `dateMode: UPCOMING`.
4. System receives the list of upcoming appointments.
5. System converts all datetimes to each appointment's local timezone.
6. System outputs the appointments as JSON to stdout.

## Alternative Flows

### A1: Filter by Date Range

**Trigger:** Admin provides `--from` and/or `--to` flags (step 1)
**Flow:**

1. System normalizes the date inputs (see BR-008).
2. System switches to `dateMode: FROM_DATE`.
3. System sends the request with `filterStart` and/or `filterEnd`.
4. Use case continues at step 4.

### A2: Filter by Type, Status, or Tags

**Trigger:** Admin provides `--type`, `--active`, `--cancelled`, `--published`, `--unpublished`, or `--tag` flags (step 1)
**Flow:**

1. System includes the corresponding filter parameters in the API request.
2. Use case continues at step 4.

### A3: Sort Results

**Trigger:** Admin provides `--sort startdate` or `--sort deadline` (step 1)
**Flow:**

1. System includes the `sortMode` parameter in the API request.
2. Use case continues at step 4.

### A4: Table Output

**Trigger:** Admin provides `--format table` (step 1)
**Flow:**

1. System formats the appointments as a table with columns: id, name, start, end, location, remind_deadline, status_deadline, active, tags.
2. System outputs the table to stdout.

### A5: UTC Output

**Trigger:** Admin provides `--utc` flag (step 1)
**Flow:**

1. System skips timezone conversion (step 5).
2. System outputs datetimes in raw ISO 8601 UTC format.

### A6: No Appointments Found

**Trigger:** API returns an empty list (step 4)
**Flow:**

1. System outputs an empty JSON array (`[]`) or an empty table.
2. Use case ends.

### A7: API Error

**Trigger:** API returns a non-200 response (step 4)
**Flow:**

1. System displays an error message including the HTTP status code and API error message (NFR-005).
2. System exits with a non-zero exit code (NFR-004).

### A8: Association Not Found

**Trigger:** Provided `--association` does not match any configured profile (step 2)
**Flow:**

1. System displays an error: "Profile '<name>' not found."
2. System lists available profiles.
3. System exits with a non-zero exit code.

### A9: No Association Configured

**Trigger:** No default set and no `--association` provided (step 2)
**Flow:**

1. System displays an error: "No association specified. Use --association or set a default with 'km config default'."
2. System lists available profiles (if any).
3. System exits with a non-zero exit code.

### A10: Multiple Pages of Results

**Trigger:** API returns a full page of 10 results (step 4)
**Flow:**

1. System automatically fetches the next page.
2. System repeats until a page with fewer than 10 results is returned.
3. System merges all pages into a single result set.
4. Use case continues at step 5.

### A11: Explicit Page Selection

**Trigger:** Admin provides `--page <n>` flag (step 1)
**Flow:**

1. System fetches only the requested page.
2. Use case continues at step 5.

## Postconditions

### Success Postconditions

- Appointments are displayed to stdout in the requested format (JSON or table)
- Datetimes are shown in local timezone (default) or UTC (if `--utc`)
- Exit code is 0

### Failure Postconditions

- An error message is displayed to stderr
- Exit code is non-zero
- No partial output is written to stdout

## Business Rules

### BR-004: Default Date Mode

When no `--from` or `--to` is provided, the CLI uses `dateMode: UPCOMING` (from today onwards).

### BR-005: Output Format Default

JSON is the default output format. Table output requires `--format table`.

### BR-006: Auto-Pagination

By default, the CLI fetches all pages and merges them. When `--page` is provided, only that single page is returned.

### BR-007: Timezone Display Default

Datetimes are displayed in each appointment's local timezone by default. The `--utc` flag overrides this to show raw UTC.

### BR-008: Date Input Normalization

`--from` and `--to` accept three formats:

- **Date** (`2026-01-01`): expanded to `2026-01-01T00:00:00Z` for `--from`, `2026-01-01T23:59:59Z` for `--to`.
- **Naive datetime** (`2026-01-01T14:00:00`): treated as UTC, `Z` is appended.
- **Full datetime** (`2026-01-01T14:00:00Z`, `2026-01-01T14:00:00+02:00`): passed through unchanged.
