# Claw CLI

`claw-cli` is the entry point for the Claw Code application. It provides the interactive REPL (Read-Eval-Print Loop), handles command-line arguments, and manages terminal rendering and user input.

## Features

- **Local Autodetection**: Use the `--local` flag to automatically find and connect to Ollama or LM Studio.
- **Interactive REPL**: A persistent, Vim-mode capable terminal interface with markdown rendering.
- **Diagnostics**: Built-in health checks for local server connectivity.
- **One-Shot Prompts**: Execute single tasks directly from the shell without entering the REPL.

## CLI Command Reference

| Command | Usage | When to use | Why? |
| :--- | :--- | :--- | :--- |
| **`repl`** | `claw` | Primary interaction mode. | Starts a stateful, interactive session with the default or specified model. |
| **`prompt`** | `claw prompt "ask something"` | Quick tasks or automation. | Executes a single request and returns the output to stdout. |
| **`doctor`** | `claw doctor` | Troubleshooting. | Probes your network for local LLM servers (Ollama/LM Studio) and reports their health. |
| **`models`** | `claw models` | Inventory check. | Queries local providers to list all currently available model weights. |
| **`init`** | `claw init` | Project setup. | Generates a project-level `CLAW.md` to guide the agent's behavior in this workspace. |

## Function-Level Documentation

| Function / Method | Description |
| :--- | :--- |
| **`parse_args`** | Translates raw CLI command-line arguments into structured `CliAction` variants. |
| **`auto_detect_local_model`** | Executes the eager provider probe to configure local inference endpoints. |
| **`run_repl_loop`** | Manages the main interactive lifecycle (input, thinking, rendering, tool output). |
| **`handle_system_prompt`** | Resolves and loads the correct system instructions based on CLI flags. |

## Developer Guide

### Command Handling
The CLI uses `clap` for sophisticated argument management. Logic for each subcommand is dispatched in `main.rs` to specialized handlers in the `commands` or `runtime` crates.

### Local-First Workflow
When the `--local` flag is present, the CLI triggers `auto_detect_local_model`. It prioritizes connectivity to `localhost:11434` (Ollama) and `localhost:1234` (LM Studio), ensuring that the agent switches to private inference immediately.

## Current Status

- [x] **Subcommand dispatch**: Robust handling of `doctor`, `models`, and `prompt`.
- [x] **Markdown Rendering**: Theme-aware highlighting of AI responses and code blocks.
- [x] **Vim Input Mode**: Supported via `input.rs` and `rustyline`.
- [x] **One-shot execution**: Clean integration with piping and shell automation.

## Future Work

- [ ] **TUI (Terminal User Interface)**: Move from line-based to full-screen progress visualization.
- [ ] **Custom Themes**: Allowing users to define color palettes for different message types.
- [ ] **Interactive Onboarding**: A guided first-run experience for setting up API keys.
