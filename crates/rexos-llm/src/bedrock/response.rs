use crate::openai_compat::{ChatMessage, Role, ToolCall, ToolFunction};
use aws_sdk_bedrockruntime::types::ContentBlock;

use super::document::document_to_json;

pub(super) fn map_response(
    response: aws_sdk_bedrockruntime::operation::converse::ConverseOutput,
) -> anyhow::Result<ChatMessage> {
    let output = response
        .output()
        .ok_or_else(|| anyhow::anyhow!("bedrock response missing output"))?;
    let message = output
        .as_message()
        .map_err(|_| anyhow::anyhow!("bedrock output is not a message"))?;

    let mut texts = Vec::new();
    let mut tool_calls = Vec::new();

    for block in message.content() {
        match block {
            ContentBlock::Text(text) => {
                if !text.trim().is_empty() {
                    texts.push(text.clone());
                }
            }
            ContentBlock::ToolUse(tool_use) => {
                tool_calls.push(ToolCall {
                    id: tool_use.tool_use_id().to_string(),
                    kind: "function".to_string(),
                    function: ToolFunction {
                        name: tool_use.name().to_string(),
                        arguments: serde_json::to_string(&document_to_json(tool_use.input()))?,
                    },
                });
            }
            _ => {}
        }
    }

    Ok(ChatMessage {
        role: Role::Assistant,
        content: if texts.is_empty() {
            None
        } else {
            Some(texts.join("\n"))
        },
        name: None,
        tool_call_id: None,
        tool_calls: if tool_calls.is_empty() {
            None
        } else {
            Some(tool_calls)
        },
    })
}
