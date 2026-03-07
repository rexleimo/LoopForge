use std::time::Duration;

use anyhow::Context;
use serde::{Deserialize, Serialize};

use crate::driver::{ChatFuture, LlmDriver};
use crate::openai_compat::{ChatCompletionRequest, ChatMessage, Role, ToolCall, ToolDefinition};

#[derive(Debug, Clone)]
pub struct DashscopeDriver {
    base_url: String,
    api_key: Option<String>,
    http: reqwest::Client,
}

impl DashscopeDriver {
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

impl LlmDriver for DashscopeDriver {
    fn chat<'a>(&'a self, req: ChatCompletionRequest) -> ChatFuture<'a> {
        Box::pin(async move {
            let dash_req = DashscopeRequest {
                model: req.model,
                input: DashscopeInput {
                    messages: map_messages(&req.messages),
                },
                parameters: DashscopeParameters {
                    result_format: "message".to_string(),
                    temperature: req.temperature,
                    tools: req.tools,
                },
            };

            let url = format!("{}/services/aigc/text-generation/generation", self.base_url);
            let mut http_req = self.http.post(url).json(&dash_req);
            if let Some(key) = &self.api_key {
                if !key.trim().is_empty() {
                    http_req = http_req.bearer_auth(key);
                }
            }

            let resp = http_req
                .send()
                .await
                .context("send dashscope request")?
                .error_for_status()
                .context("dashscope HTTP error")?;

            let body: DashscopeResponse = resp.json().await.context("decode dashscope response")?;
            let choice = body
                .output
                .choices
                .into_iter()
                .next()
                .ok_or_else(|| anyhow::anyhow!("no choices"))?;

            Ok(clean_message(choice.message))
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DashscopeRequest {
    model: String,
    input: DashscopeInput,
    parameters: DashscopeParameters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DashscopeInput {
    messages: Vec<DashscopeMessage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DashscopeMessage {
    role: Role,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    tool_call_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    tool_calls: Option<Vec<ToolCall>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DashscopeParameters {
    result_format: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    tools: Vec<ToolDefinition>,
}

#[derive(Debug, Clone, Deserialize)]
struct DashscopeResponse {
    output: DashscopeOutput,
}

#[derive(Debug, Clone, Deserialize)]
struct DashscopeOutput {
    #[serde(default)]
    choices: Vec<DashscopeChoice>,
}

#[derive(Debug, Clone, Deserialize)]
struct DashscopeChoice {
    message: ChatMessage,
}

fn map_messages(messages: &[ChatMessage]) -> Vec<DashscopeMessage> {
    messages
        .iter()
        .map(|m| DashscopeMessage {
            role: m.role,
            content: m.content.clone(),
            tool_call_id: m.tool_call_id.clone(),
            tool_calls: m.tool_calls.clone(),
        })
        .collect()
}

fn clean_message(mut m: ChatMessage) -> ChatMessage {
    if m.content
        .as_ref()
        .map(|s| s.trim().is_empty())
        .unwrap_or(false)
    {
        m.content = None;
    }
    if m.tool_calls.as_ref().map(|c| c.is_empty()).unwrap_or(false) {
        m.tool_calls = None;
    }
    m
}
