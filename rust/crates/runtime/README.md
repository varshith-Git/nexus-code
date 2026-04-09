# Claw Runtime

The `runtime` crate is the core engine of Claw Code. It orchestrates the entire agentic loop, manages session persistence, enforces security permissions, and handles high-performance parallel tool execution.

## Architecture

The runtime is designed for high-concurrency and safe workspace interaction. It operates on a sophisticated "Turn" model with integrated stability layers.

### The Conversation Loop (`ConversationRuntime`)

Every interaction follows a multi-step loop:
1.  **Prompt Building**: Concatenates system prompt, workspace context, and session history based on the active `PromptStrategy`.
2.  **Model Inference**: Streams responses from the `api` crate.
3.  **JSON Recovery & Repair**: For local models that may produce malformed structured data, the runtime applies a `repair_json` healing layer.
4.  **Parallel Tool Execution**: authorized tools are executed concurrently using `std::thread::scope`.

## Function-Level Documentation

| Function / Method | Description |
| :--- | :--- |
| **`ConversationRuntime::run_turn`** | Executes a single completion turn, managing the stream and tool execution state machine. |
| **`repair_json`** | Heals malformed JSON tool calls (handles markdown fences, trailing commas, and partial blocks). |
| **`Session::persist`** | Saves the current conversation history and usage metrics to the local filesystem. |
| **`UsageTracker::track`** | Real-time accounting of tokens and estimated financial costs across different LLM providers. |
| **`PermissionPolicy::check`** | Enforces security boundaries by validating tool calls against user-defined modes. |
| **`load_system_prompt`** | Dynamically builds the system instructions including workspace-specific `CLAW.md` rules. |

## Developer Guide

### Prompt Strategies
The runtime supports different `PromptStrategy` variants:
- **`HostedStrictJson`**: Optimization for models like Claude/Gemini that follow strict JSON tool-calling schemas.
- **`LocalXmlTools`**: (Planned) Fallback for smaller local models that perform better with XML-tagged tool blocks.

### Stability Layers
The `repair_json` utility is a critical safety net for local LLM integration. It transparently sanitizes the output of models that might struggle with strict JSON syntax, ensuring that tool execution doesn't fail due to simple formatting errors.

## Current Status

- [x] **Parallel execution**: Thread-scoped concurrent tool processing.
- [x] **JSON Repair**: Robust recovery logic for malformed model outputs.
- [x] **MCP Integration**: Full Model Context Protocol support.
- [x] **Usage & cost tracking**: Accurate cross-provider resource accounting.
- [x] **Permission Modes**: Read-only to Danger-Full-Access security tiers.

## Future Work

- [ ] **Context Pre-fetching**: Speculative loading of relevant files into context based on semantic analysis.
- [ ] **Sub-Agent Orchestration**: Unified API for spawning and managing specialized child agents for recursive tasks.
- [ ] **History Compaction**: Advanced semantic summarization for long-lived sessions.
