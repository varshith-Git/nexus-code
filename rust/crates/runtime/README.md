# Claw Runtime

The `runtime` crate is the core engine of Claw Code. It orchestrates the entire agentic loop, manages session persistence, enforces security permissions, and handles high-performance parallel tool execution.

## Architecture

The runtime is designed for high-concurrency and safe workspace interaction. It operates on a sophisticated "Turn" model.

### The Conversation Loop (`ConversationRuntime`)

Every interaction follows a multi-step loop:
1.  **Prompt Building**: Concatenates system prompt, workspace context (from `lsp`), and session history.
2.  **Model Inference**: Streams responses from the `api` crate.
3.  **Parallel Tool Execution**: If the agent calls multiple tools, the runtime processes them in a **3-phase model**:
    -   **Phase 1: Sync Auth**: All tool calls are checked against the `PermissionPolicy` and "Pre-Tool" hooks.
    -   **Phase 2: Parallel Scoped Execution**: Authorized tools are executed concurrently using `std::thread::scope`, dramatically reducing latency for multi-file operations.
    -   **Phase 3: Sync Merge**: Results are collected, "Post-Tool" hooks are run, and the conversation state is updated.

### Key Components

- **`Session`**: The source of truth for a conversation, including all messages and their associated token usage.
- **`UsageTracker`**: Provides real-time and cumulative accounting of tokens and estimated USD costs across different providers.
- **`PermissionPolicy`**: A fine-grained security layer that controls access to tools (Read, Write, Execute) based on user configuration.
- **`Sandbox`**: Provides isolation for file operations and shell commands to prevent accidental workspace damage.
- **`McpServerManager`**: Implementation of the **Model Context Protocol (MCP)**, allowing Claw to connect to external tool providers (like a Postgres MCP or a Google Search MCP).

## Developer Guide

### Working with Tools
Built-in tools are defined in the `tools` crate but executed through the `runtime`'s `ToolExecutor` interface. This ensures that all tools—whether built-in or from plugins—go through the same permission and hook checks.

### Persistence
Sessions are persisted as JSON files in `~/.claw/sessions/`. The runtime handles automatic loading and saving to ensure no work is lost between restarts.

## Current Status

- [x] Parallel tool execution (3-phase model)
- [x] Model Context Protocol (MCP) support (Stdio/SSE)
- [x] Granular permission system
- [x] Sandboxed file and bash operations
- [x] Token usage and cost accounting
- [x] History compaction logic

## Future Work

- [ ] **Context Fetching**: Speculative pre-loading of files into the model context based on user prompts.
- [ ] **Memory Management**: Advanced compaction and summarization strategies for extremely long sessions.
- [ ] **Sub-Agent Orchestration**: Spawning child runtimes for highly complex tasks.
