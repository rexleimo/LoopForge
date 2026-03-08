use serde::{Deserialize, Serialize};

use crate::openai_compat::{ChatMessage, Role, ToolCall, ToolDefinition};

#[derive(Debug, Clone, Serialize)]
pub(super) struct MiniMaxRequest {
    pub(super) model: String,
    pub(super) messages: Vec<ChatMessage>,
    pub(super) stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) temperature: Option<f32>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub(super) tools: Vec<ToolDefinition>,
    pub(super) tool_choice: String,
}

#[derive(Debug, Clone, Deserialize)]
pub(super) struct RawChatCompletionResponse {
    pub(super) choices: Vec<RawChoice>,
}

#[derive(Debug, Clone, Deserialize)]
pub(super) struct RawChoice {
    pub(super) message: RawChatMessage,
}

#[derive(Debug, Clone, Deserialize)]
pub(super) struct RawChatMessage {
    pub(super) role: Role,
    #[serde(default)]
    pub(super) content: Option<String>,
    #[serde(default)]
    pub(super) name: Option<String>,
    #[serde(default)]
    pub(super) tool_call_id: Option<String>,
    #[serde(default)]
    pub(super) tool_calls: Option<Vec<ToolCall>>,
    #[serde(default)]
    pub(super) function_call: Option<RawFunctionCall>,
}

#[derive(Debug, Clone, Deserialize)]
pub(super) struct RawFunctionCall {
    pub(super) name: String,
    pub(super) arguments: String,
}
