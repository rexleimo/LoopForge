use anyhow::Context;
use rexos_llm::openai_compat::{ChatMessage, Role};

use crate::AgentRuntime;

pub(super) fn initialize_session_messages(
    runtime: &AgentRuntime,
    session_id: &str,
    system_prompt: Option<&str>,
    user_prompt: &str,
) -> anyhow::Result<Vec<ChatMessage>> {
    let mut messages = runtime
        .memory
        .list_chat_messages(session_id)
        .context("load session history")?;

    if let Some(system_prompt) = system_prompt {
        let has_system = messages.iter().any(|message| message.role == Role::System);
        if !has_system {
            let system_msg = ChatMessage {
                role: Role::System,
                content: Some(system_prompt.to_string()),
                name: None,
                tool_call_id: None,
                tool_calls: None,
            };
            runtime
                .memory
                .append_chat_message(session_id, &system_msg)?;
            messages.push(system_msg);
        }
    }

    let user_msg = ChatMessage {
        role: Role::User,
        content: Some(user_prompt.to_string()),
        name: None,
        tool_call_id: None,
        tool_calls: None,
    };
    runtime.memory.append_chat_message(session_id, &user_msg)?;
    messages.push(user_msg);

    Ok(messages)
}
