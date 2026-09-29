# Vision: km — Konzertmeister CLI

## Goal

Make the documented [Konzertmeister M2M API](https://rest.konzertmeister.app/v3/api-docs/m2m) usable through a command-line interface. Anyone with an association API key should be able to use the API's capabilities directly in the terminal and automate them with scripts or agents.

## Users

- **Association Admin:** Manages appointments and members of an association, interactively or through scripts and agents.

## Product Priorities

- **API coverage:** Expose every documented M2M operation, including appointments, members, replies, and attendance, through CLI commands.
- **Clear, consistent commands:** Use Konzertmeister's domain terms and provide clear commands, input validation, and errors.
- **Automation:** Provide machine-readable JSON output and predictable exit codes so commands work well with other tools.
- **Association context:** Make it straightforward to configure and switch between associations without API keys appearing in output, logs, or error messages.

The API defines the core scope. CLI conveniences such as batch creation can be added where they make common workflows easier.

## Success Criteria

- Every documented M2M operation is accessible through the CLI.
- List commands offer JSON output by default and readable tables for interactive use.
- Action commands provide understandable feedback about their result.
- Every command returns documented exit codes.
- The project is available as open source on GitHub.
