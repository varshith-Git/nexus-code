# Claw LSP

The `lsp` crate provides integration with the Language Server Protocol (LSP). It allows Claw Code to leverage professional-grade static analysis tools to enrich its understanding of your workspace.

## Architecture

This crate implements an LSP client that communicates with server binaries (like `rust-analyzer` or `pyright`) via JSON-RPC over standard I/O.

### Key Components

- **`LspManager`**: The top-level orchestrator. It manages the lifecycle of multiple LSP servers and provides an aggregated view of workspace diagnostics and symbols.
- **`LspClient`**: Handles the low-level JSON-RPC message passing, document synchronization (`didOpen`, `didChange`, etc.), and request/response matching.
- **`LspContextEnrichment`**: A utility that extracts relevant information (like current definitions or nearby references) and formats it into a prompt section for the LLM.
- **`WorkspaceDiagnostics`**: Collects and prioritizes errors and warnings across the entire project.

## Features

- **Diagnostic Tracking**: Real-time monitoring of compilation errors and linting warnings.
- **Symbol Navigation**: Support for "Go to Definition" and "Find References" directly from the REPL.
- **Prompt Enrichment**: Automatically injects relevant code context into the agent's prompt based on the cursor position or current focus.
- **Multi-Server Support**: Can concurrently run different language servers for polyglot repositories.

## Current Status

- [x] JSON-RPC client implementation
- [x] Document synchronization
- [x] Diagnostic collection (PublishDiagnostics)
- [x] Go to Definition / References
- [x] Prompt context enrichment

## Future Work

- [ ] **Auto-Discovery**: Automatically detect and start the appropriate LSP server for the current workspace.
- [ ] **Semantic Highlighting**: Use LSP data for more accurate syntax highlighting in the REPL.
- [ ] **Code Actions**: Allow the agent to suggest and apply LSP-provided quick-fixes.
