# Claw Api

The `api` crate is the foundational layer of Claw Code, responsible for all communication with Large Language Model (LLM) providers. It provides a unified, provider-agnostic interface for streaming chat completions, tool definitions, and auth management.

## Architecture

The crate is built around a dynamic "Model Specification" model. Instead of hardcoded registries, it uses a flexible `ModelSpec` structure to define capabilities and routing.

### Key Components

- **`ProviderClient`**: The primary entry point. A unified dispatcher that handles Anthropic, Gemini, OpenAI-compatible, and Local-first providers.
- **`ModelSpec`**: A structured definition of a model's provider, name, base URL, and authentication requirements.
- **`ModelCapabilities`**: Tracks what a specific model can do (Tool calling, streaming, max context tokens).
- **`SseParser`**: A robust, stateful Server-Sent Events (SSE) parser that handles raw byte streams and converts them into structured `StreamEvent` objects.
- **Local Autodetection**: Built-in logic to scan for local inference servers (Ollama, LM Studio) on standard network ports.

## Function-Level Documentation

| Function / Method | Description |
| :--- | :--- |
| **`resolve_model_alias`** | Maps short names (e.g., `sonnet`, `gemini`) to full canonical model strings. |
| **`detect_provider_kind`** | Infers the correct `ProviderKind` from a model name or prefix. |
| **`ProviderClient::stream`** | The core method for starting a streaming chat completion request. |
| **`detect_local_provider`** | Asynchronously probes `localhost` for running Ollama or LM Studio instances. |
| **`list_local_models`** | Queries local providers for a list of currently downloaded/cached model weights. |
| **`check_health`** | Performs a connectivity check against a specific provider endpoint. |

## Developer Guide

### Working with Providers
The crate abstracts away the specifics of different API flavors. To add a new provider:
1. Add a variant to `ProviderKind`.
2. Implement the translation logic between Claw's `MessageRequest` and the provider's native JSON schema.
3. Hook the implementation into `ProviderClient`.

### Local Inference
The `providers/local.rs` module handles the complexities of zero-auth local servers. It uses `detect_local_provider` during CLI startup to determine if it should steer traffic toward a local backend if the user passes the `--local` flag.

## Current Status

- [x] **ModelSpec Refactor**: Moved from hardcoded metadata to dynamic model resolutions.
- [x] **Local LLM Support**: Native detection for Ollama and LM Studio.
- [x] **Automatic JSON Recovery**: (Handled in runtime, but API provides the signals).
- [x] **Unified SSE Parsing**: Handles multi-line data and nested JSON chunks correctly.

## Future Work

- [ ] **Native Protocols**: Implement direct native protocols for Groq and DeepSeek to bypass some OpenAI-compatibility overhead.
- [ ] **Multi-modal Support**: Adding `image` and `document` block types to the message specification.
- [ ] **Streaming Reasoning**: Support for "thinking" or "reasoning" block types (e.g., DeepSeek R1 / OpenAI o1).
