use crate::openai_compat::{ChatMessage, Role, ToolCall, ToolFunction};

use super::types::{GeminiPart, GeminiResponse};

pub(super) fn map_response(response: GeminiResponse) -> anyhow::Result<ChatMessage> {
    let first = response
        .candidates
        .into_iter()
        .next()
        .ok_or_else(|| anyhow::anyhow!("no candidates"))?;

    let mut texts = Vec::new();
    let mut tool_calls = Vec::new();

    for (idx, part) in first.content.parts.into_iter().enumerate() {
        match part {
            GeminiPart::Text { text } => {
                if !text.trim().is_empty() {
                    texts.push(text);
                }
            }
            GeminiPart::FunctionCall { function_call } => {
                tool_calls.push(ToolCall {
                    id: format!("call_{}", idx + 1),
                    kind: "function".to_string(),
                    function: ToolFunction {
                        name: function_call.name,
                        arguments: serde_json::to_string(&function_call.args)?,
                    },
                });
            }
            GeminiPart::FunctionResponse { .. } => {}
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
