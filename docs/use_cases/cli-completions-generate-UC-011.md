# Use Case: Generate Shell Completions

## Overview

**Use Case ID:** UC-011  
**Use Case Name:** Generate Shell Completions  
**Primary Actor:** Association Admin  
**Goal:** Complete commands, options, and fixed option values with the Tab key in the shell.  
**Trigger:** Admin wants Tab completion for `km` in their shell  
**Status:** Implemented  
**Requirements:** [FR-023, NFR-004](../requirements.md)

## Preconditions

- The CLI is installed.

## Main Success Scenario

1. Admin runs `km completions <shell>` for bash, zsh, fish, elvish, or powershell.
2. System prints the completion script for that shell and exits successfully.
3. Admin saves the script where the shell loads completions (see README) and restarts the shell.

## Alternative Flows

### A1: Unsupported Shell

**Trigger:** Admin names a shell that is not supported (step 1).

1. System reports the supported shells as an argument error and exits with code 2 (see NFR-004).
2. No script is printed.

## Postconditions

### Success Postconditions

- The completion script is printed to standard output.
- No configuration or association data is read or changed.

### Failure Postconditions

- No script is printed.
- No configuration or association data is changed.

## Business Rules

### BR-001: Completion Scope

The script completes commands, subcommands, option names, and fixed option values such as `--format`, `--sort`, and `--reply`. Values that depend on the association, such as profile names for `--association`, are not completed.

### BR-002: No Association Required

The script is generated without an association profile, API key, or network access.
