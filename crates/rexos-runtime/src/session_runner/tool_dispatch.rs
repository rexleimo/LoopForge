mod agents_hands;
mod memory;
mod parsing;
mod tasks_scheduling;
mod workflow_knowledge;

use std::path::PathBuf;

use anyhow::Context;
use rexos_kernel::router::TaskKind;
use rexos_tools::Toolset;

use crate::AgentRuntime;

impl AgentRuntime {
    pub(super) async fn dispatch_runtime_tool_call(
        &self,
        workspace_root: &PathBuf,
        session_id: &str,
        kind: TaskKind,
        tools: &Toolset,
        tool_name: &str,
        args_json: &str,
    ) -> anyhow::Result<String> {
        if let Some(output) = memory::dispatch_memory_tool(self, tool_name, args_json)? {
            return Ok(output);
        }
        if let Some(output) =
            agents_hands::dispatch_agent_hand_tool(self, workspace_root, kind, tool_name, args_json)
                .await?
        {
            return Ok(output);
        }
        if let Some(output) =
            tasks_scheduling::dispatch_task_schedule_tool(self, session_id, tool_name, args_json)?
        {
            return Ok(output);
        }
        if let Some(output) = workflow_knowledge::dispatch_workflow_knowledge_tool(
            self,
            workspace_root,
            session_id,
            kind,
            tool_name,
            args_json,
        )
        .await?
        {
            return Ok(output);
        }

        tools
            .call(tool_name, args_json)
            .await
            .with_context(|| format!("tool {tool_name}"))
    }
}
