use anyhow::Context;
use serde::Deserialize;

use super::types::{ChatMessage, Role, ToolCall, ToolFunction};

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

pub(super) async fn decode_chat_completion_response(
    response: reqwest::Response,
) -> anyhow::Result<ChatMessage> {
    let body: RawChatCompletionResponse = response
        .json()
        .await
        .context("decode chat completion response")?;

    let choice = body.choices.into_iter().next().context("no choices")?;
    Ok(map_raw_message(choice.message))
}

fn map_raw_message(raw: RawChatMessage) -> ChatMessage {
    let mut tool_calls = raw.tool_calls;
    if tool_calls
        .as_ref()
        .map(|calls| calls.is_empty())
        .unwrap_or(true)
    {
        if let Some(function_call) = raw.function_call {
            tool_calls = Some(vec![ToolCall {
                id: "call_1".to_string(),
                kind: "function".to_string(),
                function: ToolFunction {
                    name: function_call.name,
                    arguments: function_call.arguments,
                },
            }]);
        }
    }

    ChatMessage {
        role: raw.role,
        content: raw.content,
        name: raw.name,
        tool_call_id: raw.tool_call_id,
        tool_calls,
    }
}
