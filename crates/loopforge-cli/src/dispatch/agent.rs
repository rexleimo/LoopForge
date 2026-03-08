use crate::{cli::AgentCommand, runtime_env};

pub(super) async fn run(command: AgentCommand) -> anyhow::Result<()> {
    match command {
        AgentCommand::Run {
            workspace,
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
