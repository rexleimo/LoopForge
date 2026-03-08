use anyhow::Context;

use super::parsing::parse_args;
use crate::records::{MemoryRecallToolArgs, MemoryStoreToolArgs};
use crate::AgentRuntime;

pub(super) fn dispatch_memory_tool(
    runtime: &AgentRuntime,
    tool_name: &str,
    args_json: &str,
) -> anyhow::Result<Option<String>> {
    let output = match tool_name {
        "memory_store" => {
            let args: MemoryStoreToolArgs = parse_args(args_json, tool_name)?;
            runtime
                .memory
                .kv_set(&args.key, &args.value)
                .context("memory_store kv_set")?;
            Some("ok".to_string())
        }
        "memory_recall" => {
            let args: MemoryRecallToolArgs = parse_args(args_json, tool_name)?;
            Some(
                runtime
                    .memory
                    .kv_get(&args.key)
                    .context("memory_recall kv_get")?
                    .unwrap_or_default(),
            )
        }
        _ => None,
    };
    Ok(output)
}
