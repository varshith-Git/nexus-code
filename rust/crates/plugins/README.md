# Claw Plugins

The `plugins` crate implements the extensibility model for Claw Code. It allows users and developers to extend the agent's capabilities through custom tools, commands, and hooks.

## Architecture

Claw uses a manifest-based plugin system. Plugins can be "Built-in," "Bundled" (included in the source), or "External" (installed from a local path or Git URL).

### Key Components

- **`PluginManifest`**: Defined in `plugin.json`, this specifies the tools, commands, and hooks provided by the plugin, along with required permissions.
- **`PluginManager`**: Handles the discovery, installation, enabling/disabling, and updating of plugins.
- **`HookRunner`**: Orchestrates the execution of "pre-tool" and "post-tool" hooks defined by plugins.
- **`PluginTool`**: A specialized tool implementation that executes an external binary/script as a workspace tool.

## Plugin Capabilities

Plugins can currently extend Claw in three ways:
1.  **Custom Tools**: Define new tools that the agent can call (e.g., a "Deploy" tool or a "Database Query" tool).
2.  **Custom Commands**: Add new slash commands to the REPL.
3.  **Hooks**: Execute logic before or after a tool runs. Hooks can deny execution based on custom logic (e.g., preventing a `Write` call to a protected file).

## Developer Guide

### Creating a Plugin
1.  Create a directory with a `.claw-plugin/plugin.json` manifest.
2.  Define your tools or hooks in the manifest.
3.  Implement the logic in scripts (Bash, Python, etc.) or binaries.
4.  Install the plugin using `/plugins install <path>`.

## Current Status

- [x] Plugin manifest parsing
- [x] Plugin management (list/install/enable/disable)
- [x] Hook system (Pre/Post tool execution)
- [x] External tool execution via CLI
- [x] Permission-aware plugin execution

## Future Work

- [ ] **Marketplace**: A centralized registry for discovering and installing community plugins.
- [ ] **Sandboxed Hooks**: Running hook scripts in a more restricted environment.
- [ ] **Richer Lifecycle**: More events (e.g., `OnSessionStart`, `OnMessageReceived`).
