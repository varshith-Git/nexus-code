use std::collections::VecDeque;
use std::time::Duration;

use serde::Deserialize;
use serde_json::{json, Value};

use crate::error::ApiError;
use crate::types::{
    ContentBlockDelta, ContentBlockDeltaEvent, ContentBlockStartEvent, ContentBlockStopEvent,
    InputContentBlock, InputMessage, MessageDelta, MessageDeltaEvent, MessageRequest,
    MessageResponse, MessageStartEvent, MessageStopEvent, OutputContentBlock, StreamEvent,
    ToolChoice, ToolDefinition, ToolResultContentBlock, Usage,
};

use super::{Provider, ProviderFuture};

pub const DEFAULT_BASE_URL: &str = "https://generativelanguage.googleapis.com";
const DEFAULT_INITIAL_BACKOFF: Duration = Duration::from_millis(200);
const DEFAULT_MAX_BACKOFF: Duration = Duration::from_secs(2);
const DEFAULT_MAX_RETRIES: u32 = 2;

const GEMINI_ENV_VARS: &[&str] = &["GEMINI_API_KEY"];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GeminiConfig {
    pub provider_name: &'static str,
    pub api_key_env: &'static str,
    pub base_url_env: &'static str,
    pub default_base_url: &'static str,
}

impl GeminiConfig {
    #[must_use]
    pub const fn default() -> Self {
        Self {
            provider_name: "Gemini",
            api_key_env: "GEMINI_API_KEY",
            base_url_env: "GEMINI_BASE_URL",
            default_base_url: DEFAULT_BASE_URL,
        }
    }

    #[must_use]
    pub fn credential_env_vars(self) -> &'static [&'static str] {
        GEMINI_ENV_VARS
    }
}

#[derive(Debug, Clone)]
pub struct GeminiClient {
    http: reqwest::Client,
    api_key: String,
    base_url: String,
    max_retries: u32,
    initial_backoff: Duration,
    max_backoff: Duration,
}

impl GeminiClient {
    #[must_use]
    pub fn new(api_key: impl Into<String>, config: GeminiConfig) -> Self {
        Self {
            http: reqwest::Client::new(),
            api_key: api_key.into(),
            base_url: read_base_url(config),
            max_retries: DEFAULT_MAX_RETRIES,
            initial_backoff: DEFAULT_INITIAL_BACKOFF,
            max_backoff: DEFAULT_MAX_BACKOFF,
        }
    }

    pub fn from_env(config: GeminiConfig) -> Result<Self, ApiError> {
        let Some(api_key) = read_env_non_empty(config.api_key_env)? else {
            return Err(ApiError::missing_credentials(
                config.provider_name,
                config.credential_env_vars(),
            ));
        };
        Ok(Self::new(api_key, config))
    }

    #[must_use]
    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    #[must_use]
    pub fn with_retry_policy(
        mut self,
        max_retries: u32,
        initial_backoff: Duration,
        max_backoff: Duration,
    ) -> Self {
        self.max_retries = max_retries;
        self.initial_backoff = initial_backoff;
        self.max_backoff = max_backoff;
        self
    }

    pub async fn send_message(
        &self,
        request: &MessageRequest,
    ) -> Result<MessageResponse, ApiError> {
        let response = self.send_with_retry(request, false).await?;
        let raw: GeminiResponse = response.json().await.map_err(ApiError::from)?;
        normalize_response(&request.model, raw)
    }

    pub async fn stream_message(
        &self,
        request: &MessageRequest,
    ) -> Result<MessageStream, ApiError> {
        let response = self.send_with_retry(request, true).await?;
        Ok(MessageStream {
            request_id: None,
            response,
            parser: GeminiSseParser::new(),
            pending: VecDeque::new(),
            done: false,
            state: StreamState::new(request.model.clone()),
        })
    }

    async fn send_with_retry(
        &self,
        request: &MessageRequest,
        stream: bool,
    ) -> Result<reqwest::Response, ApiError> {
        let mut attempts = 0;

        let last_error = loop {
            attempts += 1;
            let retryable_error = match self.send_raw_request(request, stream).await {
                Ok(response) => match expect_success(response).await {
                    Ok(response) => return Ok(response),
                    Err(error) if error.is_retryable() && attempts <= self.max_retries + 1 => error,
                    Err(error) => return Err(error),
                },
                Err(error) if error.is_retryable() && attempts <= self.max_retries + 1 => error,
                Err(error) => return Err(error),
            };

            if attempts > self.max_retries {
                break retryable_error;
            }

            tokio::time::sleep(self.backoff_for_attempt(attempts)?).await;
        };

        Err(ApiError::RetriesExhausted {
            attempts,
            last_error: Box::new(last_error),
        })
    }

    async fn send_raw_request(
        &self,
        request: &MessageRequest,
        stream: bool,
    ) -> Result<reqwest::Response, ApiError> {
        let endpoint = generate_content_endpoint(&self.base_url, &request.model, stream);
        self.http
            .post(&endpoint)
            .header("content-type", "application/json")
            .query(&[("key", &self.api_key)])
            .json(&build_gemini_request(request))
            .send()
            .await
            .map_err(ApiError::from)
    }

    fn backoff_for_attempt(&self, attempt: u32) -> Result<Duration, ApiError> {
        let Some(multiplier) = 1_u32.checked_shl(attempt.saturating_sub(1)) else {
            return Err(ApiError::BackoffOverflow {
                attempt,
                base_delay: self.initial_backoff,
            });
        };
        Ok(self
            .initial_backoff
            .checked_mul(multiplier)
            .map_or(self.max_backoff, |delay| delay.min(self.max_backoff)))
    }
}

impl Provider for GeminiClient {
    type Stream = MessageStream;

    fn send_message<'a>(
        &'a self,
        request: &'a MessageRequest,
    ) -> ProviderFuture<'a, MessageResponse> {
        Box::pin(async move { self.send_message(request).await })
    }

    fn stream_message<'a>(
        &'a self,
        request: &'a MessageRequest,
    ) -> ProviderFuture<'a, Self::Stream> {
        Box::pin(async move { self.stream_message(request).await })
    }
}

// ---------------------------------------------------------------------------
// Streaming
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub struct MessageStream {
    request_id: Option<String>,
    response: reqwest::Response,
    parser: GeminiSseParser,
    pending: VecDeque<StreamEvent>,
    done: bool,
    state: StreamState,
}

impl MessageStream {
    #[must_use]
    pub fn request_id(&self) -> Option<&str> {
        self.request_id.as_deref()
    }

    pub async fn next_event(&mut self) -> Result<Option<StreamEvent>, ApiError> {
        loop {
            if let Some(event) = self.pending.pop_front() {
                return Ok(Some(event));
            }

            if self.done {
                self.pending.extend(self.state.finish()?);
                if let Some(event) = self.pending.pop_front() {
                    return Ok(Some(event));
                }
                return Ok(None);
            }

            match self.response.chunk().await? {
                Some(chunk) => {
                    for parsed in self.parser.push(&chunk)? {
                        self.pending.extend(self.state.ingest_chunk(parsed)?);
                    }
                }
                None => {
                    self.done = true;
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// SSE parser for Gemini streaming responses
// Gemini emits: data: <JSON of GeminiResponse>\n\n
// ---------------------------------------------------------------------------

#[derive(Debug, Default)]
struct GeminiSseParser {
    buffer: Vec<u8>,
}

impl GeminiSseParser {
    fn new() -> Self {
        Self::default()
    }

    fn push(&mut self, chunk: &[u8]) -> Result<Vec<GeminiResponse>, ApiError> {
        self.buffer.extend_from_slice(chunk);
        let mut events = Vec::new();

        while let Some(frame) = next_sse_frame(&mut self.buffer) {
            if let Some(event) = parse_sse_frame(&frame)? {
                events.push(event);
            }
        }

        Ok(events)
    }
}

// ---------------------------------------------------------------------------
// Stream state machine — translates Gemini chunks → internal StreamEvents
// ---------------------------------------------------------------------------

#[derive(Debug)]
struct StreamState {
    model: String,
    message_id: String,
    message_started: bool,
    text_started: bool,
    text_finished: bool,
    finished: bool,
    stop_reason: Option<String>,
    usage: Option<Usage>,
    // tool call accumulation: index → (id, name, accumulated_args, started, stopped)
    tool_calls: Vec<ToolCallAccum>,
}

#[derive(Debug, Default)]
struct ToolCallAccum {
    id: String,
    name: String,
    arguments: String,
    started: bool,
    stopped: bool,
}

impl StreamState {
    fn new(model: String) -> Self {
        Self {
            model,
            message_id: uuid_v4(),
            message_started: false,
            text_started: false,
            text_finished: false,
            finished: false,
            stop_reason: None,
            usage: None,
            tool_calls: Vec::new(),
        }
    }

    fn ingest_chunk(&mut self, chunk: GeminiResponse) -> Result<Vec<StreamEvent>, ApiError> {
        let mut events = Vec::new();

        if !self.message_started {
            self.message_started = true;
            events.push(StreamEvent::MessageStart(MessageStartEvent {
                message: MessageResponse {
                    id: self.message_id.clone(),
                    kind: "message".to_string(),
                    role: "assistant".to_string(),
                    content: Vec::new(),
                    model: self.model.clone(),
                    stop_reason: None,
                    stop_sequence: None,
                    usage: Usage {
                        input_tokens: 0,
                        cache_creation_input_tokens: 0,
                        cache_read_input_tokens: 0,
                        output_tokens: 0,
                    },
                    request_id: None,
                },
            }));
        }

        // Collect usage from usageMetadata if present
        if let Some(meta) = chunk.usage_metadata {
            self.usage = Some(Usage {
                input_tokens: meta.prompt_token_count,
                cache_creation_input_tokens: 0,
                cache_read_input_tokens: 0,
                output_tokens: meta.candidates_token_count,
            });
        }

        for candidate in chunk.candidates {
            // Capture finish reason
            if let Some(reason) = &candidate.finish_reason {
                self.stop_reason = Some(normalize_finish_reason(reason));
            }

            let content = candidate.content.unwrap_or_default();
            for part in content.parts {
                if let Some(text) = part.text.filter(|t| !t.is_empty()) {
                    if !self.text_started {
                        self.text_started = true;
                        events.push(StreamEvent::ContentBlockStart(ContentBlockStartEvent {
                            index: 0,
                            content_block: OutputContentBlock::Text {
                                text: String::new(),
                            },
                        }));
                    }
                    events.push(StreamEvent::ContentBlockDelta(ContentBlockDeltaEvent {
                        index: 0,
                        delta: ContentBlockDelta::TextDelta { text },
                    }));
                }

                if let Some(fc) = part.function_call {
                    // Find or create tool call accumulator
                    let tc_index = self
                        .tool_calls
                        .iter()
                        .position(|tc| tc.name == fc.name)
                        .unwrap_or_else(|| {
                            self.tool_calls.push(ToolCallAccum {
                                id: format!("call_{}", self.tool_calls.len()),
                                name: fc.name.clone(),
                                arguments: String::new(),
                                started: false,
                                stopped: false,
                            });
                            self.tool_calls.len() - 1
                        });

                    // Block index: text takes 0, tool calls start at 1
                    let block_index = (tc_index + 1) as u32;
                    let args_json = fc.args.to_string();
                    self.tool_calls[tc_index].arguments.push_str(&args_json);

                    if !self.tool_calls[tc_index].started {
                        self.tool_calls[tc_index].started = true;
                        events.push(StreamEvent::ContentBlockStart(ContentBlockStartEvent {
                            index: block_index,
                            content_block: OutputContentBlock::ToolUse {
                                id: self.tool_calls[tc_index].id.clone(),
                                name: self.tool_calls[tc_index].name.clone(),
                                input: json!({}),
                            },
                        }));
                    }

                    events.push(StreamEvent::ContentBlockDelta(ContentBlockDeltaEvent {
                        index: block_index,
                        delta: ContentBlockDelta::InputJsonDelta {
                            partial_json: args_json,
                        },
                    }));
                }
            }
        }

        Ok(events)
    }

    fn finish(&mut self) -> Result<Vec<StreamEvent>, ApiError> {
        if self.finished {
            return Ok(Vec::new());
        }
        self.finished = true;

        let mut events = Vec::new();

        if self.text_started && !self.text_finished {
            self.text_finished = true;
            events.push(StreamEvent::ContentBlockStop(ContentBlockStopEvent {
                index: 0,
            }));
        }

        for (i, tc) in self.tool_calls.iter_mut().enumerate() {
            let block_index = (i + 1) as u32;
            if !tc.started {
                tc.started = true;
                events.push(StreamEvent::ContentBlockStart(ContentBlockStartEvent {
                    index: block_index,
                    content_block: OutputContentBlock::ToolUse {
                        id: tc.id.clone(),
                        name: tc.name.clone(),
                        input: json!({}),
                    },
                }));
                if !tc.arguments.is_empty() {
                    events.push(StreamEvent::ContentBlockDelta(ContentBlockDeltaEvent {
                        index: block_index,
                        delta: ContentBlockDelta::InputJsonDelta {
                            partial_json: tc.arguments.clone(),
                        },
                    }));
                }
            }
            if !tc.stopped {
                tc.stopped = true;
                events.push(StreamEvent::ContentBlockStop(ContentBlockStopEvent {
                    index: block_index,
                }));
            }
        }

        if self.message_started {
            events.push(StreamEvent::MessageDelta(MessageDeltaEvent {
                delta: MessageDelta {
                    stop_reason: Some(
                        self.stop_reason
                            .clone()
                            .unwrap_or_else(|| "end_turn".to_string()),
                    ),
                    stop_sequence: None,
                },
                usage: self.usage.clone().unwrap_or(Usage {
                    input_tokens: 0,
                    cache_creation_input_tokens: 0,
                    cache_read_input_tokens: 0,
                    output_tokens: 0,
                }),
            }));
            events.push(StreamEvent::MessageStop(MessageStopEvent {}));
        }

        Ok(events)
    }
}

// ---------------------------------------------------------------------------
// Gemini API request/response types (serde)
// ---------------------------------------------------------------------------

/// Top-level Gemini generateContent request body.
fn build_gemini_request(request: &MessageRequest) -> Value {
    let mut body = json!({
        "contents": translate_messages(&request.messages),
        "generationConfig": {
            "maxOutputTokens": request.max_tokens,
        },
    });

    if let Some(system) = request.system.as_ref().filter(|s| !s.is_empty()) {
        body["systemInstruction"] = json!({
            "parts": [{ "text": system }]
        });
    }

    if let Some(tools) = &request.tools {
        body["tools"] = json!([{
            "functionDeclarations": tools.iter().map(gemini_tool_definition).collect::<Vec<_>>()
        }]);
    }

    if let Some(tool_choice) = &request.tool_choice {
        body["toolConfig"] = gemini_tool_config(tool_choice);
    }

    body
}

fn translate_messages(messages: &[InputMessage]) -> Vec<Value> {
    let mut out = Vec::new();
    for msg in messages {
        let role = if msg.role == "assistant" { "model" } else { "user" };
        let parts = translate_content_blocks(&msg.content, &msg.role);
        if !parts.is_empty() {
            out.push(json!({ "role": role, "parts": parts }));
        }
    }
    out
}

fn translate_content_blocks(blocks: &[InputContentBlock], role: &str) -> Vec<Value> {
    let mut parts = Vec::new();
    for block in blocks {
        match block {
            InputContentBlock::Text { text } => {
                parts.push(json!({ "text": text }));
            }
            InputContentBlock::ToolUse { name, input, .. } => {
                // assistant's tool use → functionCall part
                parts.push(json!({
                    "functionCall": {
                        "name": name,
                        "args": input,
                    }
                }));
            }
            InputContentBlock::ToolResult {
                tool_use_id,
                content,
                is_error,
            } => {
                // user's tool result → functionResponse part
                let result_text = flatten_tool_result(content);
                let response_body = if *is_error {
                    json!({ "error": result_text })
                } else {
                    json!({ "output": result_text })
                };
                // Gemini requires the function name in the response, but we only have the id.
                // Use the id as a best-effort name; real implementations would track id → name.
                let _ = role; // suppress unused warning
                parts.push(json!({
                    "functionResponse": {
                        "name": tool_use_id,
                        "response": response_body,
                    }
                }));
            }
        }
    }
    parts
}

fn flatten_tool_result(content: &[ToolResultContentBlock]) -> String {
    content
        .iter()
        .map(|b| match b {
            ToolResultContentBlock::Text { text } => text.clone(),
            ToolResultContentBlock::Json { value } => value.to_string(),
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn gemini_tool_definition(tool: &ToolDefinition) -> Value {
    let mut def = json!({
        "name": tool.name,
        "parameters": strip_unsupported_schema_fields(&tool.input_schema),
    });
    if let Some(desc) = &tool.description {
        def["description"] = json!(desc);
    }
    def
}

/// Gemini's function declaration schema is a strict subset of JSON Schema.
/// It rejects: `additionalProperties`, array `type` values like ["string","null"],
/// and some other fields OpenAI tools allow.
/// This recursively strips/normalizes those keys so the request doesn't fail.
fn strip_unsupported_schema_fields(schema: &Value) -> Value {
    match schema {
        Value::Object(map) => {
            let mut new_map = serde_json::Map::new();
            for (key, value) in map {
                // Drop fields Gemini doesn't support in function parameter schemas
                if matches!(
                    key.as_str(),
                    "additionalProperties" | "$schema" | "$id" | "definitions" | "$defs"
                ) {
                    continue;
                }

                // Gemini only accepts "type": "string", not "type": ["string", "null"]
                // Flatten array types to the first non-null type (or "string" as fallback)
                if key == "type" {
                    if let Value::Array(types) = value {
                        let picked = types
                            .iter()
                            .find_map(|t| {
                                t.as_str()
                                    .filter(|s| *s != "null")
                                    .map(|s| Value::String(s.to_string()))
                            })
                            .or_else(|| {
                                types.first().and_then(|t| {
                                    t.as_str().map(|s| Value::String(s.to_string()))
                                })
                            })
                            .unwrap_or_else(|| Value::String("string".to_string()));
                        new_map.insert(key.clone(), picked);
                        continue;
                    }
                }

                new_map.insert(key.clone(), strip_unsupported_schema_fields(value));
            }
            Value::Object(new_map)
        }
        Value::Array(arr) => {
            Value::Array(arr.iter().map(strip_unsupported_schema_fields).collect())
        }
        other => other.clone(),
    }
}

fn gemini_tool_config(choice: &ToolChoice) -> Value {
    match choice {
        ToolChoice::Auto => json!({
            "functionCallingConfig": { "mode": "AUTO" }
        }),
        ToolChoice::Any => json!({
            "functionCallingConfig": { "mode": "ANY" }
        }),
        ToolChoice::Tool { name } => json!({
            "functionCallingConfig": {
                "mode": "ANY",
                "allowedFunctionNames": [name],
            }
        }),
    }
}

// ---------------------------------------------------------------------------
// Gemini response deserialization
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GeminiResponse {
    #[serde(default)]
    candidates: Vec<GeminiCandidate>,
    #[serde(default)]
    usage_metadata: Option<GeminiUsageMetadata>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GeminiCandidate {
    #[serde(default)]
    content: Option<GeminiContent>,
    #[serde(default)]
    finish_reason: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GeminiContent {
    #[serde(default)]
    parts: Vec<GeminiPart>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GeminiPart {
    #[serde(default)]
    text: Option<String>,
    #[serde(default)]
    function_call: Option<GeminiFunctionCall>,
}

#[derive(Debug, Deserialize)]
struct GeminiFunctionCall {
    name: String,
    #[serde(default)]
    args: Value,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GeminiUsageMetadata {
    #[serde(default)]
    prompt_token_count: u32,
    #[serde(default)]
    candidates_token_count: u32,
}

#[derive(Debug, Deserialize)]
struct GeminiErrorEnvelope {
    error: GeminiErrorBody,
}

#[derive(Debug, Deserialize)]
struct GeminiErrorBody {
    message: Option<String>,
    status: Option<String>,
}

// ---------------------------------------------------------------------------
// Response normalization: GeminiResponse → MessageResponse
// ---------------------------------------------------------------------------

fn normalize_response(model: &str, raw: GeminiResponse) -> Result<MessageResponse, ApiError> {
    let id = uuid_v4();
    let mut content = Vec::new();

    let usage = raw.usage_metadata.map_or(
        Usage {
            input_tokens: 0,
            cache_creation_input_tokens: 0,
            cache_read_input_tokens: 0,
            output_tokens: 0,
        },
        |meta| Usage {
            input_tokens: meta.prompt_token_count,
            cache_creation_input_tokens: 0,
            cache_read_input_tokens: 0,
            output_tokens: meta.candidates_token_count,
        },
    );

    let mut stop_reason = None;

    for candidate in raw.candidates {
        if let Some(reason) = &candidate.finish_reason {
            stop_reason = Some(normalize_finish_reason(reason));
        }

        let parts = candidate.content.unwrap_or_default().parts;
        for part in parts {
            if let Some(text) = part.text.filter(|t| !t.is_empty()) {
                content.push(OutputContentBlock::Text { text });
            }
            if let Some(fc) = part.function_call {
                content.push(OutputContentBlock::ToolUse {
                    id: format!("call_{}", content.len()),
                    name: fc.name,
                    input: fc.args,
                });
            }
        }
    }

    Ok(MessageResponse {
        id,
        kind: "message".to_string(),
        role: "assistant".to_string(),
        content,
        model: model.to_string(),
        stop_reason,
        stop_sequence: None,
        usage,
        request_id: None,
    })
}

fn normalize_finish_reason(reason: &str) -> String {
    match reason {
        "STOP" | "stop" => "end_turn",
        "MAX_TOKENS" | "max_tokens" => "max_tokens",
        "MALFORMED_FUNCTION_CALL" | "SAFETY" => "stop_sequence",
        "FINISH_REASON_UNSPECIFIED" | "OTHER" => "end_turn",
        other => other,
    }
    .to_string()
}

// ---------------------------------------------------------------------------
// SSE frame parsing
// ---------------------------------------------------------------------------

fn next_sse_frame(buffer: &mut Vec<u8>) -> Option<String> {
    let separator = buffer
        .windows(2)
        .position(|w| w == b"\n\n")
        .map(|pos| (pos, 2))
        .or_else(|| {
            buffer
                .windows(4)
                .position(|w| w == b"\r\n\r\n")
                .map(|pos| (pos, 4))
        })?;

    let (position, sep_len) = separator;
    let frame = buffer.drain(..position + sep_len).collect::<Vec<_>>();
    let frame_len = frame.len().saturating_sub(sep_len);
    Some(String::from_utf8_lossy(&frame[..frame_len]).into_owned())
}

fn parse_sse_frame(frame: &str) -> Result<Option<GeminiResponse>, ApiError> {
    let trimmed = frame.trim();
    if trimmed.is_empty() {
        return Ok(None);
    }

    let mut data_lines: Vec<&str> = Vec::new();
    for line in trimmed.lines() {
        if line.starts_with(':') {
            continue;
        }
        if let Some(data) = line.strip_prefix("data:") {
            data_lines.push(data.trim_start());
        }
    }

    if data_lines.is_empty() {
        return Ok(None);
    }

    let payload = data_lines.join("\n");
    if payload == "[DONE]" {
        return Ok(None);
    }

    serde_json::from_str(&payload)
        .map(Some)
        .map_err(ApiError::from)
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn read_env_non_empty(key: &str) -> Result<Option<String>, ApiError> {
    match std::env::var(key) {
        Ok(value) if !value.is_empty() => Ok(Some(value)),
        Ok(_) | Err(std::env::VarError::NotPresent) => Ok(None),
        Err(error) => Err(ApiError::from(error)),
    }
}

#[must_use]
pub fn has_api_key() -> bool {
    read_env_non_empty("GEMINI_API_KEY")
        .ok()
        .and_then(std::convert::identity)
        .is_some()
}

#[must_use]
pub fn read_base_url(config: GeminiConfig) -> String {
    std::env::var(config.base_url_env)
        .unwrap_or_else(|_| config.default_base_url.to_string())
}

fn generate_content_endpoint(base_url: &str, model: &str, stream: bool) -> String {
    let trimmed = base_url.trim_end_matches('/');
    let method = if stream {
        "streamGenerateContent"
    } else {
        "generateContent"
    };
    format!("{trimmed}/v1beta/models/{model}:{method}")
}

/// Minimal pseudo-UUID using the current time and a counter. Avoids pulling in the `uuid` crate.
fn uuid_v4() -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::time::{SystemTime, UNIX_EPOCH};

    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_nanos() as u64);
    let count = COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("gemini-{ts:x}-{count:x}")
}

async fn expect_success(response: reqwest::Response) -> Result<reqwest::Response, ApiError> {
    let status = response.status();
    if status.is_success() {
        return Ok(response);
    }

    let body = response.text().await.unwrap_or_default();
    let parsed = serde_json::from_str::<GeminiErrorEnvelope>(&body).ok();
    let retryable = is_retryable_status(status);

    Err(ApiError::Api {
        status,
        error_type: parsed.as_ref().and_then(|e| e.error.status.clone()),
        message: parsed.as_ref().and_then(|e| e.error.message.clone()),
        body,
        retryable,
    })
}

const fn is_retryable_status(status: reqwest::StatusCode) -> bool {
    matches!(status.as_u16(), 408 | 409 | 429 | 500 | 502 | 503 | 504)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::{
        build_gemini_request, generate_content_endpoint, normalize_finish_reason,
        translate_messages,
    };
    use crate::types::{
        InputContentBlock, InputMessage, MessageRequest, ToolChoice, ToolDefinition,
        ToolResultContentBlock,
    };
    use serde_json::json;

    fn simple_request() -> MessageRequest {
        MessageRequest {
            model: "gemini-2.0-flash".to_string(),
            max_tokens: 100,
            messages: vec![InputMessage::user_text("hello")],
            system: None,
            tools: None,
            tool_choice: None,
            stream: false,
        }
    }

    #[test]
    fn builds_basic_request_body() {
        let req = simple_request();
        let body = build_gemini_request(&req);
        assert_eq!(body["contents"][0]["role"], json!("user"));
        assert_eq!(body["contents"][0]["parts"][0]["text"], json!("hello"));
        assert_eq!(body["generationConfig"]["maxOutputTokens"], json!(100));
        assert!(body.get("systemInstruction").is_none());
    }

    #[test]
    fn includes_system_instruction_when_set() {
        let mut req = simple_request();
        req.system = Some("be concise".to_string());
        let body = build_gemini_request(&req);
        assert_eq!(
            body["systemInstruction"]["parts"][0]["text"],
            json!("be concise")
        );
    }

    #[test]
    fn skips_empty_system_instruction() {
        let mut req = simple_request();
        req.system = Some(String::new());
        let body = build_gemini_request(&req);
        assert!(body.get("systemInstruction").is_none());
    }

    #[test]
    fn translates_assistant_role_to_model() {
        let msgs = vec![InputMessage {
            role: "assistant".to_string(),
            content: vec![InputContentBlock::Text {
                text: "I am here".to_string(),
            }],
        }];
        let parts = translate_messages(&msgs);
        assert_eq!(parts[0]["role"], json!("model"));
    }

    #[test]
    fn translates_tool_definitions() {
        let mut req = simple_request();
        req.tools = Some(vec![ToolDefinition {
            name: "get_weather".to_string(),
            description: Some("Returns weather".to_string()),
            input_schema: json!({ "type": "object", "properties": { "city": { "type": "string" } } }),
        }]);
        let body = build_gemini_request(&req);
        let func = &body["tools"][0]["functionDeclarations"][0];
        assert_eq!(func["name"], json!("get_weather"));
        assert_eq!(func["description"], json!("Returns weather"));
    }

    #[test]
    fn translates_tool_choice_auto() {
        let mut req = simple_request();
        req.tool_choice = Some(ToolChoice::Auto);
        let body = build_gemini_request(&req);
        assert_eq!(
            body["toolConfig"]["functionCallingConfig"]["mode"],
            json!("AUTO")
        );
    }

    #[test]
    fn translates_tool_choice_any() {
        let mut req = simple_request();
        req.tool_choice = Some(ToolChoice::Any);
        let body = build_gemini_request(&req);
        assert_eq!(
            body["toolConfig"]["functionCallingConfig"]["mode"],
            json!("ANY")
        );
    }

    #[test]
    fn translates_specific_tool_choice() {
        let mut req = simple_request();
        req.tool_choice = Some(ToolChoice::Tool {
            name: "get_weather".to_string(),
        });
        let body = build_gemini_request(&req);
        assert_eq!(
            body["toolConfig"]["functionCallingConfig"]["allowedFunctionNames"][0],
            json!("get_weather")
        );
    }

    #[test]
    fn translates_tool_result_blocks() {
        let msgs = vec![InputMessage {
            role: "user".to_string(),
            content: vec![InputContentBlock::ToolResult {
                tool_use_id: "call_123".to_string(),
                content: vec![ToolResultContentBlock::Text {
                    text: "sunny".to_string(),
                }],
                is_error: false,
            }],
        }];
        let parts = translate_messages(&msgs);
        assert_eq!(
            parts[0]["parts"][0]["functionResponse"]["response"]["output"],
            json!("sunny")
        );
    }

    #[test]
    fn normalize_stop_reasons() {
        assert_eq!(normalize_finish_reason("STOP"), "end_turn");
        assert_eq!(normalize_finish_reason("stop"), "end_turn");
        assert_eq!(normalize_finish_reason("MAX_TOKENS"), "max_tokens");
        assert_eq!(normalize_finish_reason("MALFORMED_FUNCTION_CALL"), "stop_sequence");
    }

    #[test]
    fn endpoint_uses_correct_method_names() {
        assert_eq!(
            generate_content_endpoint("https://generativelanguage.googleapis.com", "gemini-2.0-flash", false),
            "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent"
        );
        assert_eq!(
            generate_content_endpoint("https://generativelanguage.googleapis.com", "gemini-2.0-flash", true),
            "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:streamGenerateContent"
        );
    }

    #[test]
    fn endpoint_strips_trailing_slash() {
        assert_eq!(
            generate_content_endpoint("https://generativelanguage.googleapis.com/", "gemini-1.5-pro", false),
            "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-pro:generateContent"
        );
    }
}
