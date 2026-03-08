use serde::{Deserialize, Serialize};

use crate::openai_compat::{ChatMessage, Role, ToolCall, ToolDefinition};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct DashscopeRequest {
    pub(super) model: String,
    pub(super) input: DashscopeInput,
    pub(super) parameters: DashscopeParameters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct DashscopeInput {
    pub(super) messages: Vec<DashscopeMessage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct DashscopeMessage {
    pub(super) role: Role,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) content: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) tool_call_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) tool_calls: Option<Vec<ToolCall>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct DashscopeParameters {
    pub(super) result_format: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) temperature: Option<f32>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub(super) tools: Vec<ToolDefinition>,
}

#[derive(Debug, Clone, Deserialize)]
pub(super) struct DashscopeResponse {
    pub(super) output: DashscopeOutput,
}

#[derive(Debug, Clone, Deserialize)]
pub(super) struct DashscopeOutput {
    #[serde(default)]
    pub(super) choices: Vec<DashscopeChoice>,
}

#[derive(Debug, Clone, Deserialize)]
pub(super) struct DashscopeChoice {
    pub(super) message: ChatMessage,
}
