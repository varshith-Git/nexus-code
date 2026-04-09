# Claw Commands

This crate manages the "slash commands" used in the Claw Code REPL. It provides a registry of commands, logic for parsing user input, and a unified execution dispatch system.

## Architecture

Commands are defined as static specifications (`SlashCommandSpec`) that include their name, aliases, summary, and category. Parsing logic identifies these commands in user input and extracts their arguments into a structured `SlashCommand` enum.

---

## Command Reference

### Core Flow Commands
These commands manage the basic operation of the agent and current session status.

| Command | Usage | When to use | Why? |
| :--- | :--- | :--- | :--- |
| **`/help`** | `/help` | When you're unsure of available commands. | Provides a categorized list of all registered slash commands and their aliases. |
| **`/status`** | `/status` | To see the health of the current session. | Shows elapsed time, message count, and current provider/model state. |
| **`/compact`** | `/compact` | When a session becomes too long or "unfocused." | Triggers the compaction algorithm to summarize history into a context-efficient state, saving tokens and improving model focus. |
| **`/model`** | `/model [id]` | When you want to switch LLMs (e.g., from Sonnet to Gemini). | Allows real-time model swapping without losing session context. |
| **`/permissions`** | `/permissions [mode]` | When you need to elevate or restrict what the agent can do. | Switches between `read-only`, `workspace-write`, and `danger-full-access` security modes. |
| **`/cost`** | `/cost` | To check your spend for the current session. | Shows a summary of tokens used (input, output, cache) and estimated USD cost. |

### Workspace & Memory
Commands for inspecting and initializing the local workspace environment.

| Command | Usage | When to use | Why? |
| :--- | :--- | :--- | :--- |
| **`/init`** | `/init` | At the start of a new project. | Bootstraps a `CLAW.md` file with project-specific instructions that the agent will read on startup. |
| **`/config`** | `/config [section]` | When debugging why an agent is behaving a certain way. | Inspects merged configuration from user, project, and local sources (env, hooks, plugins). |
| **`/memory`** | `/memory` | To see why the agent has certain specific context. | Lists all "memory" files (like `CLAW.md`) currently loaded into the agent's system prompt. |
| **`/diff`** | `/diff` | Before committing changes. | Shows a standard Git diff of all pending workspace changes. |
| **`/teleport`** | `/teleport <target>` | When navigating large codebases. | Quickly jumps to a file or symbol using the LSP manager's search capabilities. |

### Sessions & Output
Commands for managing persistence and data export.

| Command | Usage | When to use | Why? |
| :--- | :--- | :--- | :--- |
| **`/clear`** | `/clear [--confirm]` | When starting a new task to avoid context pollution. | Safely wipes the current local session history and starts fresh. |
| **`/resume`** | `/resume <path>` | To continue a task from a specific session file. | Loads a previously saved JSON session into the active REPL. |
| **`/session`** | `/session [list|switch]` | To jump between different concurrent tasks. | Manages multiple named local sessions, allowing for easy task switching. |
| **`/export`** | `/export [path]` | When you want to save a conversation for sharing or debugging. | Saves the entire session history to a structured JSON file. |

### Git & Automation
Commands for high-level repository management and AI-driven workflows.

| Command | Usage | When to use | Why? |
| :--- | :--- | :--- | :--- |
| **`/commit`** | `/commit` | After the agent finishes a feature. | Automatically generates a semantic commit message based on changes and executes `git commit`. |
| **`/pr`** | `/pr [context]` | When ready to submit code. | Drafts a pull request description based on the conversation history. |
| **`/issue`** | `/issue [context]` | When a bug is found or a feature is requested. | Drafts a GitHub issue with relevant context from the workspace. |
| **`/bughunter`** | `/bughunter [scope]` | When searching for subtle regressions. | Triggers a specialized prompt flow to audit the codebase for architectural flaws. |
| **`/ultraplan`** | `/ultraplan [task]` | For complex, multi-step engineering tasks. | Uses a deep reasoning prompt to build an execution plan before the agent starts writing code. |

---

## Developer Guide

### Adding a New Command
1.  **Define the Variant**: Add the new command (and its arguments) to the `SlashCommand` enum in `src/lib.rs`.
2.  **Declare the Spec**: Add a `SlashCommandSpec` entry to the `SLASH_COMMAND_SPECS` slice. Ensure you provide a clear summary and appropriate `argument_hint`.
3.  **Implement Parsing**: Update the `SlashCommand::parse` function to handle the command string and its aliases.
4.  **Execute**: Implement the handler logic in the `claw-cli` crate or via specialized handlers in `runtime`.

## Current Status

- [x] Full command registry with categories
- [x] Robust argument parsing via `whitespace` splitting
- [x] Tab completion support for REPLs
- [x] Multi-command aliasing (e.g., `/plugin`, `/plugins`, `/marketplace`)

## Future Work

- [ ] **`/undo` / `/redo`**: Basic state manipulation for the local session.
- [ ] **Interactive Command Forms**: Support for complex arguments via TUI prompts.
- [ ] **Custom Plugin Commands**: Allowing plugins to inject commands directly into this registry.
