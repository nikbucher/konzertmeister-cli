# Use Case: List Attendances

## Overview

**Use Case ID:** UC-009  
**Use Case Name:** List Attendances  
**Primary Actor:** Association Admin  
**Goal:** View recorded actual attendance for an appointment to review participation.  
**Status:** Implemented

## Preconditions

- An association profile with an API key is configured.
- The admin can access the appointment and knows its ID.

## Main Success Scenario

1. Admin obtains an appointment ID from `km appointment list` and runs `km attendance list APPOINTMENT_ID`.
2. System selects the association profile.
3. System retrieves recorded attendance for the appointment.
4. System displays the attendance records as JSON and exits successfully.

## Alternative Flows

### A1: Show Present Members

**Trigger:** Admin supplies `--attending` (step 1)  
**Flow:**

1. System keeps records marked as attending.
2. Use case continues at step 4.

### A2: Show Absent Members

**Trigger:** Admin supplies `--absent` (step 1)  
**Flow:**

1. System keeps records marked as absent.
2. Use case continues at step 4.

### A3: Table Output

**Trigger:** Admin supplies `--format table` (step 1)  
**Flow:**

1. System displays the person's name, email, attendance state, and comment in a table.
2. Use case ends.

### A4: No Attendance Matches

**Trigger:** No record matches the appointment and optional filter (step 3)  
**Flow:**

1. System displays an empty list and exits successfully.
2. Use case ends.

### A5: Conflicting Filters

**Trigger:** Admin supplies both `--attending` and `--absent` (step 1)  
**Flow:**

1. System reports that the two filters conflict.
2. Use case ends.

### A6: Appointment or Service Unavailable

**Trigger:** The appointment cannot be accessed or attendance cannot be retrieved (step 3)  
**Flow:**

1. System reports the service error and exits unsuccessfully.
2. Use case ends.

## Postconditions

### Success Postconditions

- The requested attendance records are displayed, including an empty list when none match.
- No attendance record is changed.

### Failure Postconditions

- No attendance list is displayed.
- An error is reported and the command exits unsuccessfully.

## Business Rules

### BR-001: Attendance Source

The CLI retrieves actual attendance from `/api/v2/attreal/m2m/{appId}`. The version 2 path differs from the appointment and reply paths.

### BR-002: Filter Semantics

`--attending` keeps only records with `attending: true`; `--absent` keeps only `attending: false`. Missing attendance values match neither filter. The two options are mutually exclusive. JSON is the default output.
