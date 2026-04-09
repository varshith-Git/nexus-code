# Claw LSP (Language Server Protocol)

The `lsp` crate provides the integration layer between Claw Code and modern editor tooling. It is responsible for extracting rich "Project Context" by communicating with language servers (like `rust-analyzer` or `pyright`).

## Architecture

This crate implements a subset of the Language Server Protocol (LSP) focused on read-only diagnostics and symbol navigation. It allows the agent to see the workspace "as a compiler sees it."

### Key Features

- **Project Context**: Aggregates diagnostics (errors/warnings), project-wide symbol definitions, and file trees.
- **Diagnostic Extraction**: Pulls real-time compiler feedback into the agent's context to help with debugging.
- **Symbol Resolution**: Allows the agent to "Go to definition" across the workspace without opening every file.

## Function-Level Documentation

| Function / Method | Description |
| :--- | :--- |
| **`LspManager::initialize_project`** | Spawns and handshakes with a language server implementation for the current workspace. |
| **`LspManager::fetch_diagnostics`** | Returns a list of all current compiler errors and warnings in a structured format. |
| **`ProjectContext::build`** | Synthesizes a high-level summary of the codebase for the agent's system prompt. |
| **`LspServerConfig`** | Defines how to launch specific servers (e.g., command path, environment, initialization options). |

## Developer Guide

### Context Synthesis
The `lsp` crate is used by the `runtime` to build the "Repository shape" portion of the system prompt. By using the LSP instead of raw file reads, Claw can understand:
- Which files are primary entry points.
- Where definitions of structs and functions are located.
- What errors are currently preventing the project from building.

## Current Status

- [x] **Rust-Analyzer Integration**: Specific optimizations for Rust projects.
- [x] **Diagnostic Reporting**: Streaming of errors/warnings into the agent turn.
- [x] **Symbol Workspace Search**: (Partial) Support for locating definitions.
- [x] **Process Management**: Robust spawning and termination of child LSP processes.

## Future Work

- [ ] **Multi-language Support**: Standardized configurations for Python, TypeScript, and Go language servers.
- [ ] **Interactive Symbols**: Allowing the agent to trigger "Rename" or "Reference Search" directly.
- [ ] **LSP SSE/TCP**: Support for connecting to remote language servers.
- [ ] **Auto-Installation**: Automatically downloading and configuring language servers for missing environments.
