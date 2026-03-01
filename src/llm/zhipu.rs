use std::time::{Duration, SystemTime, UNIX_EPOCH};

use anyhow::Context;
use base64::Engine;
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;

use crate::llm::driver::{ChatFuture, LlmDriver};
use crate::llm::openai_compat::{
    ChatCompletionRequest, ChatMessage, Role, ToolCall, ToolDefinition, ToolFunction,
};

type HmacSha256 = Hmac<Sha256>;

const TOKEN_TTL_MS: i64 = 3 * 60 * 1000;

#[derive(Debug, Clone)]
pub struct ZhipuDriver {
    base_url: String,
    api_key: Option<String>,
    http: reqwest::Client,
}

impl ZhipuDriver {
    pub fn new(base_url: String, api_key: Option<String>) -> anyhow::Result<Self> {
        let base_url = base_url.trim_end_matches('/').to_string();
        let http = reqwest::Client::builder()
            .timeout(Duration::from_secs(60))
            .build()
            .context("build http client")?;
        Ok(Self {
            base_url,
            api_key,
            http,
        })
    }
}

impl LlmDriver for ZhipuDriver {
    fn chat<'a>(&'a self, req: ChatCompletionRequest) -> ChatFuture<'a> {
        Box::pin(async move {
            let token = self.api_key.as_deref().and_then(to_bearer_token);

            let zhipu_req = ZhipuRequest {
                model: req.model,
                messages: req.messages,
                tools: req.tools,
                temperature: req.temperature,
                stream: false,
            };

            let url = format!("{}/chat/completions", self.base_url);
            let mut http_req = self.http.post(url).json(&zhipu_req);
            if let Some(t) = token {
                if !t.trim().is_empty() {
                    http_req = http_req.bearer_auth(t);
                }
            }

            let resp = http_req
                .send()
                .await
                .context("send zhipu request")?
                .error_for_status()
                .context("zhipu HTTP error")?;

            let body: RawChatCompletionResponse = resp.json().await.context("decode zhipu response")?;
            let choice = body.choices.into_iter().next().context("no choices")?;
            Ok(map_message(choice.message)?)
        })
    }
}

#[derive(Debug, Clone, Serialize)]
struct ZhipuRequest {
    model: String,
    messages: Vec<ChatMessage>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    tools: Vec<ToolDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    stream: bool,
}

#[derive(Debug, Clone, Deserialize)]
struct RawChatCompletionResponse {
    choices: Vec<RawChoice>,
}

#[derive(Debug, Clone, Deserialize)]
struct RawChoice {
    message: RawChatMessage,
}

#[derive(Debug, Clone, Deserialize)]
struct RawChatMessage {
    role: Role,
    #[serde(default)]
    content: Option<String>,
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    tool_call_id: Option<String>,
    #[serde(default)]
    tool_calls: Option<Vec<ToolCall>>,
    #[serde(default)]
    function_call: Option<RawFunctionCall>,
}

#[derive(Debug, Clone, Deserialize)]
struct RawFunctionCall {
    name: String,
    arguments: String,
}

fn map_message(raw: RawChatMessage) -> anyhow::Result<ChatMessage> {
    let mut tool_calls = raw.tool_calls;
    if tool_calls.as_ref().map(|c| c.is_empty()).unwrap_or(true) {
        if let Some(fc) = raw.function_call {
            tool_calls = Some(vec![ToolCall {
                id: "call_1".to_string(),
                kind: "function".to_string(),
                function: ToolFunction {
                    name: fc.name,
                    arguments: fc.arguments,
                },
            }]);
        }
    }

    Ok(ChatMessage {
        role: raw.role,
        content: raw.content,
        name: raw.name,
        tool_call_id: raw.tool_call_id,
        tool_calls,
    })
}

fn to_bearer_token(api_key: &str) -> Option<String> {
    let key = api_key.trim();
    if key.is_empty() {
        return None;
    }

    let parts: Vec<&str> = key.split('.').collect();
    match parts.len() {
        2 => {
            let key_id = parts[0];
            let key_secret = parts[1];
            Some(sign_jwt(key_id, key_secret))
        }
        3 => Some(key.to_string()),
        _ => Some(key.to_string()),
    }
}

fn sign_jwt(key_id: &str, key_secret: &str) -> String {
    let now_ms: i64 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
        .try_into()
        .unwrap_or(0);
    let exp_ms = now_ms.saturating_add(TOKEN_TTL_MS);

    let header = serde_json::json!({
        "alg": "HS256",
        "sign_type": "SIGN",
        "typ": "JWT"
    });
    let claims = serde_json::json!({
        "api_key": key_id,
        "timestamp": now_ms,
        "exp": exp_ms
    });

    let header_b64 = b64_url_no_pad(serde_json::to_vec(&header).unwrap_or_default());
    let claims_b64 = b64_url_no_pad(serde_json::to_vec(&claims).unwrap_or_default());
    let signing_input = format!("{header_b64}.{claims_b64}");

    let mut mac = HmacSha256::new_from_slice(key_secret.as_bytes()).unwrap();
    mac.update(signing_input.as_bytes());
    let sig = mac.finalize().into_bytes();
    let sig_b64 = b64_url_no_pad(sig.to_vec());

    format!("{signing_input}.{sig_b64}")
}

fn b64_url_no_pad(data: Vec<u8>) -> String {
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(data)
}
