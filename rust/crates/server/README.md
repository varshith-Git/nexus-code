# Claw Server

The `server` crate provides an optional HTTP backend for Claw Code, allowing for remote session management, centralized orchestration, and a web-based interface.

## Architecture

Built using **Axum**, the server implements a RESTful API for session lifecycle and a Server-Sent Events (SSE) endpoint for real-time interaction streaming.

### Key Components

- **`SessionRegistry`**: An in-memory and persistent store of all active and historical agent sessions.
- **REST API**: Standard endpoints for creating, listing, and querying agent sessions.
- **SSE Stream**: A high-performance bridge that pipes `runtime` events directly to HTTP clients in real-time.
- **State Partitioning**: Logical separation of session data to ensure multi-user isolation (where configured).

## Function-Level Documentation

| Function / Method | Description |
| :--- | :--- |
| **`app`** | The main router definition, including middleware for tracing, timeout, and CORS. |
| **`create_session`** | Generates a new `SessionId` and initializes a `ConversationRuntime` in the background. |
| **`stream_session_events`** | Upgrades an HTTP request to an SSE stream to provide real-time updates of agent "thinking" and tool calls. |
| **`list_sessions`** | Returns a paginated list of available session metadata and cumulative costs. |
| **`send_message`** | Injects a new user message into an existing session, triggering the agentic loop. |

## Developer Guide

### Running as a Service
You can launch the server as a standalone daemon:
```bash
cargo run --bin claw -- server # (Planned CLI entry point)
```

### API Usage
The server follows a "Fire and Forget" then "Stream" pattern:
1. `POST /sessions` to create.
2. `GET /sessions/{id}/events` to subscribe to the streaming response.
3. `POST /sessions/{id}/message` to interact.

## Current Status

- [x] **Axum integration**: High-performance, concurrent request handling.
- [x] **SSE Streaming**: Full parity with the CLI's native streaming experience.
- [x] **Session persistence**: Shared state management between the server and local CLI files.
- [x] **Unit Testing**: robust coverage of the session lifecycle and event piping.

## Future Work

- [ ] **Authentication Middleware**: Adding JWT or OAuth2 protection to the API endpoints.
- [ ] **Web UI**: A first-party frontend for interacting with agents via a browser.
- [ ] **Multi-tenant isolation**: Encrypted storage and per-user workspace sandboxing.
- [ ] **Deployment Templates**: Docker and Kubernetes configurations for cloud hosting.
