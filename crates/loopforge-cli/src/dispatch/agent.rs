use crate::{cli::AgentCommand, runtime_env};

pub(super) async fn run(command: AgentCommand) -> anyhow::Result<()> {
    match command {
        AgentCommand::Run {
            workspace,
            mcp_config,
            prompt,
            system,
            session,
            kind,
            allowed_tools,
        } => {
            let (_paths, agent) = runtime_env::load_agent_runtime()?;

            let session_id = match session {
                Some(id) => id,
                None => rexos::harness::resolve_session_id(&workspace)?,
            };
            if !allowed_tools.is_empty() {
                agent.set_session_allowed_tools(&session_id, allowed_tools)?;
            }
            if let Some(path) = mcp_config.as_ref() {
                let raw = std::fs::read_to_string(path)?;
                let json: serde_json::Value =
                    serde_json::from_str(&raw).map_err(|err| anyhow::anyhow!("{err}"))?;
                agent.set_session_mcp_config(&session_id, serde_json::to_string(&json)?)?;
            }
            let out = agent
                .run_session(
                    workspace,
                    &session_id,
                    system.as_deref(),
                    &prompt,
                    kind.into(),
                )
                .await?;
            println!("{out}");
            eprintln!("[loopforge] session_id={session_id}");
            Ok(())
        }
    }
}
