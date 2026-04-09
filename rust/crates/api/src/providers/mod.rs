use std::future::Future;
use std::pin::Pin;

use crate::error::ApiError;
use crate::types::{MessageRequest, MessageResponse};

pub mod claw_provider;
pub mod gemini;
pub mod local;
pub mod openai_compat;

pub type ProviderFuture<'a, T> = Pin<Box<dyn Future<Output = Result<T, ApiError>> + Send + 'a>>;

pub trait Provider {
    type Stream;

    fn send_message<'a>(
        &'a self,
        request: &'a MessageRequest,
    ) -> ProviderFuture<'a, MessageResponse>;

    fn stream_message<'a>(
        &'a self,
        request: &'a MessageRequest,
    ) -> ProviderFuture<'a, Self::Stream>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderKind {
    ClawApi,
    Xai,
    OpenAi,
    Gemini,
    DeepSeek,
    OpenRouter,
    Ollama,
    LocalOpenAICompat,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelSpec {
    pub provider: ProviderKind,
    pub model_name: String,
    pub base_url: String,
    pub requires_auth: bool,
    pub supports_tools: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModelCapabilities {
    pub tool_calling: bool,
    pub streaming: bool,
    pub json_mode: bool,
    pub max_context: usize,
}

impl ModelSpec {
    pub fn capabilities(&self) -> ModelCapabilities {
        match self.provider {
            ProviderKind::Ollama | ProviderKind::LocalOpenAICompat => ModelCapabilities {
                tool_calling: self.supports_tools,
                streaming: true,
                json_mode: true,
                max_context: 8192,
            },
            _ => {
                // Determine max context heuristically as before using fallback logic
                let max_tokens = if self.model_name.contains("opus") {
                    32_000
                } else {
                    64_000
                };
                ModelCapabilities {
                    tool_calling: true,
                    streaming: true,
                    json_mode: true,
                    max_context: max_tokens,
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProviderMetadata {
    pub provider: ProviderKind,
    pub auth_env: &'static str,
    pub base_url_env: &'static str,
    pub default_base_url: &'static str,
}

const MODEL_REGISTRY: &[(&str, ProviderMetadata)] = &[
    (
        "opus",
        ProviderMetadata {
            provider: ProviderKind::ClawApi,
            auth_env: "ANTHROPIC_API_KEY",
            base_url_env: "ANTHROPIC_BASE_URL",
            default_base_url: claw_provider::DEFAULT_BASE_URL,
        },
    ),
    (
        "sonnet",
        ProviderMetadata {
            provider: ProviderKind::ClawApi,
            auth_env: "ANTHROPIC_API_KEY",
            base_url_env: "ANTHROPIC_BASE_URL",
            default_base_url: claw_provider::DEFAULT_BASE_URL,
        },
    ),
    (
        "haiku",
        ProviderMetadata {
            provider: ProviderKind::ClawApi,
            auth_env: "ANTHROPIC_API_KEY",
            base_url_env: "ANTHROPIC_BASE_URL",
            default_base_url: claw_provider::DEFAULT_BASE_URL,
        },
    ),
    (
        "claude-opus-4-6",
        ProviderMetadata {
            provider: ProviderKind::ClawApi,
            auth_env: "ANTHROPIC_API_KEY",
            base_url_env: "ANTHROPIC_BASE_URL",
            default_base_url: claw_provider::DEFAULT_BASE_URL,
        },
    ),
    (
        "claude-sonnet-4-6",
        ProviderMetadata {
            provider: ProviderKind::ClawApi,
            auth_env: "ANTHROPIC_API_KEY",
            base_url_env: "ANTHROPIC_BASE_URL",
            default_base_url: claw_provider::DEFAULT_BASE_URL,
        },
    ),
    (
        "claude-haiku-4-5-20251213",
        ProviderMetadata {
            provider: ProviderKind::ClawApi,
            auth_env: "ANTHROPIC_API_KEY",
            base_url_env: "ANTHROPIC_BASE_URL",
            default_base_url: claw_provider::DEFAULT_BASE_URL,
        },
    ),
    (
        "grok",
        ProviderMetadata {
            provider: ProviderKind::Xai,
            auth_env: "XAI_API_KEY",
            base_url_env: "XAI_BASE_URL",
            default_base_url: openai_compat::DEFAULT_XAI_BASE_URL,
        },
    ),
    (
        "grok-3",
        ProviderMetadata {
            provider: ProviderKind::Xai,
            auth_env: "XAI_API_KEY",
            base_url_env: "XAI_BASE_URL",
            default_base_url: openai_compat::DEFAULT_XAI_BASE_URL,
        },
    ),
    (
        "grok-mini",
        ProviderMetadata {
            provider: ProviderKind::Xai,
            auth_env: "XAI_API_KEY",
            base_url_env: "XAI_BASE_URL",
            default_base_url: openai_compat::DEFAULT_XAI_BASE_URL,
        },
    ),
    (
        "grok-3-mini",
        ProviderMetadata {
            provider: ProviderKind::Xai,
            auth_env: "XAI_API_KEY",
            base_url_env: "XAI_BASE_URL",
            default_base_url: openai_compat::DEFAULT_XAI_BASE_URL,
        },
    ),
    (
        "grok-2",
        ProviderMetadata {
            provider: ProviderKind::Xai,
            auth_env: "XAI_API_KEY",
            base_url_env: "XAI_BASE_URL",
            default_base_url: openai_compat::DEFAULT_XAI_BASE_URL,
        },
    ),
    // ── Gemini models ──────────────────────────────────────────────────────
    // Short aliases (resolve in the alias arm below)
    (
        "gemini",
        ProviderMetadata {
            provider: ProviderKind::Gemini,
            auth_env: "GEMINI_API_KEY",
            base_url_env: "GEMINI_BASE_URL",
            default_base_url: gemini::DEFAULT_BASE_URL,
        },
    ),
    (
        "gemini-flash",
        ProviderMetadata {
            provider: ProviderKind::Gemini,
            auth_env: "GEMINI_API_KEY",
            base_url_env: "GEMINI_BASE_URL",
            default_base_url: gemini::DEFAULT_BASE_URL,
        },
    ),
    (
        "gemini-flash-lite",
        ProviderMetadata {
            provider: ProviderKind::Gemini,
            auth_env: "GEMINI_API_KEY",
            base_url_env: "GEMINI_BASE_URL",
            default_base_url: gemini::DEFAULT_BASE_URL,
        },
    ),
    (
        "gemini-pro",
        ProviderMetadata {
            provider: ProviderKind::Gemini,
            auth_env: "GEMINI_API_KEY",
            base_url_env: "GEMINI_BASE_URL",
            default_base_url: gemini::DEFAULT_BASE_URL,
        },
    ),
    // ── Gemini 2.5 (stable / recommended) ────────────────────────────────
    (
        "gemini-2.5-flash",
        ProviderMetadata {
            provider: ProviderKind::Gemini,
            auth_env: "GEMINI_API_KEY",
            base_url_env: "GEMINI_BASE_URL",
            default_base_url: gemini::DEFAULT_BASE_URL,
        },
    ),
    (
        "gemini-2.5-flash-lite",
        ProviderMetadata {
            provider: ProviderKind::Gemini,
            auth_env: "GEMINI_API_KEY",
            base_url_env: "GEMINI_BASE_URL",
            default_base_url: gemini::DEFAULT_BASE_URL,
        },
    ),
    (
        "gemini-2.5-pro",
        ProviderMetadata {
            provider: ProviderKind::Gemini,
            auth_env: "GEMINI_API_KEY",
            base_url_env: "GEMINI_BASE_URL",
            default_base_url: gemini::DEFAULT_BASE_URL,
        },
    ),
    // ── Gemini 2.0 (stable but deprecated by Google) ─────────────────────
    (
        "gemini-2.0-flash",
        ProviderMetadata {
            provider: ProviderKind::Gemini,
            auth_env: "GEMINI_API_KEY",
            base_url_env: "GEMINI_BASE_URL",
            default_base_url: gemini::DEFAULT_BASE_URL,
        },
    ),
    (
        "gemini-2.0-flash-lite",
        ProviderMetadata {
            provider: ProviderKind::Gemini,
            auth_env: "GEMINI_API_KEY",
            base_url_env: "GEMINI_BASE_URL",
            default_base_url: gemini::DEFAULT_BASE_URL,
        },
    ),
    // ── Gemini 3.x previews ───────────────────────────────────────────────
    (
        "gemini-3-flash",
        ProviderMetadata {
            provider: ProviderKind::Gemini,
            auth_env: "GEMINI_API_KEY",
            base_url_env: "GEMINI_BASE_URL",
            default_base_url: gemini::DEFAULT_BASE_URL,
        },
    ),
    (
        "gemini-3.1-flash",
        ProviderMetadata {
            provider: ProviderKind::Gemini,
            auth_env: "GEMINI_API_KEY",
            base_url_env: "GEMINI_BASE_URL",
            default_base_url: gemini::DEFAULT_BASE_URL,
        },
    ),
    (
        "gemini-3.1-flash-lite",
        ProviderMetadata {
            provider: ProviderKind::Gemini,
            auth_env: "GEMINI_API_KEY",
            base_url_env: "GEMINI_BASE_URL",
            default_base_url: gemini::DEFAULT_BASE_URL,
        },
    ),
    (
        "gemini-3.1-pro",
        ProviderMetadata {
            provider: ProviderKind::Gemini,
            auth_env: "GEMINI_API_KEY",
            base_url_env: "GEMINI_BASE_URL",
            default_base_url: gemini::DEFAULT_BASE_URL,
        },
    ),
];

#[must_use]
pub fn resolve_model_alias(model: &str) -> String {
    let trimmed = model.trim();
    let lower = trimmed.to_ascii_lowercase();
    MODEL_REGISTRY
        .iter()
        .find_map(|(alias, metadata)| {
            (*alias == lower).then_some(match metadata.provider {
                ProviderKind::ClawApi => match *alias {
                    "opus" => "claude-opus-4-6",
                    "sonnet" => "claude-sonnet-4-6",
                    "haiku" => "claude-haiku-4-5-20251213",
                    _ => trimmed,
                },
                ProviderKind::Xai => match *alias {
                    "grok" | "grok-3" => "grok-3",
                    "grok-mini" | "grok-3-mini" => "grok-3-mini",
                    "grok-2" => "grok-2",
                    _ => trimmed,
                },
                ProviderKind::OpenAi => trimmed,
                ProviderKind::Gemini => match *alias {
                    // Short aliases → latest stable recommended models
                    "gemini" | "gemini-flash"       => "gemini-2.5-flash",
                    "gemini-flash-lite"             => "gemini-2.5-flash-lite",
                    "gemini-pro"                    => "gemini-2.5-pro",
                    // Gemini 3 previews
                    "gemini-3-flash"    => "gemini-3-flash-preview-04-17",
                    "gemini-3.1-flash" => "gemini-3.1-flash-preview-04-17",
                    "gemini-3.1-flash-lite" => "gemini-3.1-flash-lite-preview-04-17",
                    "gemini-3.1-pro"   => "gemini-3.1-pro-preview-05-06",
                    // Exact IDs pass through unchanged
                    _ => trimmed,
                },
                ProviderKind::DeepSeek => match *alias {
                    "deepseek" | "deepseek-chat" => "deepseek-chat",
                    "deepseek-reasoner" => "deepseek-reasoner",
                    _ => trimmed,
                },
                ProviderKind::OpenRouter => trimmed.strip_prefix("openrouter/").unwrap_or(trimmed),
                ProviderKind::Ollama => trimmed.strip_prefix("ollama/").unwrap_or(trimmed),
                ProviderKind::LocalOpenAICompat => trimmed.strip_prefix("local/").unwrap_or(trimmed),
            })
        })
        .map(|s| s.to_string())
        .unwrap_or_else(|| {
            let mut result = trimmed;
            if result.starts_with("openrouter/") {
                result = result.strip_prefix("openrouter/").unwrap();
            } else if result.starts_with("ollama/") {
                result = result.strip_prefix("ollama/").unwrap();
            } else if result.starts_with("local/") {
                result = result.strip_prefix("local/").unwrap();
            }
            result.to_string()
        })
}

fn resolve_base_url(env_key: &str, default_url: &str) -> String {
    std::env::var(env_key).unwrap_or_else(|_| default_url.to_string())
}

#[must_use]
pub fn metadata_for_model(model: &str) -> Option<ModelSpec> {
    let canonical = resolve_model_alias(model);
    let lower = canonical.to_ascii_lowercase();
    if let Some((_, metadata)) = MODEL_REGISTRY.iter().find(|(alias, _)| *alias == lower) {
        return Some(ModelSpec {
            provider: metadata.provider,
            model_name: canonical.clone(),
            base_url: resolve_base_url(metadata.base_url_env, metadata.default_base_url),
            requires_auth: true,
            supports_tools: true,
        });
    }
    if lower.starts_with("grok") {
        return Some(ModelSpec {
            provider: ProviderKind::Xai,
            model_name: canonical.clone(),
            base_url: resolve_base_url("XAI_BASE_URL", openai_compat::DEFAULT_XAI_BASE_URL),
            requires_auth: true,
            supports_tools: true,
        });
    }
    if lower.starts_with("gemini") {
        return Some(ModelSpec {
            provider: ProviderKind::Gemini,
            model_name: canonical.clone(),
            base_url: resolve_base_url("GEMINI_BASE_URL", gemini::DEFAULT_BASE_URL),
            requires_auth: true,
            supports_tools: true,
        });
    }
    if lower.starts_with("deepseek") {
        return Some(ModelSpec {
            provider: ProviderKind::DeepSeek,
            model_name: canonical.clone(),
            base_url: resolve_base_url("DEEPSEEK_BASE_URL", openai_compat::DEFAULT_DEEPSEEK_BASE_URL),
            requires_auth: true,
            supports_tools: true,
        });
    }
    if lower.starts_with("openrouter/") {
        return Some(ModelSpec {
            provider: ProviderKind::OpenRouter,
            model_name: canonical.clone(),
            base_url: resolve_base_url("OPENROUTER_BASE_URL", openai_compat::DEFAULT_OPENROUTER_BASE_URL),
            requires_auth: true,
            supports_tools: true,
        });
    }
    if lower.starts_with("ollama/") {
        return Some(ModelSpec {
            provider: ProviderKind::Ollama,
            model_name: canonical.clone(),
            base_url: resolve_base_url("OLLAMA_BASE_URL", "http://localhost:11434/v1"),
            requires_auth: false,
            supports_tools: true,
        });
    }
    if lower.starts_with("local/") {
        return Some(ModelSpec {
            provider: ProviderKind::LocalOpenAICompat,
            model_name: canonical.clone(),
            base_url: resolve_base_url("LOCAL_BASE_URL", "http://localhost:1234/v1"),
            requires_auth: false,
            supports_tools: true,
        });
    }
    None
}

#[must_use]
pub fn detect_provider_kind(model: &str) -> ProviderKind {
    if let Some(metadata) = metadata_for_model(model) {
        return metadata.provider;
    }
    if claw_provider::has_auth_from_env_or_saved().unwrap_or(false) {
        return ProviderKind::ClawApi;
    }
    if openai_compat::has_api_key("OPENAI_API_KEY") {
        return ProviderKind::OpenAi;
    }
    if openai_compat::has_api_key("XAI_API_KEY") {
        return ProviderKind::Xai;
    }
    if openai_compat::has_api_key("DEEPSEEK_API_KEY") {
        return ProviderKind::DeepSeek;
    }
    if openai_compat::has_api_key("OPENROUTER_API_KEY") {
        return ProviderKind::OpenRouter;
    }
    if gemini::has_api_key() {
        return ProviderKind::Gemini;
    }
    ProviderKind::ClawApi
}

#[must_use]
pub fn max_tokens_for_model(model: &str) -> u32 {
    let canonical = resolve_model_alias(model);
    if canonical.contains("opus") {
        32_000
    } else {
        64_000
    }
}

#[cfg(test)]
mod tests {
    use super::{detect_provider_kind, max_tokens_for_model, resolve_model_alias, ProviderKind};

    #[test]
    fn resolves_grok_aliases() {
        assert_eq!(resolve_model_alias("grok"), "grok-3");
        assert_eq!(resolve_model_alias("grok-mini"), "grok-3-mini");
        assert_eq!(resolve_model_alias("grok-2"), "grok-2");
    }

    #[test]
    fn detects_provider_from_model_name_first() {
        assert_eq!(detect_provider_kind("grok"), ProviderKind::Xai);
        assert_eq!(
            detect_provider_kind("claude-sonnet-4-6"),
            ProviderKind::ClawApi
        );
    }

    #[test]
    fn keeps_existing_max_token_heuristic() {
        assert_eq!(max_tokens_for_model("opus"), 32_000);
        assert_eq!(max_tokens_for_model("grok-3"), 64_000);
    }
}
