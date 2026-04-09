# Claw Code

Claw Code is a local coding-agent CLI implemented in safe Rust. It is **Claude Code inspired** and developed as a **clean-room implementation**: it aims for a strong local agent experience, but it is **not** a direct port or copy of Claude Code.

The Rust workspace is the current main product surface. The `claw` binary provides interactive sessions, one-shot prompts, workspace-aware tools, local agent workflows, and plugin-capable operation from a single workspace.

## Current status

- **Version:** `0.1.0`
- **Release stage:** Initial public release, source-build distribution
- **Primary implementation:** Rust workspace in this repository
- **Platform focus:** macOS and Linux developer workstations

## Authentication & Providers

Claw Code supports multiple backend providers through a unified interface.

### Hosted Providers
Set the relevant environment variables for your chosen provider:
- **Anthropic**: `ANTHROPIC_API_KEY`
- **Gemini**: `GEMINI_API_KEY`
- **Grok**: `XAI_API_KEY`
- **DeepSeek**: `DEEPSEEK_API_KEY`
- **OpenRouter**: `OPENROUTER_API_KEY`

### Local LLMs (Ollama / LM Studio)
Claw Code features a **zero-cost, privacy-first local inference layer**. It can auto-detect locally running model servers on standard ports (11434 for Ollama, 1234 for LM Studio).

Run with the `--local` flag to enable auto-detection:
```bash
cargo run --bin claw -- --local
```

### OAuth Verification
For supported platforms, you can use the built-in OAuth flow:
```bash
cargo run --bin claw -- login
```

## Install and Run

### Install locally
```bash
cargo install --path crates/claw-cli --locked
```

### Run
```bash
# Start interactive REPL
claw

# Start REPL with local model auto-detection
claw --local

# One-shot prompt
claw prompt "summarize this workspace"

# Health check local LLM connectivity
claw doctor

# List available local models
claw models
```

## Supported capabilities

- **Local LLM Intelligence**: Connect to Ollama or LM Studio for 100% private, offline coding.
- **Multi-threaded Parallel Tool Execution**: Concurrent processing of complex agent workflows for ultra-low latency.
- **Robust Tool Calling**: Built-in JSON recovery layers ensure stability even with smaller local models.
- **Model Context Protocol (MCP)**: Extend functionality with external tool servers (Stdio/SSE).
- **Interactive REPL**: Rich terminal experience with markdown rendering and Vim-mode support.
- **Granular Permissions**: Fine-grained security modes (`read-only`, `workspace-write`, `danger-full-access`).
- **Slash Commands**: High-level controls for history compaction, cost tracking, git workflows, and more.

## Architecture

The project is organized into modular crates:

- **[`claw-cli`](crates/claw-cli/README.md)**: User-facing binary, REPL engine, and CLI command handling.
- **[`api`](crates/api/README.md)**: Unified provider clients, streaming SSE parser, and local model detection.
- **[`runtime`](crates/runtime/README.md)**: The "brain" — handles the agentic loop, parallel execution, permissions, and sessions.
- **[`tools`](crates/tools/README.md)**: Built-in toolset (Filesystem, Shell, Web Search, Multi-agent).
- **[`commands`](crates/commands/README.md)**: REPL slash-command registry and handlers (Git, Plugins, Worktrees).
- **[`plugins`](crates/plugins/README.md)**: Plugin discovery and MCP server management.
- **[`lsp`](crates/lsp/README.md)**: Workspace context extraction via Language Server Protocol.
- **[`server`](crates/server/README.md)**: Optional HTTP server for remote session orchestration.

## Roadmap

- [ ] **TUI Elements**: Rich progress visualization and interactive dashboard.
- [ ] **Advanced Memory**: Long-term memory management and RAG-based context injection.
- [ ] **Public Artifacts**: Automated release packaging for macOS/Linux/Windows.
- [ ] **Custom Themes**: Highly tailorable markdown and terminal styling.
- [ ] **Sub-Agent Orchestration**: Native support for spawning specialized child-agents for complex tasks.

## Release notes

- Draft 0.1.0 release notes: [`docs/releases/0.1.0.md`](docs/releases/0.1.0.md)

## License

See the repository root for licensing details.
