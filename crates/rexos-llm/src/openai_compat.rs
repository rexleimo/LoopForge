use std::time::Duration;

use anyhow::Context;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
    Tool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChatMessage {
    pub role: Role,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ToolCall {
    pub id: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub function: ToolFunction,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ToolFunction {
    pub name: String,
    pub arguments: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ToolDefinition {
    #[serde(rename = "type")]
    pub kind: String,
    pub function: ToolFunctionDefinition,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ToolFunctionDefinition {
    pub name: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub description: String,
    pub parameters: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatCompletionRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub tools: Vec<ToolDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatCompletionResponse {
    pub choices: Vec<Choice>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Choice {
    pub index: usize,
    pub message: ChatMessage,
    #[serde(default)]
    pub finish_reason: Option<String>,
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

#[derive(Debug, Clone)]
pub struct OpenAiCompatibleClient {
    base_url: String,
    api_key: Option<String>,
    http: reqwest::Client,
}

impl OpenAiCompatibleClient {
    pub fn new(base_url: String, api_key: Option<String>) -> anyhow::Result<Self> {
        let base_url = base_url.trim_end_matches('/').to_string();
        let timeout = openai_compat_timeout();
        let http = reqwest::Client::builder()
            .timeout(timeout)
            .build()
            .context("build http client")?;

        Ok(Self {
            base_url,
            api_key,
            http,
        })
    }

    pub async fn chat_completions(
        &self,
        req: ChatCompletionRequest,
    ) -> anyhow::Result<ChatMessage> {
        let url = format!("{}/chat/completions", self.base_url);

        let max_retries = llm_retry_max();

        for attempt in 0..=max_retries {
            let mut http_req = self.http.post(&url).json(&req);
            if let Some(key) = &self.api_key {
                if !key.is_empty() {
                    http_req = http_req.bearer_auth(key);
                }
            }

            let resp = http_req.send().await;
            match resp {
                Ok(resp) => {
                    if resp.status().is_success() {
                        let body: RawChatCompletionResponse = resp
                            .json()
                            .await
                            .context("decode chat completion response")?;

                        let choice = body.choices.into_iter().next().context("no choices")?;
                        let raw = choice.message;

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

                        return Ok(ChatMessage {
                            role: raw.role,
                            content: raw.content,
                            name: raw.name,
                            tool_call_id: raw.tool_call_id,
                            tool_calls,
                        });
                    }

                    let status = resp.status();
                    let body = resp.text().await.unwrap_or_default();
                    let retryable = is_retryable_status(status);
                    if attempt < max_retries && retryable {
                        sleep_retry_backoff(attempt + 1).await;
                        continue;
                    }

                    anyhow::bail!(
                        "chat completion HTTP error (status {}): {}",
                        status,
                        truncate_one_line(&body, 400)
                    );
                }
                Err(e) => {
                    let retryable = is_retryable_reqwest_error(&e);
                    if attempt < max_retries && retryable {
                        sleep_retry_backoff(attempt + 1).await;
                        continue;
                    }
                    return Err(e).context("send chat completion request");
                }
            }
        }

        unreachable!("retry loop should return or bail")
    }
}

fn openai_compat_timeout() -> Duration {
    const DEFAULT_SECS: u64 = 600;
    match std::env::var("REXOS_OPENAI_COMPAT_TIMEOUT_SECS") {
        Ok(raw) => match raw.trim().parse::<u64>() {
            Ok(secs) if secs > 0 => Duration::from_secs(secs),
            _ => Duration::from_secs(DEFAULT_SECS),
        },
        Err(_) => Duration::from_secs(DEFAULT_SECS),
    }
}

fn llm_retry_max() -> u32 {
    const DEFAULT: u32 = 2;
    match std::env::var("REXOS_LLM_RETRY_MAX") {
        Ok(v) => v.trim().parse::<u32>().ok().unwrap_or(DEFAULT),
        Err(_) => DEFAULT,
    }
}

fn is_retryable_status(status: reqwest::StatusCode) -> bool {
    matches!(status.as_u16(), 429 | 500 | 502 | 503 | 504)
}

fn is_retryable_reqwest_error(err: &reqwest::Error) -> bool {
    err.is_timeout() || err.is_connect()
}

async fn sleep_retry_backoff(retry_number: u32) {
    // Exponential backoff with small jitter (no extra RNG dependency).
    let base_ms: u64 = 250;
    let cap_ms: u64 = 2_000;

    let shift: u32 = retry_number.saturating_sub(1).min(31);
    let exp = 1u64.checked_shl(shift).unwrap_or(u64::MAX);
    let delay_ms = base_ms.saturating_mul(exp).min(cap_ms);

    let jitter_window = (delay_ms / 4).max(1);
    let jitter = match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
        Ok(d) => (d.subsec_nanos() as u64) % jitter_window,
        Err(_) => 0,
    };

    tokio::time::sleep(Duration::from_millis(delay_ms + jitter)).await;
}

fn truncate_one_line(s: &str, max_len: usize) -> String {
    let mut out = String::new();
    for ch in s.chars() {
        if ch == '\n' || ch == '\r' {
            break;
        }
        out.push(ch);
        if out.len() >= max_len {
            out.push_str("…");
            break;
        }
    }
    out.trim().to_string()
}
