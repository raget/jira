# Jira CLI

A simple command-line tool to create Jira issues in a Cloud instance. This tool is designed to be easily integrated with Cursor's AI capabilities.

## Features

- Create Jira issues with explicit arguments
- Supports Jira Cloud instances
- Uses Atlassian Document Format for descriptions
- Supports both Bug and Task issue types
- Sets High priority by default
- Provides clear error messages and output

## Prerequisites

- Rust and Cargo installed
- Jira Cloud instance
- Jira API token
- Jira account email

## Setup

1. Clone the repository
2. Create a `.env` file in the project root with your Jira credentials:
```env
JIRA_DOMAIN=your-domain.atlassian.net
JIRA_API_TOKEN=your-api-token
JIRA_USER=your-email@example.com
```

## Installation

```bash
cargo install --path .
```

## Usage

Create a new Jira issue using the create command:

```bash
jira create --project <PROJECT> --summary <SUMMARY> --description <DESCRIPTION> [--issue-type <TYPE>]
```

### Example

```bash
# Create a bug (default)
jira create --project SBT --summary "Fix login page bug" --description "The login page is not working properly"

# Create a task (case insensitive)
jira create --project SBT --summary "Implement dark mode" --description "Add dark mode support"
jira create --project SBT --summary "Implement dark mode" --description "Add dark mode support"
jira create --project SBT --summary "Implement dark mode" --description "Add dark mode support"
```

### Arguments

- `create`: Command to create a new issue
- `--project`: The key of the Jira project where the issue will be created (e.g., "SBT")
- `--summary`: The summary/title of the issue
- `--description`: The detailed description of the issue
- `--type`: Type of the issue (bug or task, case insensitive), defaults to bug

### Output

The tool will output:
- The URL of the created issue
- The issue key
- The issue summary

Example output:
```
Created issue: https://your-domain.atlassian.net/browse/SBT-123
Key: SBT-123
Summary: Fix login page bug
```

## Development

To run the tool during development:

```bash
cargo run -- create --project <PROJECT> --summary <SUMMARY> --description <DESCRIPTION> [--issue-type <TYPE>]
```

## Error Handling

The tool will display appropriate error messages if:
- Environment variables are missing
- API request fails
- Invalid project key is provided
- Other API-related errors occur

## Cursor Integration

Cursor can use this tool to create Jira issues directly from the editor. Here are some example prompts for Cursor:

1. Create a bug report:
```
Create a Jira bug for the login page issue with the description "Users cannot log in after password reset"
```

2. Create a task:
```
Create a Jira task for implementing dark mode in the application
```

The tool will automatically:
- Format the description in Atlassian Document Format
- Set the issue type (Bug or Task)
- Set the priority to High
- Provide the issue URL and key in the output

## Project Structure

```
.
├── src/
│   ├── main.rs      # Main CLI implementation
│   ├── commands.rs  # Command definitions
│   └── jira.rs      # Jira API client implementation
├── Cargo.toml       # Project dependencies
├── .env            # Environment variables (not in version control)
└── README.md       # This file
```

## Dependencies

- clap: Command-line argument parsing
- reqwest: HTTP client
- serde_json: JSON handling
- dotenv: Environment variable management
- chrono: Date and time handling

## License

MIT