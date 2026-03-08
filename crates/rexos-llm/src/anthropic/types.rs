use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct AnthropicRequest {
    pub(super) model: String,
    pub(super) max_tokens: u32,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(super) system: String,
    pub(super) messages: Vec<AnthropicMessage>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub(super) tools: Vec<AnthropicTool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct AnthropicMessage {
    pub(super) role: String,
    pub(super) content: Vec<AnthropicContentBlock>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub(super) enum AnthropicContentBlock {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "tool_use")]
    ToolUse {
        id: String,
        name: String,
        #[serde(default)]
        input: serde_json::Value,
    },
    #[serde(rename = "tool_result")]
    ToolResult {
        tool_use_id: String,
        content: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        is_error: Option<bool>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct AnthropicTool {
    pub(super) name: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(super) description: String,
    #[serde(rename = "input_schema")]
    pub(super) input_schema: serde_json::Value,
}

#[derive(Debug, Clone, Deserialize)]
pub(super) struct AnthropicResponse {
    #[serde(default)]
    pub(super) content: Vec<AnthropicContentBlock>,
}
