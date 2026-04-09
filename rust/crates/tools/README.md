# Claw Tools

`tools` is the standard library of workspace-aware tools that Claw Code provides to the AI agent. It defines the schemas, validation logic, and execution handlers for interacting with the file system, running commands, and searching the web.

## Architecture

This crate provides a centralized registry of tool definitions that are compatible with the LLM providers in the `api` crate.

### Key Tools

- **`ReadFile`**: Reads the contents of one or more files from the workspace.
- **`WriteFile`**: Creates or completely overwrites a file.
- **`EditFile`**: Applies surgical, line-based modifications to existing files using a structured patch format.
- **`Bash`**: Executes shell commands in the workspace environment (sandboxed by the runtime).
- **`ListDir` / `GlobSearch`**: Navigates the workspace and finds files based on patterns.
- **`GrepSearch`**: Performs fast, text-based searches across the entire project (powered by logic in `runtime`).
- **`TodoWrite`**: Manages a structured "Todo" list for the session to help the agent track multi-step progress.
- **`SearchWeb` / `FetchUrl`**: Allows the agent to gather external information and read web documentation.

## User Interface

Each tool is defined with:
1.  **Name**: Unique identifier (e.g., `bash`, `read_file`).
2.  **Description**: Clear instructions for the LLM on when and how to use the tool.
3.  **Input Schema**: A JSON Schema validating the arguments passed by the model.

## Current Status

- [x] Comprehensive file operation tools
- [x] Sandboxed Bash execution
- [x] Persistent Todo management
- [x] Structured Grep/Glob search
- [x] Basic web fetch support

## Future Work

- [ ] **Advanced Semantic Search**: Integration with a local vector database for better context discovery.
- [ ] **Richer Web Tools**: Support for specialized search engines and documentation scrapers.
- [ ] **Network Tools**: Basic ping/curl utilities for debugging local services.
