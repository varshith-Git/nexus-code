# Claw Plugins

The `plugins` crate implements support for extending Claw Code's capabilities through external tool servers and custom scripts. It primarily leverages the **Model Context Protocol (MCP)** to integrate with a vast ecosystem of third-party tools.

## Architecture

Plugins are managed through a central `PluginManager` that handles installation, metadata validation, and lifecycle orchestration (Initialize/Shutdown).

### Plugin Types

- **Built-in Plugins**: Bundled directly with the binary (e.g., standard Git helpers).
- **External MCP Plugins**: Stdio-based or SSE-based JSON-RPC servers that provide their own tool definitions.
- **Dynamic Hooks**: (Planned) Custom scripts that can intercept or modify agent prompts and tool results.

## Function-Level Documentation

| Function / Method | Description |
| :--- | :--- |
| **`PluginManager::new`** | Initializes the manager with a specific configuration (install root, bundled path). |
| **`PluginManager::install`** | Validates and copies a new plugin from a local directory or remote source. |
| **`PluginManager::aggregated_tools`** | Scans all enabled plugins and aggregates their `ToolDefinition` objects for the LLM. |
| **`McpServer::call_tool`** | Executes a JSON-RPC call to an MCP server and returns the structured result. |
| **`load_enabled_plugins`** | Reads the persistent configuration to determine which plugins should be active on startup. |

## Developer Guide

### Using MCP
Claw Code supports any MCP-compliant server. To use a new set of tools:
1. Ensure the MCP server is compatible with the "Stdio" transport.
2. Use the CLI or slash command to register the server:
   ```bash
   claw --local # Local plugins are auto-loaded from ~/.claw/plugins
   ```
3. The `plugins` crate will automatically discover and expose the tools to the active `ConversationRuntime`.

## Current Status

- [x] **MCP Stdio Transport**: Full support for local executable tool servers.
- [x] **Plugin Registry**: Persistent tracking of installed and enabled plugins.
- [x] **Tool Aggregation**: Seamless merging of plugin tools into the agent's system prompt.
- [x] **Safe Initialization**: Timeout-guarded handshakes with external processes.

## Future Work

- [ ] **MCP SSE Transport**: Support for connecting to remote tool servers over HTTP.
- [ ] **Marketplace Support**: Integrated discovery of popular community plugins.
- [ ] **Plugin Sandboxing**: Running plugin executables within restricted OS containers.
