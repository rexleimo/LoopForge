use super::parsing::parse_args;
use crate::records::{
    ChannelSendToolArgs, CronCancelToolArgs, CronCreateToolArgs, EventPublishToolArgs,
    ScheduleCreateToolArgs, ScheduleDeleteToolArgs, TaskClaimToolArgs, TaskCompleteToolArgs,
    TaskListToolArgs, TaskPostToolArgs,
};
use crate::AgentRuntime;

pub(super) fn dispatch_task_schedule_tool(
    runtime: &AgentRuntime,
    session_id: &str,
    tool_name: &str,
    args_json: &str,
) -> anyhow::Result<Option<String>> {
    let output = match tool_name {
        "task_post" => {
            let args: TaskPostToolArgs = parse_args(args_json, tool_name)?;
            Some(
                runtime
                    .task_post(args)
                    .map_err(|err| err.context("task_post"))?,
            )
        }
        "task_list" => {
            let args: TaskListToolArgs = parse_args(args_json, tool_name)?;
            Some(
                runtime
                    .task_list(args.status.as_deref())
                    .map_err(|err| err.context("task_list"))?,
            )
        }
        "task_claim" => {
            let args: TaskClaimToolArgs = parse_args(args_json, tool_name)?;
            Some(
                runtime
                    .task_claim(args.agent_id.as_deref())
                    .map_err(|err| err.context("task_claim"))?,
            )
        }
        "task_complete" => {
            let args: TaskCompleteToolArgs = parse_args(args_json, tool_name)?;
            Some(
                runtime
                    .task_complete(&args.task_id, &args.result)
                    .map_err(|err| err.context("task_complete"))?,
            )
        }
        "event_publish" => {
            let args: EventPublishToolArgs = parse_args(args_json, tool_name)?;
            Some(
                runtime
                    .event_publish(args)
                    .map_err(|err| err.context("event_publish"))?,
            )
        }
        "schedule_create" => {
            let args: ScheduleCreateToolArgs = parse_args(args_json, tool_name)?;
            Some(
                runtime
                    .schedule_create(args)
                    .map_err(|err| err.context("schedule_create"))?,
            )
        }
        "schedule_list" => Some(
            runtime
                .schedule_list()
                .map_err(|err| err.context("schedule_list"))?,
        ),
        "schedule_delete" => {
            let args: ScheduleDeleteToolArgs = parse_args(args_json, tool_name)?;
            Some(
                runtime
                    .schedule_delete(&args.id)
                    .map_err(|err| err.context("schedule_delete"))?,
            )
        }
        "cron_create" => {
            let args: CronCreateToolArgs = parse_args(args_json, tool_name)?;
            Some(
                runtime
                    .cron_create(args)
                    .map_err(|err| err.context("cron_create"))?,
            )
        }
        "cron_list" => Some(
            runtime
                .cron_list()
                .map_err(|err| err.context("cron_list"))?,
        ),
        "cron_cancel" => {
            let args: CronCancelToolArgs = parse_args(args_json, tool_name)?;
            Some(
                runtime
                    .cron_cancel(&args.job_id)
                    .map_err(|err| err.context("cron_cancel"))?,
            )
        }
        "channel_send" => {
            let args: ChannelSendToolArgs = parse_args(args_json, tool_name)?;
            Some(
                runtime
                    .channel_send(Some(session_id), args)
                    .map_err(|err| err.context("channel_send"))?,
            )
        }
        _ => None,
    };
    Ok(output)
}
