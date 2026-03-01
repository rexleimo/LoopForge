use std::time::Duration;

use anyhow::Context;
use serde::Deserialize;
use serde::Serialize;

use crate::llm::driver::{ChatFuture, LlmDriver};
use crate::llm::openai_compat::{
    ChatCompletionRequest, ChatMessage, Role, ToolCall, ToolDefinition, ToolFunction,
};

#[derive(Debug, Clone)]
pub struct MiniMaxDriver {
    base_url: String,
    api_key: Option<String>,
    http: reqwest::Client,
}

impl MiniMaxDriver {
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

impl LlmDriver for MiniMaxDriver {
    fn chat<'a>(&'a self, req: ChatCompletionRequest) -> ChatFuture<'a> {
        Box::pin(async move {
            let tool_choice = if req.tools.is_empty() {
                "none".to_string()
            } else {
                "auto".to_string()
            };

            let mm_req = MiniMaxRequest {
                model: req.model,
                messages: req.messages,
                stream: false,
                temperature: req.temperature,
                tools: req.tools,
                tool_choice,
            };

            let url = format!("{}/text/chatcompletion_v2", self.base_url);
            let mut http_req = self.http.post(url).json(&mm_req);
            if let Some(key) = &self.api_key {
                if !key.trim().is_empty() {
                    http_req = http_req.bearer_auth(key);
                }
            }

            let resp = http_req
                .send()
                .await
                .context("send minimax request")?
                .error_for_status()
                .context("minimax HTTP error")?;

            let body: RawChatCompletionResponse =
                resp.json().await.context("decode minimax response")?;
            let choice = body.choices.into_iter().next().context("no choices")?;
            Ok(map_message(choice.message)?)
        })
    }
}

#[derive(Debug, Clone, Serialize)]
struct MiniMaxRequest {
    model: String,
    messages: Vec<ChatMessage>,
    stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    tools: Vec<ToolDefinition>,
    tool_choice: String,
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

