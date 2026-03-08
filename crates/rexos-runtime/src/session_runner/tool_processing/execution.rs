use std::collections::HashSet;
use std::path::PathBuf;

use rexos_kernel::router::TaskKind;
use rexos_llm::openai_compat::{ChatMessage, Role, ToolCall};
use rexos_tools::Toolset;

use super::outcome::finalize_tool_output;
use super::preflight::{
    emit_tool_approval_warning, emit_tool_started, enforce_session_tool_whitelist,
};
use crate::tool_calls::normalize_tool_arguments;
use crate::AgentRuntime;

pub(super) fn build_tool_chat_message(call: ToolCall, output: String) -> ChatMessage {
    ChatMessage {
        role: Role::Tool,
        content: Some(output),
        name: Some(call.function.name),
        tool_call_id: Some(call.id),
        tool_calls: None,
    }
}

impl AgentRuntime {
    pub(crate) async fn process_tool_call(
        &self,
        workspace_root: &PathBuf,
        session_id: &str,
        kind: TaskKind,
        allowed_lookup: Option<&HashSet<String>>,
        tools: &Toolset,
        call: ToolCall,
    ) -> anyhow::Result<ChatMessage> {
        let started_at = std::time::Instant::now();
        enforce_session_tool_whitelist(self, session_id, allowed_lookup, &call, &started_at)?;

        let args_json = normalize_tool_arguments(&call.function.name, &call.function.arguments);
        emit_tool_approval_warning(self, session_id, &call.function.name, &args_json)?;
        emit_tool_started(self, session_id, &call.function.name);

        let output_result = self
            .dispatch_runtime_tool_call(
                workspace_root,
                session_id,
                kind,
                tools,
                &call.function.name,
                &args_json,
            )
            .await;

        let duration_ms = started_at.elapsed().as_millis() as u64;
        let output = finalize_tool_output(
            self,
            session_id,
            &call.function.name,
            duration_ms,
            output_result,
        )?;

        Ok(build_tool_chat_message(call, output))
    }
}
