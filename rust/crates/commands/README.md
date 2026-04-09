# Claw Commands

This crate manages the "slash commands" used in the Claw Code REPL. It provides a registry of commands, logic for parsing user input, and a unified execution dispatcher for complex workflows.

## Architecture

Commands are defined as static specifications (`SlashCommandSpec`) and dispatched through handlers that interact with the `runtime`, `git` workspace, or `plugin` manager.

## Slash Command Reference

| Command | Usage | When to use | Why? |
| :--- | :--- | :--- | :--- |
| **`/status`** | `/status` | Checking project health. | Displays current model, token usage, elapsed time, and provider connectivity. |
| **`/compact`** | `/compact` | When history is too long. | Summarizes previous messages into a single context-block to save tokens and improve model focus. |
| **`/model`** | `/model [id]` | Switching LLMs. | Hot-swaps the underlying model (e.g., Sonnet -> Gemini) without destroying session history. |
| **`/permissions`** | `/permissions [mode]` | Security escalation. | Toggles between `read-only`, `workspace-write`, and `danger-full-access` modes. |
| **`/cost`** | `/cost` | Budget tracking. | Shows real-time spend estimates and token breakdown for the active session. |
| **`/init`** | `/init` | Project onboarding. | Scans the workspace and generates a tailored `CLAW.md` for the agent. |
| **`/config`** | `/config [section]` | Debugging settings. | Displays the final merged configuration across all layers (User, Project, Local). |
| **`/commit`** | `/commit` | Finalizing changes. | Staging all changes and generating a semantic commit message automatically. |
| **`/branch`** | `/branch [list|create|switch]` | Task branching. | High-level Git branch management without leaving the REPL. |
| **`/plugins`** | `/plugins [list|install]` | Extending capability. | Manages the Model Context Protocol (MCP) servers and custom tool integrations. |

## Function-Level Documentation

| Function / Method | Description |
| :--- | :--- |
| **`SlashCommand::parse`** | Robust parser that handles command names, aliases, and argument tokenization. |
| **`handle_plugins_slash_command`** | Logic for installing, enabling, and disabling MCP plugins. |
| **`handle_commit_slash_command`** | Orchestrates Git staging and semantic message generation via an internal agent turn. |
| **`handle_branch_slash_command`** | Wraps Git operations for safe branch manipulation within the workspace. |
| **`handle_commit_push_pr_slash_command`** | Full workflow: Commit changes, push to remote, and create a PR via `gh`. |

## Developer Guide

### Adding a New Command
1.  **Define the Variant**: Add the new command to the `SlashCommand` enum.
2.  **Declare the Spec**: Add an entry to `SLASH_COMMAND_SPECS` with a category and summary.
3.  **Implement Handler**: Write the execution logic (ideally abstracted from the UI).
4.  **Register Aliases**: Ensure common short-forms are mapped in the parser.

## Current Status

- [x] **Git Workflow Integration**: High-level branch, worktree, and commit support.
- [x] **Plugin Management**: Integrated CLI handlers for the MCP ecosystem.
- [x] **Context Awareness**: Commands like `/memory` and `/config` assist in debugging agent behavior.
- [x] **Tab Completion**: Integrated with the REPL's input engine.

## Future Work

- [ ] **`/undo` / `/redo`**: Atomic state reversal for session history.
- [ ] **Interactive TUI Forms**: Replacing raw string arguments with guided terminal prompts.
- [ ] **Macro Support**: allowing users to chain slash-commands together for automation.
