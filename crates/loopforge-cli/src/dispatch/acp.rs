use crate::{
    acp::{load_acp_checkpoints, load_acp_events},
    cli::AcpCommand,
    runtime_env,
};

pub(super) fn run(command: AcpCommand) -> anyhow::Result<()> {
    match command {
        AcpCommand::Events {
            session,
            limit,
            json,
        } => {
            let paths = runtime_env::ensure_paths()?;
            let memory = runtime_env::open_memory(&paths)?;

            let events = load_acp_events(&memory, session.as_deref(), limit)?;
            if json {
                println!("{}", serde_json::to_string_pretty(&events)?);
            } else {
                for event in events {
                    let session = event
                        .get("session_id")
                        .and_then(|value| value.as_str())
                        .unwrap_or("-");
                    let event_type = event
                        .get("event_type")
                        .and_then(|value| value.as_str())
                        .unwrap_or("unknown");
                    let created_at = event
                        .get("created_at")
                        .and_then(|value| value.as_i64())
                        .unwrap_or(0);
                    println!("[{created_at}] session={session} type={event_type}");
                }
            }
            Ok(())
        }
        AcpCommand::Checkpoints { session, json } => {
            let paths = runtime_env::ensure_paths()?;
            let memory = runtime_env::open_memory(&paths)?;

            let checkpoints = load_acp_checkpoints(&memory, &session)?;
            if json {
                println!("{}", serde_json::to_string_pretty(&checkpoints)?);
            } else if checkpoints.is_empty() {
                println!("no checkpoints for session {}", session);
            } else {
                for checkpoint in checkpoints {
                    let channel = checkpoint
                        .get("channel")
                        .and_then(|value| value.as_str())
                        .unwrap_or("-");
                    let cursor = checkpoint
                        .get("cursor")
                        .and_then(|value| value.as_str())
                        .unwrap_or("-");
                    let updated_at = checkpoint
                        .get("updated_at")
                        .and_then(|value| value.as_i64())
                        .unwrap_or(0);
                    println!("[{updated_at}] channel={channel} cursor={cursor}");
                }
            }
            Ok(())
        }
    }
}
