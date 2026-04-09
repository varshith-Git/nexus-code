# Claw Api

The `api` crate is the foundational layer of Claw Code, responsible for all communication with Large Language Model (LLM) providers. It provides a unified, provider-agnostic interface for streaming chat completions, tool definitions, and auth management.

## Architecture

The crate is built around a "dispatch" model where a central `ProviderClient` handles routing requests to specific provider implementations.

### Key Components

- **`ProviderClient`**: The main dispatcher enum. It encapsulates all supported clients (Anthropic, Gemini, OpenAI-compat) and routes `stream()` calls to the appropriate backend.
- **`ApiClient`**: A trait (primarily implemented through `ClawApiClient` but abstracted for the runtime) that defines how to interact with an LLM.
- **`SseParser`**: A robust, stateful Server-Sent Events (SSE) parser that handles the raw byte streams from LLM providers and converts them into structured `StreamEvent` objects.
- **`types`**: Data models for the unified API, including `InputMessage`, `ToolDefinition`, and `Usage`.

## Supported Backend Providers

Claw Code currently supports:
- **Anthropic**: Native integration with Claude 3.5 Sonnet/Haiku.
- **Gemini**: Integration via the Google Generative AI API (Gemini 1.5/2.0).
- **OpenRouter / OpenAI-Compat**: A generic wrapper that supports any OpenAI-compatible endpoint. This includes deep integration with OpenRouter (including custom headers for platform attribution).

## Developer Guide

### Adding a New Provider
1.  Add a new variant to `ProviderKind` in `crates/api/src/providers/mod.rs`.
2.  Implement the provider-specific logic in `crates/api/src/providers/<name>.rs`.
3.  Update the `ProviderClient` enum in `crates/api/src/client.rs` to include the new backend.
4.  Ensure that the new provider correctly translates its specific response format into the unified `StreamEvent` type.

### Environment Handling
The crate includes built-in helpers for resolving API keys from environment variables (e.g., `ANTHROPIC_API_KEY`, `OPENROUTER_API_KEY`) and provides actionable error messages when credentials are missing.

## Current Status

- [x] Anthropic (native)
- [x] Gemini (native)
- [x] OpenRouter (OpenAI-compatible)
- [x] Streaming support (SSE)
- [x] Usage tracking (token counts)

## Future Work

- [ ] Support for **Groq** and **DeepSeek** native protocols (currently handled via OpenAI-compat).
- [ ] Improved multi-modal input support (images/documents).
- [ ] Advanced `ToolChoice` logic for force-calling specific tools.
