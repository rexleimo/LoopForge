use crate::openai_compat::{ChatMessage, Role, ToolCall, ToolFunction};

use super::types::{AnthropicContentBlock, AnthropicResponse};

pub(super) fn map_response(response: AnthropicResponse) -> anyhow::Result<ChatMessage> {
    let mut texts = Vec::new();
    let mut tool_calls: Vec<ToolCall> = Vec::new();

    for block in response.content {
        match block {
            AnthropicContentBlock::Text { text } => {
                if !text.trim().is_empty() {
                    texts.push(text);
                }
            }
            AnthropicContentBlock::ToolUse { id, name, input } => {
                tool_calls.push(ToolCall {
                    id,
                    kind: "function".to_string(),
                    function: ToolFunction {
                        name,
                        arguments: serde_json::to_string(&input)?,
                    },
                });
            }
            AnthropicContentBlock::ToolResult { .. } => {}
        }
    }

    let content = if texts.is_empty() {
        None
    } else {
        Some(texts.join("\n"))
    };

    Ok(ChatMessage {
        role: Role::Assistant,
        content,
        name: None,
        tool_call_id: None,
        tool_calls: if tool_calls.is_empty() {
            None
        } else {
            Some(tool_calls)
        },
    })
}
