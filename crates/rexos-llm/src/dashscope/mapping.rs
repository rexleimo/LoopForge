use crate::openai_compat::ChatMessage;

use super::types::DashscopeMessage;

pub(super) fn map_messages(messages: &[ChatMessage]) -> Vec<DashscopeMessage> {
    messages
        .iter()
        .map(|message| DashscopeMessage {
            role: message.role,
            content: message.content.clone(),
            tool_call_id: message.tool_call_id.clone(),
            tool_calls: message.tool_calls.clone(),
        })
        .collect()
}

pub(super) fn clean_message(mut message: ChatMessage) -> ChatMessage {
    if message
        .content
        .as_ref()
        .map(|content| content.trim().is_empty())
        .unwrap_or(false)
    {
        message.content = None;
    }
    if message
        .tool_calls
        .as_ref()
        .map(|calls| calls.is_empty())
        .unwrap_or(false)
    {
        message.tool_calls = None;
    }
    message
}
