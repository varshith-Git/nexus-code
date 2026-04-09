# Claw Code

Claw Code is a local coding-agent CLI implemented in safe Rust. It is **Claude Code inspired** and developed as a **clean-room implementation**: it aims for a strong local agent experience, but it is **not** a direct port or copy of Claude Code.

The code and primary implementation live in the **[`rust/`](rust/)** directory.

## Current status

- **Version:** `0.1.0`
- **Release stage:** Initial public release, source-build distribution
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

### OAuth Verification
For supported platforms, you can use the built-in OAuth flow:
```bash
cargo run --bin claw -- login
```

## Quick Start (Build & Run)

```bash
cd rust/
cargo build --release -p claw-cli
./target/release/claw
```

## Supported capabilities

- **Local LLM Intelligence**: Connect to Ollama or LM Studio for 100% private, offline coding.
- **Multi-threaded Parallel Tool Execution**: Concurrent processing of complex agent workflows for ultra-low latency.
- **Robust Tool Calling**: Built-in JSON recovery layers ensure stability even with smaller local models.
- **Model Context Protocol (MCP)**: Extend functionality with external tool servers (Stdio/SSE).
- **Interactive REPL**: Rich terminal experience with markdown rendering and Vim-mode support.
- **Granular Permissions**: Fine-grained security modes (`read-only`, `workspace-write`, `danger-full-access`).
- **Slash Commands**: High-level controls for history compaction, cost tracking, git workflows, and more.

## Architecture (Folders in `rust/`)

- **[`claw-cli`](rust/crates/claw-cli/README.md)**: User-facing binary and REPL engine.
- **[`api`](rust/crates/api/README.md)**: Unified provider clients and streaming SSE parser.
- **[`runtime`](rust/crates/runtime/README.md)**: The "brain" — handles the agentic loop, parallel execution, and permissions.
- **[`tools`](rust/crates/tools/README.md)**: Built-in toolset (Filesystem, Shell, Web Search).
- **[`commands`](rust/crates/commands/README.md)**: REPL slash-command handlers.
- **[`plugins`](rust/crates/plugins/README.md)**: MCP server management.
- **[`lsp`](rust/crates/lsp/README.md)**: Workspace context via Language Server Protocol.

## Roadmap

- [ ] **TUI Elements**: Rich progress visualization and interactive dashboard.
- [ ] **Advanced Memory**: Long-term memory management and RAG-based context injection.
- [ ] **Public Artifacts**: Automated release packaging for macOS/Linux/Windows.

## License

See the **[LICENSE](LICENSE)** file for details.
