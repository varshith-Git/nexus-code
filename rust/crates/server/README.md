# Claw Server

The `server` crate provides a lightweight web service for remote monitoring and management of Claw Code sessions. It uses a high-performance, asynchronous architecture powered by the `axum` framework.

## Architecture

The server acts as a centralized broadcast hub for session events, allowing multiple clients (like a web dashboard or external monitor) to observe the agent's work in real-time.

### Key Components

- **Axum Router**: Defines a RESTful API for listing, creating, and retrieving session details.
- **Broadcaster**: Uses `tokio::sync::broadcast` to push session events (snapshots, new messages) to all connected clients.
- **SSE Stream**: Provides a real-time Server-Sent Events (SSE) endpoint for low-latency observation of the agent's "thinking" process.
- **`AppState`**: Manages the in-memory `SessionStore`, which acts as a cache for active sessions before they are fully persisted by the runtime.

## API Endpoints

- **`POST /sessions`**: Create a new interactive session.
- **`GET /sessions`**: List all active managed sessions.
- **`GET /sessions/{id}`**: Retrieve the full state (snapshot) of a session.
- **`GET /sessions/{id}/events`**: Subscribe to a real-time SSE stream of session events.
- **`POST /sessions/{id}/message`**: (Draft) Inject a message directly into an active session.

## Current Status

- [x] Axum-based REST API
- [x] Real-time SSE event broadcasting
- [x] Snapshot/Message event model
- [x] Multi-client subscription support

## Future Work

- [ ] **Authentication**: Secure the API with OAuth2 or API tokens.
- [ ] **Web UI**: A built-in dashboard for visual session management.
- [ ] **Remote Control**: Full bidirectional control over the REPL via the API.
