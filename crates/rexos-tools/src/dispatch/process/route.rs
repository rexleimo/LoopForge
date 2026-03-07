use crate::Toolset;

use super::classify::{dispatch_kind, ProcessDispatchKind};

pub(super) async fn dispatch_process_tool(
    tools: &Toolset,
    name: &str,
    arguments_json: &str,
) -> anyhow::Result<String> {
    match dispatch_kind(name) {
        Some(ProcessDispatchKind::Exec) => super::exec::dispatch(tools, name, arguments_json).await,
        Some(ProcessDispatchKind::Managed) => {
            super::managed::dispatch(tools, name, arguments_json).await
        }
        Some(ProcessDispatchKind::List) => super::list::dispatch(tools, name, arguments_json).await,
        None => unreachable!("unexpected process tool: {name}"),
    }
}
