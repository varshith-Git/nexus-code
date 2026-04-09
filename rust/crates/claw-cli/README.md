# Claw CLI

`claw-cli` is the entry point for the Claw Code application. It provides the interactive REPL (Read-Eval-Print Loop), handles command-line arguments, and manages terminal rendering and user input.

## Architecture

The CLI is designed to be a thin wrapper around the `runtime` and `commands` crates, focusing on the human-machine interface.

### Key Components

- **`main.rs`**: The main entry point. It initializes the environment, parses arguments, and starts either the interactive REPL or a one-shot prompt session.
- **`app.rs`**: Orchestrates the REPL lifecycle. It manages transitions between user input, thinking states, and tool execution progress.
- **`input.rs`**: Handles user input (powered by `rustyline`). It supports command history, multiline input, and tab completion for slash commands.
- **`render.rs`**: Responsible for output formatting. It uses a custom markdown renderer to ensure that AI responses, tool calls, and code snippets are easy to read in the terminal.
- **`args.rs`**: Command-line argument parsing using `clap`. It handles model selection, session resumption, and one-shot commands.
- **`init.rs`**: Logic for the `/init` command, which bootstraps new workspaces with a `CLAW.md` file.

## Features

- **Interactive REPL**: A persistent session where the agent can interact with your workspace.
- **One-Shot Prompts**: Execute a single command without entering the full REPL (e.g., `claw prompt "summarize this file"`).
- **Session Resumption**: Resume any previous session using the session ID or log file.
- **Markdown Rendering**: High-quality, theme-aware rendering of agent responses.

## Current Status

- [x] Interactive REPL
- [x] One-shot prompt execution
- [x] Command-line argument handling
- [x] Tab completion for slash commands
- [x] Markdown terminal rendering

## Future Work

- [ ] Support for **TUI (Terminal User Interface)** elements for better progress visualization.
- [ ] Improved multi-line editing experience.
- [ ] Customizable themes for markdown rendering.
