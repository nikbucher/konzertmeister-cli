# Use Case: Show Version

## Overview

**Use Case ID:** UC-010  
**Use Case Name:** Show Version  
**Primary Actor:** Association Admin  
**Goal:** Check the installed CLI version to confirm which release is running.  
**Trigger:** Admin wants to know which release is installed, e.g. before reporting an issue or after upgrading  
**Status:** Implemented  
**Requirements:** [FR-022, NFR-004](../requirements.md)

## Preconditions

- The CLI is installed.

## Main Success Scenario

1. Admin runs `km --version` or `km -V`.
2. System displays the CLI name and the installed version and exits successfully.

## Alternative Flows

_None — showing the version needs no association profile, input, or service, so no step can fail or branch._

## Postconditions

### Success Postconditions

- The installed version is displayed.
- No configuration or association data is read or changed.

### Failure Postconditions

- No configuration or association data is changed.

## Business Rules

### BR-001: Version Source and Format

The version is the release version the binary was built with. Output is a single line `km <version>` on stdout, e.g. `km 0.2.0`.

### BR-002: No Association Required

The version is shown without an association profile, API key, or network access. The command exits with code 0 (see NFR-004).
