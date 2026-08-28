use serde_json::Value;

// ---------------------------------------------------------------------------
// Local AI Overlay State
// ---------------------------------------------------------------------------

pub(crate) const DEFAULT_AI_OVERLAY_SERVICE_URL: &str = "http://127.0.0.1:8765";
const MAX_AI_OVERLAY_URL_CHARS: usize = 256;
const MAX_AI_OVERLAY_TEXT_CHARS: usize = 320;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum AiOverlayConnection {
    Disconnected,
    Checking,
    NoDecision,
    DecisionAvailable,
    Error,
}

impl AiOverlayConnection {
    pub(crate) fn label(&self) -> &'static str {
        match self {
            Self::Disconnected => "Disconnected",
            Self::Checking => "Checking local service",
            Self::NoDecision => "Connected - no decision",
            Self::DecisionAvailable => "Decision available",
            Self::Error => "Service error",
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct AiOverlayState {
    pub(crate) connection: AiOverlayConnection,
    pub(crate) detail: Option<String>,
    pub(crate) request_generation: u64,
}

impl Default for AiOverlayState {
    fn default() -> Self {
        Self {
            connection: AiOverlayConnection::Disconnected,
            detail: None,
            request_generation: 0,
        }
    }
}

pub(crate) fn normalize_ai_overlay_service_url(value: &str) -> Result<String, String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err("Enter a local AI service address".to_string());
    }
    if trimmed.len() > MAX_AI_OVERLAY_URL_CHARS {
        return Err("Local AI service address is too long".to_string());
    }

    let url = reqwest::Url::parse(trimmed).map_err(|_| "Invalid local AI service address")?;
    if url.scheme() != "http"
        || !url.username().is_empty()
        || url.password().is_some()
        || url.query().is_some()
        || url.fragment().is_some()
        || !matches!(url.host_str(), Some("127.0.0.1" | "::1" | "localhost"))
    {
        return Err("AI service must use a loopback HTTP address without credentials".to_string());
    }
    if url.path() != "/" && !url.path().is_empty() {
        return Err("AI service address must not include a path".to_string());
    }

    Ok(url.as_str().trim_end_matches('/').to_string())
}

pub(crate) async fn fetch_ai_overlay_status(service_url: String) -> Result<Option<String>, String> {
    let url = format!("{service_url}/v1/status");
    let response = crate::api::CLIENT
        .get(url)
        .send()
        .await
        .map_err(|_| "Local AI service is unreachable".to_string())?
        .error_for_status()
        .map_err(|_| "Local AI service returned an error".to_string())?;
    let payload: Value = response
        .json()
        .await
        .map_err(|_| "Local AI service returned invalid JSON".to_string())?;

    let decision = payload
        .get("decision")
        .and_then(Value::as_str)
        .map(bounded_ai_text)
        .filter(|value| !value.is_empty());
    Ok(decision)
}

fn bounded_ai_text(value: &str) -> String {
    value.chars().take(MAX_AI_OVERLAY_TEXT_CHARS).collect()
}

#[cfg(test)]
mod tests {
    use super::{DEFAULT_AI_OVERLAY_SERVICE_URL, normalize_ai_overlay_service_url};

    #[test]
    fn accepts_only_plain_loopback_service_urls() {
        assert_eq!(
            normalize_ai_overlay_service_url(DEFAULT_AI_OVERLAY_SERVICE_URL),
            Ok(DEFAULT_AI_OVERLAY_SERVICE_URL.to_string())
        );
        assert_eq!(
            normalize_ai_overlay_service_url("http://localhost:9000/"),
            Ok("http://localhost:9000".to_string())
        );
        assert!(normalize_ai_overlay_service_url("https://127.0.0.1:8765").is_err());
        assert!(normalize_ai_overlay_service_url("http://example.com:8765").is_err());
        assert!(normalize_ai_overlay_service_url("http://token@127.0.0.1:8765").is_err());
        assert!(normalize_ai_overlay_service_url("http://127.0.0.1:8765/v1/status").is_err());
    }
}
