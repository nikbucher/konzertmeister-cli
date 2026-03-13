# Vision: km — Konzertmeister CLI

## Goal

A command-line tool that makes the Konzertmeister API accessible, allowing users to efficiently list and verify appointments as well as automate their creation.

## Users

- **Association Admin**: Uses the CLI to list and quickly verify appointment data, and to automate appointment creation (manually, via scripts, or in the future via an agent).
- **Open-Source Community**: Any Konzertmeister user with an API key who wants to use the API from the command line.

## Core Features

- **List appointments**: Query upcoming appointments with filters (date, type, status, tags) and output as JSON
- **Create appointment**: Create individual appointments from templates via the API
- **Batch creation**: Create multiple appointments at once
- **API key configuration**: Simple configuration of the API key per association

## Key Workflows

1. Configure API key and connect to the Konzertmeister API
2. List, filter, and verify appointment data
3. Create appointments individually or in batch from templates

## Success Criteria

- All available API endpoints are usable via the CLI
- Output is available as JSON and pipeable for further processing
- The CLI is available as an open-source project on GitHub
