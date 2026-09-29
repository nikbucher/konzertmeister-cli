# Use Case: Add Member

## Overview

**Use Case ID:** UC-006  
**Use Case Name:** Add Member  
**Primary Actor:** Association Admin  
**Goal:** Add a person to the association by email and optionally set member details and data fields.  
**Status:** Implemented

## Preconditions

- An association profile with an API key is configured for a live request.
- The admin is authorized to add members to the association.

## Main Success Scenario

1. Admin runs `km member add --mail person@example.com` with optional details and data fields.
2. System checks the supplied data field values and selects the association profile.
3. System requests that the person be added to the association.
4. System reports success to the admin and exits successfully.

## Alternative Flows

### A1: Person Has No Account

**Trigger:** The supplied email does not belong to an existing account (step 3)  
**Flow:**

1. The service invites the person to create an account and adds them to the association.
2. Use case continues at step 4.

### A2: Set Data Fields

**Trigger:** Admin supplies one or more `--prop-*` values (step 1)  
**Flow:**

1. System includes each valid value with its data field's external ID in the request.
2. Use case continues at step 3.

### A3: Preview Request

**Trigger:** Admin provides `--dry-run` (step 1)  
**Flow:**

1. System displays the resolved request as JSON.
2. No member is added or invited.
3. Use case ends.

### A4: Invalid Input

**Trigger:** Email is missing or a data field value is invalid (step 2)  
**Flow:**

1. System reports the missing or invalid value.
2. Use case ends.

### A5: Association Unavailable

**Trigger:** The selected association profile does not exist (step 2)  
**Flow:**

1. System reports the missing profile and available profile names.
2. Use case ends.

### A6: Service Error

**Trigger:** The person cannot be added (step 3)  
**Flow:**

1. System reports the service error and exits unsuccessfully.
2. Use case ends.

## Postconditions

### Success Postconditions

- The person is associated with the association, or receives an invitation if no account exists.
- The CLI confirms success without expecting response content.

### Failure Postconditions

- The CLI does not confirm an addition.
- An error is reported and the command exits unsuccessfully.

## Business Rules

### BR-001: Required Email and Invitation

`--mail` is required. The service sends an invitation when the address has no existing account. The CLI uses `/api/v4/org/m2m/addmember` for live requests.

### BR-002: Data Fields

String, number, date-time, and Boolean data fields are supplied through repeatable `--prop-string`, `--prop-number`, `--prop-date`, and `--prop-bool` options in `EXT=VALUE` form. `EXT` is the external ID. Numbers must be finite; Boolean values are `true` or `false`. Date-times without an offset are interpreted in the machine's local timezone.

### BR-003: Preview and Success Output

`--dry-run` writes the request JSON to stdout without sending it. A successful live request writes a confirmation to stderr and exits with code 0; the service returns no response body. The OpenAPI input has no dedicated select value field.
