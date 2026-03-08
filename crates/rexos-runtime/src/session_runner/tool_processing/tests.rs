use rexos_kernel::router::TaskKind;
use rexos_llm::openai_compat::{ToolCall, ToolFunction};

use super::events::session_kind_label;
use super::execution::build_tool_chat_message;

#[test]
fn session_kind_label_uses_lowercase_debug_name() {
    assert_eq!(session_kind_label(TaskKind::Coding), "coding");
    assert_eq!(session_kind_label(TaskKind::Planning), "planning");
}

#[test]
fn build_tool_chat_message_keeps_tool_name_and_call_id() {
    let call = ToolCall {
        id: "call_123".to_string(),
        kind: "function".to_string(),
        function: ToolFunction {
            name: "fs_read".to_string(),
            arguments: "{\"path\":\"README.md\"}".to_string(),
        },
    };

    let message = build_tool_chat_message(call, "hello".to_string());
    assert_eq!(message.name.as_deref(), Some("fs_read"));
    assert_eq!(message.tool_call_id.as_deref(), Some("call_123"));
    assert_eq!(message.content.as_deref(), Some("hello"));
}
