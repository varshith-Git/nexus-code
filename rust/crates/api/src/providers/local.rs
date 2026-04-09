use std::time::Duration;
use reqwest::Client;
use serde_json::Value;

use crate::providers::ProviderKind;

pub async fn detect_local_provider() -> Option<ProviderKind> {
    let client = Client::builder()
        .timeout(Duration::from_millis(500))
        .build()
        .ok()?;

    // Check Ollama first (most common)
    if let Ok(resp) = client.get("http://localhost:11434/v1/models").send().await {
        if resp.status().is_success() {
            return Some(ProviderKind::Ollama);
        }
    }

    // Check LMStudio / generic proxy
    if let Ok(resp) = client.get("http://localhost:1234/v1/models").send().await {
        if resp.status().is_success() {
            return Some(ProviderKind::LocalOpenAICompat);
        }
    }

    None
}

pub async fn list_local_models(provider: ProviderKind) -> Result<Vec<String>, String> {
    let base_url = match provider {
        ProviderKind::Ollama => "http://localhost:11434/v1",
        ProviderKind::LocalOpenAICompat => "http://localhost:1234/v1",
        _ => return Err("Not a local provider".to_string()),
    };

    let client = Client::builder()
        .timeout(Duration::from_secs(2))
        .build()
        .map_err(|e| e.to_string())?;

    let url = format!("{}/models", base_url);
    let resp = client.get(&url).send().await.map_err(|e| e.to_string())?;

    if !resp.status().is_success() {
        return Err(format!("Server returned error status: {}", resp.status()));
    }

    let json: Value = resp.json().await.map_err(|e| e.to_string())?;
    
    let models = json.get("data")
        .and_then(|data| data.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|m| m.get("id").and_then(|i| i.as_str()).map(|s| s.to_string()))
                .collect::<Vec<String>>()
        })
        .unwrap_or_default();

    Ok(models)
}
