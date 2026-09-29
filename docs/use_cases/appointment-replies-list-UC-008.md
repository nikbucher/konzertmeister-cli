# Use Case: List Replies

## Overview

**Use Case ID:** UC-008  
**Use Case Name:** List Replies  
**Primary Actor:** Association Admin  
**Goal:** View members' replies to an appointment to plan participation.  
**Status:** Implemented

## Preconditions

- An association profile with an API key is configured.
- The admin can access the appointment and knows its ID.

## Main Success Scenario

1. Admin obtains an appointment ID from `km appointment list` and runs `km reply list APPOINTMENT_ID`.
2. System selects the association profile.
3. System retrieves replies for the appointment.
4. System displays the replies as JSON and exits successfully.

## Alternative Flows

### A1: Filter by Reply

**Trigger:** Admin supplies `--reply` (step 1)  
**Flow:**

1. System keeps only replies with the selected value.
2. Use case continues at step 4.

### A2: Table Output

**Trigger:** Admin supplies `--format table` (step 1)  
**Flow:**

1. System displays the person's name, email, reply, comment, and reply time in a table.
2. Use case ends.

### A3: No Replies Match

**Trigger:** No reply matches the appointment and optional filter (step 3)  
**Flow:**

1. System displays an empty list and exits successfully.
2. Use case ends.

### A4: Unknown Reply Value

**Trigger:** The service returns a reply value the CLI does not recognize (step 3)  
**Flow:**

1. System displays the reply as unknown and continues showing the other replies.
2. Use case continues at step 4.

### A5: Appointment or Association Unavailable

**Trigger:** The appointment cannot be accessed or the selected association profile does not exist (step 2)  
**Flow:**

1. System reports the missing appointment or profile.
2. Use case ends.

### A6: Service Error

**Trigger:** The replies cannot be retrieved (step 3)  
**Flow:**

1. System reports the service error and exits unsuccessfully.
2. Use case ends.

## Postconditions

### Success Postconditions

- The requested replies are displayed, including an empty list when none match.
- No reply is changed.

### Failure Postconditions

- No reply list is displayed.
- An error is reported and the command exits unsuccessfully.

## Business Rules

### BR-001: Reply Values and Source

The CLI retrieves `/api/v4/att/m2m/{appId}`. Recognized values are `POSITIVE`, `MAYBE`, `NEGATIVE`, and `UNANSWERED`. Unknown service values are shown as `UNKNOWN`; the original unknown text cannot be recovered from the typed response.

### BR-002: Filtering and Display

`--reply` accepts one recognized value in lowercase. Filtering happens after retrieval. JSON is the default output; table output converts reply time to the machine's local timezone when possible.
