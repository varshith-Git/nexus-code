# Claw Tools

The `tools` crate defines the standard library of capabilities available to the Claw Code agent. It provides a structured registry for tool definitions and a unified dispatcher for their execution.

## Architecture

Tools are exposed as `ToolSpec` objects which define their name, human-readable description, JSON input schema, and required security permissions.

### Feature Areas

- **Filesystem**: High-performance, sandboxed file operations (`read_file`, `write_file`, `edit_file`).
- **Discovery**: Workspace-wide search utilities (`glob_search`, `grep_search`).
- **Execution**: Secure terminal command execution (`bash`).
- **Agentic**: Specialized tools for spawning child agents (`agent`) or loading expert knowledge (`skill`).
- **External**: Integration with web intelligence (`WebSearch`, `WebFetch`).

## Built-in Tool Reference

| Tool | Permission | Description |
| :--- | :--- | :--- |
| **`read_file`** | `read-only` | Reads text content from a specific workspace path (supports offset/limit). |
| **`write_file`** | `workspace-write` | Creates or overwrites a file with provided content. |
| **`edit_file`** | `workspace-write` | Performs precise, chunk-based edits to existing files using Target/Replacement blocks. |
| **`glob_search`** | `read-only` | Finds files matching a pattern (e.g., `**/*.rs`). |
| **`grep_search`** | `read-only` | Searches for text patterns across the workspace using `ripgrep` semantics. |
| **`bash`** | `danger-full-access` | Executes a shell command with optional background running and sandbox disabling. |
| **`agent`** | `danger-full-access` | Spawns a sub-agent to handle a sub-task independently with its own context. |
| **`skill`** | `read-only` | Loads expert-authored instructions (`SKILL.md`) for specialized domains. |
| **`WebSearch`** | `read-only` | Searches the public internet for documentation or current information. |

## Function-Level Documentation

| Function / Method | Description |
| :--- | :--- |
| **`mvp_tool_specs`** | Returns the list of all "minimum viable product" built-in tool specifications. |
| **`execute_tool`** | The main dispatcher that routes raw JSON tool calls to the correct internal handler. |
| **`execute_agent`** | Orchestrates the lifecycle of a sub-agent, including context synthesis and result reporting. |
| **`execute_skill`** | Resolves and loads local domain-specific instructions from the `.agents/skills` registry. |
| **`GlobalToolRegistry`** | Normalizes and manages both built-in and plugin-provided tools. |

## Developer Guide

### Implementing a New Tool
1.  **Define the Schema**: Use JSON Schema to define the input parameters the LLM should provide.
2.  **Assign Permissions**: Determine the minimum `PermissionMode` required (e.g., `read-only` for search tools).
3.  **Implement the Handler**: Add the logic to `execute_tool` (or a dedicated module).
4.  **Register as MVP**: Add the spec to the `mvp_tool_specs` vector in `lib.rs`.

## Current Status

- [x] **Sandboxed execution**: Bash and file tools respect security boundaries.
- [x] **Parallel Dispatch**: Integrated with the runtime for concurrent tool runs.
- [x] **Multi-agent capability**: Support for child runtimes via the `agent` tool.
- [x] **Symbol navigation**: (Via LSP integration hooks).

## Future Work

- [ ] **Interactive Tools**: Tools that can prompt the user for additional input during execution.
- [ ] **Visual Reasoning**: Support for tools that can inspect screenshots or UI layouts.
- [ ] **Native IDE Integration**: Tools that directly manipulate editor state (VS Code/JetBrains).
