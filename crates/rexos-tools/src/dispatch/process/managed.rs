use crate::defs::{ProcessKillArgs, ProcessPollArgs, ProcessStartArgs, ProcessWriteArgs};
use crate::Toolset;

pub(super) async fn dispatch(
    toolset: &Toolset,
    name: &str,
    arguments_json: &str,
) -> anyhow::Result<String> {
    match name {
        "process_start" => {
            let args: ProcessStartArgs = super::super::parse_args(arguments_json, name)?;
            toolset.process_start(&args.command, &args.args).await
        }
        "process_poll" => {
            let args: ProcessPollArgs = super::super::parse_args(arguments_json, name)?;
            toolset.process_poll(&args.process_id).await
        }
        "process_write" => {
            let args: ProcessWriteArgs = super::super::parse_args(arguments_json, name)?;
            toolset.process_write(&args.process_id, &args.data).await
        }
        "process_kill" => {
            let args: ProcessKillArgs = super::super::parse_args(arguments_json, name)?;
            toolset.process_kill(&args.process_id).await
        }
        _ => unreachable!("unexpected managed process tool: {name}"),
    }
}
