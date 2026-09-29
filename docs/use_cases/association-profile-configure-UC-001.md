# Use Case: Configure Association Profile

## Overview

**Use Case ID:** UC-001  
**Use Case Name:** Configure Association Profile  
**Primary Actor:** Association Admin  
**Goal:** Configure an association profile with an API key so that the CLI can authenticate with the Konzertmeister API  
**Status:** Implemented

## Preconditions

- The CLI (`km`) is installed
- The admin has an API key for the association (obtained from the Konzertmeister web UI)
- The admin knows the creator email if the profile will be used to create appointments.

## Main Success Scenario

1. Admin runs `km config set <profile-name>`.
2. System prompts for the API key (input is masked).
3. Admin enters the API key.
4. System prompts for the creator email.
5. Admin enters the creator email.
6. System stores the profile (name, API key, and optional creator email) in the configuration file (`<config-dir>/km/config.toml`).
7. System confirms that the profile has been saved.

## Alternative Flows

### A1: Values Provided via Flags

**Trigger:** Admin provides `--api-key` and/or `--creator-mail` flags (step 1)
**Flow:**

1. Admin runs `km config set <profile-name> --api-key "key" --creator-mail "mail"`.
2. System skips prompts for provided values, prompts only for missing ones.
3. Use case continues at step 6.

### A2: Profile Already Exists

**Trigger:** Profile name matches an existing profile (step 6)
**Flow:**

1. System overwrites the existing profile values.
2. System confirms that the profile has been saved.
3. Use case ends.

### A3: Set Default Profile

**Trigger:** Admin chooses to set a default association (step 1)
**Flow:**

1. Admin runs `km config default <profile-name>`.
2. System verifies that the profile exists.
3. System stores the profile name as the default in the configuration file.
4. System confirms that the default has been set.
5. Use case ends.

### A4: Default Profile Does Not Exist

**Trigger:** The requested default profile does not exist (step 1; A3 step 2)
**Flow:**

1. System displays an error: "Profile '<name>' not found."
2. Use case ends.

### A5: No API Key Entered

**Trigger:** Admin provides an empty API key (step 3)
**Flow:**

1. System displays an error: "API key must not be empty."
2. Use case ends.

### A6: No Creator Email Entered

**Trigger:** Admin provides an empty creator email (step 5)
**Flow:**

1. System warns that appointment creation will not be possible with this profile.
2. System stores the profile without a creator email.
3. Use case continues at step 7.

### A7: Config Directory Does Not Exist

**Trigger:** Configuration directory does not exist yet (step 6)
**Flow:**

1. System creates the configuration directory and file.
2. Use case continues at step 7.

### A8: Edit Configuration in Editor

**Trigger:** Admin chooses to edit the configuration file directly (step 1)
**Flow:**

1. Admin runs `km config edit`.
2. If the configuration file does not exist, system creates a template file.
3. System opens the configuration file in `$EDITOR`, `$VISUAL`, or `vim`.
4. Admin edits and saves the file.
5. Use case ends.

## Postconditions

### Success Postconditions

- The association profile is stored in `<config-dir>/km/config.toml`
- The API key and any supplied creator email are persisted and available for subsequent commands
- If a default was set, subsequent commands without `--association` use this profile

### Failure Postconditions

- No profile is created or modified
- The configuration file remains unchanged
- An error message is displayed to the admin

## Business Rules

### BR-001: API Key Security

API keys must not be logged, echoed to the terminal, or included in error output (NFR-006).

### BR-002: Profile Name Uniqueness

Each profile name must be unique within the configuration file. Setting a profile that already exists overwrites the API key.

### BR-003: Configuration Directory

The configuration file is stored at `<config-dir>/km/config.toml`, where `<config-dir>` is the platform-standard user configuration directory:

| Platform | `<config-dir>`                    |
|----------|-----------------------------------|
| Linux    | `$XDG_CONFIG_HOME` or `~/.config` |
| macOS    | `~/Library/Application Support`   |
| Windows  | `C:\Users\<user>\AppData\Roaming` |

The admin can check the resolved path with `km config path`.

### BR-004: Association Selection

If no default is set and `--association` is not provided, commands that require an association must display an error with the list of available profiles.
