use crate::error::ApiError;
use crate::providers::claw_provider::{self, AuthSource, ClawApiClient};
use crate::providers::gemini::{self, GeminiClient, GeminiConfig};
use crate::providers::openai_compat::{self, OpenAiCompatClient, OpenAiCompatConfig};
use crate::providers::{self, Provider, ProviderKind};
use crate::types::{MessageRequest, MessageResponse, StreamEvent};

async fn send_via_provider<P: Provider>(
    provider: &P,
    request: &MessageRequest,
) -> Result<MessageResponse, ApiError> {
    provider.send_message(request).await
}

async fn stream_via_provider<P: Provider>(
    provider: &P,
    request: &MessageRequest,
) -> Result<P::Stream, ApiError> {
    provider.stream_message(request).await
}

#[derive(Debug, Clone)]
pub enum ProviderClient {
    ClawApi(ClawApiClient),
    Xai(OpenAiCompatClient),
    OpenAi(OpenAiCompatClient),
    Gemini(GeminiClient),
    DeepSeek(OpenAiCompatClient),
    OpenRouter(OpenAiCompatClient),
    Ollama(OpenAiCompatClient),
    Local(OpenAiCompatClient),
}

impl ProviderClient {
    pub fn from_model(model: &str) -> Result<Self, ApiError> {
        Self::from_model_with_default_auth(model, None)
    }

    pub fn from_model_with_default_auth(
        model: &str,
        default_auth: Option<AuthSource>,
    ) -> Result<Self, ApiError> {
        let resolved_model = providers::resolve_model_alias(model);
        match providers::detect_provider_kind(&resolved_model) {
            ProviderKind::ClawApi => Ok(Self::ClawApi(match default_auth {
                Some(auth) => ClawApiClient::from_auth(auth),
                None => ClawApiClient::from_env()?,
            })),
            ProviderKind::Xai => Ok(Self::Xai(OpenAiCompatClient::from_env(
                OpenAiCompatConfig::xai(),
            )?)),
            ProviderKind::OpenAi => Ok(Self::OpenAi(OpenAiCompatClient::from_env(
                OpenAiCompatConfig::openai(),
            )?)),
            ProviderKind::Gemini => Ok(Self::Gemini(GeminiClient::from_env(
                GeminiConfig::default(),
            )?)),
            ProviderKind::DeepSeek => Ok(Self::DeepSeek(OpenAiCompatClient::from_env(
                OpenAiCompatConfig::deepseek(),
            )?)),
            ProviderKind::OpenRouter => Ok(Self::OpenRouter(OpenAiCompatClient::from_env(
                OpenAiCompatConfig::openrouter(),
            )?)),
            ProviderKind::Ollama => Ok(Self::Ollama(OpenAiCompatClient::from_env(
                OpenAiCompatConfig::ollama(),
            )?)),
            ProviderKind::LocalOpenAICompat => Ok(Self::Local(OpenAiCompatClient::from_env(
                OpenAiCompatConfig::local_compat(),
            )?)),
        }
    }

    #[must_use]
    pub const fn provider_kind(&self) -> ProviderKind {
        match self {
            Self::ClawApi(_) => ProviderKind::ClawApi,
            Self::Xai(_) => ProviderKind::Xai,
            Self::OpenAi(_) => ProviderKind::OpenAi,
            Self::Gemini(_) => ProviderKind::Gemini,
            Self::DeepSeek(_) => ProviderKind::DeepSeek,
            Self::OpenRouter(_) => ProviderKind::OpenRouter,
            Self::Ollama(_) => ProviderKind::Ollama,
            Self::Local(_) => ProviderKind::LocalOpenAICompat,
        }
    }

    pub async fn check_health(&self) -> Result<(), ApiError> {
        match self {
            Self::Ollama(client) | Self::Local(client) => {
                let url = format!("{}/models", client.base_url());
                let resp = reqwest::Client::new()
                    .get(&url)
                    .timeout(std::time::Duration::from_millis(1500))
                    .send()
                    .await
                    .map_err(ApiError::Http)?;

                if !resp.status().is_success() {
                    return Err(ApiError::Api {
                        status: resp.status(),
                        error_type: None,
                        message: Some("Local server returned error status".to_string()),
                        body: String::new(),
                        retryable: false,
                    });
                }
                Ok(())
            }
            _ => Ok(()), // Health checks for hosted models aren't strictly necessary eagerly
        }
    }

    pub async fn send_message(
        &self,
        request: &MessageRequest,
    ) -> Result<MessageResponse, ApiError> {
        match self {
            Self::ClawApi(client) => send_via_provider(client, request).await,
            Self::Xai(client) 
            | Self::OpenAi(client) 
            | Self::DeepSeek(client) 
            | Self::OpenRouter(client) 
            | Self::Ollama(client)
            | Self::Local(client) => {
                send_via_provider(client, request).await
            }
            Self::Gemini(client) => send_via_provider(client, request).await,
        }
    }

    pub async fn stream_message(
        &self,
        request: &MessageRequest,
    ) -> Result<MessageStream, ApiError> {
        match self {
            Self::ClawApi(client) => stream_via_provider(client, request)
                .await
                .map(MessageStream::ClawApi),
            Self::Xai(client) 
            | Self::OpenAi(client) 
            | Self::DeepSeek(client) 
            | Self::OpenRouter(client)
            | Self::Ollama(client)
            | Self::Local(client) => {
                stream_via_provider(client, request)
                    .await
                    .map(MessageStream::OpenAiCompat)
            }
            Self::Gemini(client) => stream_via_provider(client, request)
                .await
                .map(MessageStream::Gemini),
        }
    }
}

#[derive(Debug)]
pub enum MessageStream {
    ClawApi(claw_provider::MessageStream),
    OpenAiCompat(openai_compat::MessageStream),
    Gemini(gemini::MessageStream),
}

impl MessageStream {
    #[must_use]
    pub fn request_id(&self) -> Option<&str> {
        match self {
            Self::ClawApi(stream) => stream.request_id(),
            Self::OpenAiCompat(stream) => stream.request_id(),
            Self::Gemini(stream) => stream.request_id(),
        }
    }

    pub async fn next_event(&mut self) -> Result<Option<StreamEvent>, ApiError> {
        match self {
            Self::ClawApi(stream) => stream.next_event().await,
            Self::OpenAiCompat(stream) => stream.next_event().await,
            Self::Gemini(stream) => stream.next_event().await,
        }
    }
}

pub use claw_provider::{
    oauth_token_is_expired, resolve_saved_oauth_token, resolve_startup_auth_source, OAuthTokenSet,
};
#[must_use]
pub fn read_base_url() -> String {
    claw_provider::read_base_url()
}

#[must_use]
pub fn read_xai_base_url() -> String {
    openai_compat::read_base_url(OpenAiCompatConfig::xai())
}

#[must_use]
pub fn read_gemini_base_url() -> String {
    gemini::read_base_url(GeminiConfig::default())
}

#[must_use]
pub fn read_deepseek_base_url() -> String {
    openai_compat::read_base_url(OpenAiCompatConfig::deepseek())
}

#[cfg(test)]
mod tests {
    use crate::providers::{detect_provider_kind, resolve_model_alias, ProviderKind};

    #[test]
    fn resolves_existing_and_grok_aliases() {
        assert_eq!(resolve_model_alias("opus"), "claude-opus-4-6");
        assert_eq!(resolve_model_alias("grok"), "grok-3");
        assert_eq!(resolve_model_alias("grok-mini"), "grok-3-mini");
    }

    #[test]
    fn provider_detection_prefers_model_family() {
        assert_eq!(detect_provider_kind("grok-3"), ProviderKind::Xai);
        assert_eq!(
            detect_provider_kind("claude-sonnet-4-6"),
            ProviderKind::ClawApi
        );
    }
}
