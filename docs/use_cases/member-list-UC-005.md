# Use Case: List Members

## Overview

**Use Case ID:** UC-005  
**Use Case Name:** List Members  
**Primary Actor:** Association Admin  
**Goal:** View the association roster, including member details and data fields, to check current records.  
**Status:** Implemented

## Preconditions

- An association profile with an API key is configured.
- The admin can access the association.

## Main Success Scenario

1. Admin runs `km member list`.
2. System selects the default association profile.
3. System retrieves the association's members and their data fields.
4. System displays the members as JSON and exits successfully.

## Alternative Flows

### A1: Select Association

**Trigger:** Admin provides `--association` (step 1)  
**Flow:**

1. System selects the named association profile.
2. Use case continues at step 3.

### A2: Filter by Email

**Trigger:** Admin provides `--mail` (step 1)  
**Flow:**

1. System keeps members whose email matches the supplied address, ignoring case.
2. Use case continues at step 4.

### A3: Table Output

**Trigger:** Admin provides `--format table` (step 1)  
**Flow:**

1. System displays each member's name, email, and mobile phone in a table.
2. Use case ends.

### A4: No Members Match

**Trigger:** The roster or filtered result is empty (step 3)  
**Flow:**

1. System displays an empty list and exits successfully.
2. Use case ends.

### A5: Association Unavailable

**Trigger:** The selected association profile does not exist (step 2)  
**Flow:**

1. System reports the missing profile and available profile names.
2. Use case ends.

### A6: Service Error

**Trigger:** The roster cannot be retrieved (step 3)  
**Flow:**

1. System reports the service error and exits unsuccessfully.
2. Use case ends.

## Postconditions

### Success Postconditions

- The requested member list is displayed, including an empty list when nothing matches.
- No member record is changed.

### Failure Postconditions

- No member list is displayed.
- An error is reported and the command exits unsuccessfully.

## Business Rules

### BR-001: Member List Source

The CLI retrieves the complete association roster from `/api/v4/org/m2m/members`. Email filtering is applied to the returned list.

### BR-002: Output Format

JSON is the default and includes member data fields. Table output shows name, email, and mobile phone. A missing field displays as empty in the table.
