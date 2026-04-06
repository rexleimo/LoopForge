use anyhow::Context;

use crate::{cli::SessionCommand, runtime_env};

mod json;
mod mcp;
mod render;
#[cfg(test)]
mod tests;

pub(super) async fn run(command: SessionCommand) -> anyhow::Result<()> {
    match command {
        SessionCommand::Policy {
            workspace,
            session,
            show_mcp_config,
            json,
        } => {
            let (_paths, agent) = runtime_env::load_agent_runtime()?;

            let session_id = match session {
                Some(id) => id,
                None => rexos::harness::resolve_session_id(&workspace)?,
            };

            let snapshot = agent
                .load_session_policy_snapshot(&session_id)
                .with_context(|| format!("load session policy snapshot: {session_id}"))?;

            if json {
                let out = json::build_session_policy_json(
                    session_id,
                    &snapshot.allowed_tools,
                    &snapshot.allowed_skills,
                    &snapshot.skill_policy,
                    snapshot.mcp_config_json.as_deref(),
                    show_mcp_config,
                )?;
                println!("{}", serde_json::to_string_pretty(&out)?);
                return Ok(());
            }

            let (mcp_servers, mcp_config_sanitized, mcp_parse_error) =
                mcp::summarize_mcp_config(snapshot.mcp_config_json.as_deref(), show_mcp_config);

            render::print_session_policy(render::SessionPolicyView {
                session_id: &session_id,
                allowed_tools: &snapshot.allowed_tools,
                allowed_skills: &snapshot.allowed_skills,
                skill_policy: &snapshot.skill_policy,
                has_mcp_config: snapshot.mcp_config_json.is_some(),
                mcp_servers,
                mcp_config_sanitized,
                mcp_parse_error,
            })
        }
    }
}
