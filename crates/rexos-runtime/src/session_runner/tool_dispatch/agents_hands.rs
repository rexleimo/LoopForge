use std::path::Path;

use rexos_kernel::router::TaskKind;

use super::parsing::parse_args;
use crate::records::{
    AgentFindToolArgs, AgentKillToolArgs, AgentSendToolArgs, AgentSpawnToolArgs,
    HandActivateToolArgs, HandDeactivateToolArgs, HandStatusToolArgs,
};
use crate::AgentRuntime;

pub(super) async fn dispatch_agent_hand_tool(
    runtime: &AgentRuntime,
    workspace_root: &Path,
    kind: TaskKind,
    tool_name: &str,
    args_json: &str,
) -> anyhow::Result<Option<String>> {
    let output = match tool_name {
        "agent_spawn" => {
            let args: AgentSpawnToolArgs = parse_args(args_json, tool_name)?;
            Some(
                runtime
                    .agent_spawn(args)
                    .map_err(|err| err.context("agent_spawn"))?,
            )
        }
        "agent_list" => Some(
            runtime
                .agent_list()
                .map_err(|err| err.context("agent_list"))?,
        ),
        "agent_find" => {
            let args: AgentFindToolArgs = parse_args(args_json, tool_name)?;
            Some(
                runtime
                    .agent_find(&args.query)
                    .map_err(|err| err.context("agent_find"))?,
            )
        }
        "agent_kill" => {
            let args: AgentKillToolArgs = parse_args(args_json, tool_name)?;
            Some(
                runtime
                    .agent_kill(&args.agent_id)
                    .map_err(|err| err.context("agent_kill"))?,
            )
        }
        "agent_send" => {
            let args: AgentSendToolArgs = parse_args(args_json, tool_name)?;
            Some(
                runtime
                    .agent_send(workspace_root.to_path_buf(), kind, args)
                    .await
                    .map_err(|err| err.context("agent_send"))?,
            )
        }
        "hand_list" => Some(
            runtime
                .hand_list()
                .map_err(|err| err.context("hand_list"))?,
        ),
        "hand_activate" => {
            let args: HandActivateToolArgs = parse_args(args_json, tool_name)?;
            Some(
                runtime
                    .hand_activate(args)
                    .map_err(|err| err.context("hand_activate"))?,
            )
        }
        "hand_status" => {
            let args: HandStatusToolArgs = parse_args(args_json, tool_name)?;
            Some(
                runtime
                    .hand_status(&args.hand_id)
                    .map_err(|err| err.context("hand_status"))?,
            )
        }
        "hand_deactivate" => {
            let args: HandDeactivateToolArgs = parse_args(args_json, tool_name)?;
            Some(
                runtime
                    .hand_deactivate(&args.instance_id)
                    .map_err(|err| err.context("hand_deactivate"))?,
            )
        }
        _ => None,
    };
    Ok(output)
}
