use std::path::Path;

use rexos_kernel::router::TaskKind;

use super::parsing::parse_args;
use crate::records::{
    KnowledgeAddEntityToolArgs, KnowledgeAddRelationToolArgs, KnowledgeQueryToolArgs,
    WorkflowRunToolArgs,
};
use crate::AgentRuntime;

pub(super) async fn dispatch_workflow_knowledge_tool(
    runtime: &AgentRuntime,
    workspace_root: &Path,
    session_id: &str,
    kind: TaskKind,
    tool_name: &str,
    args_json: &str,
) -> anyhow::Result<Option<String>> {
    let output = match tool_name {
        "workflow_run" => {
            let args: WorkflowRunToolArgs = parse_args(args_json, tool_name)?;
            Some(
                runtime
                    .workflow_run(workspace_root, session_id, kind, args)
                    .await
                    .map_err(|err| err.context("workflow_run"))?,
            )
        }
        "knowledge_add_entity" => {
            let args: KnowledgeAddEntityToolArgs = parse_args(args_json, tool_name)?;
            Some(
                runtime
                    .knowledge_add_entity(args)
                    .map_err(|err| err.context("knowledge_add_entity"))?,
            )
        }
        "knowledge_add_relation" => {
            let args: KnowledgeAddRelationToolArgs = parse_args(args_json, tool_name)?;
            Some(
                runtime
                    .knowledge_add_relation(args)
                    .map_err(|err| err.context("knowledge_add_relation"))?,
            )
        }
        "knowledge_query" => {
            let args: KnowledgeQueryToolArgs = parse_args(args_json, tool_name)?;
            Some(
                runtime
                    .knowledge_query(&args.query)
                    .map_err(|err| err.context("knowledge_query"))?,
            )
        }
        _ => None,
    };
    Ok(output)
}
