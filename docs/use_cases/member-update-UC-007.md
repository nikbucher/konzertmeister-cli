# Use Case: Update Member

## Overview

**Use Case ID:** UC-007  
**Use Case Name:** Update Member  
**Primary Actor:** Association Admin  
**Goal:** Change a member's details or data fields using their email address.  
**Status:** Implemented

## Preconditions

- An association profile with an API key is configured for a live request.
- The admin is authorized to update members of the association.
- The admin knows the member's email address.

## Main Success Scenario

1. Admin runs `km member update --mail person@example.com` with at least one changed detail or data field.
2. System checks that a change was supplied and validates its value.
3. System selects the association profile and requests the member update.
4. System confirms the update and exits successfully.

## Alternative Flows

### A1: Change Data Fields

**Trigger:** Admin supplies one or more `--prop-*` values (step 1)  
**Flow:**

1. System includes each valid value with its data field's external ID.
2. Use case continues at step 3.

### A2: Preview Request

**Trigger:** Admin provides `--dry-run` (step 1)  
**Flow:**

1. System displays the resolved request as JSON.
2. Member data remains unchanged.
3. Use case ends.

### A3: Nothing to Update

**Trigger:** Admin supplies only `--mail` (step 2)  
**Flow:**

1. System reports that a name, mobile phone, or data field must be supplied.
2. Use case ends.

### A4: Name Restricted by Account State

**Trigger:** Admin changes a registered account's first or last name (step 3)  
**Flow:**

1. The service rejects the restricted change and the system reports the error.
2. Use case ends.

### A5: Member or Association Unavailable

**Trigger:** The member cannot be found or the selected association profile does not exist (step 3)  
**Flow:**

1. System reports the missing member or profile.
2. Use case ends.

### A6: Invalid Data Field or Service Error

**Trigger:** A data field value is invalid or the update is rejected (step 2)  
**Flow:**

1. System reports the invalid value or service error.
2. Use case ends.

## Postconditions

### Success Postconditions

- The supplied details or data fields are updated for the member.
- The CLI confirms success without expecting response content.

### Failure Postconditions

- The CLI does not confirm an update.
- An error is reported and the command exits unsuccessfully.

## Business Rules

### BR-001: Member Identity and Change Requirement

`--mail` identifies the member. At least one of `--firstname`, `--lastname`, `--mobile-phone`, or a `--prop-*` value must also be supplied. The CLI uses `/api/v4/org/m2m/updatemember` for live requests.

### BR-002: Account Name Restriction

The service permits first and last name changes only while the member account has not signed up. The CLI sends the requested change and reports any rejection.

### BR-003: Data Fields and Preview

Data fields follow UC-006 BR-002. `--dry-run` writes the request JSON to stdout without sending it. A successful live request writes a confirmation to stderr and exits with code 0.
