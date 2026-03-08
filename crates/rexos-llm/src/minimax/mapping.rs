use crate::openai_compat::{ChatMessage, ToolCall, ToolFunction};

use super::types::RawChatMessage;

pub(super) fn map_message(raw: RawChatMessage) -> anyhow::Result<ChatMessage> {
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

    Ok(ChatMessage {
        role: raw.role,
        content: raw.content,
        name: raw.name,
        tool_call_id: raw.tool_call_id,
        tool_calls,
    })
}
