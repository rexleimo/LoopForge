use std::path::PathBuf;

use anyhow::Context;
use rexos::config::RexosConfig;

use crate::{cli::AgentKind, runtime_env, skills};

pub(super) async fn run_skill(
    name: String,
    workspace: PathBuf,
    input: String,
    session: Option<String>,
    kind: AgentKind,
) -> anyhow::Result<()> {
    let (paths, cfg) = runtime_env::load_runtime_config()?;
    let skills_cfg = RexosConfig::load_skills_config(&paths).unwrap_or_default();

    std::fs::create_dir_all(&workspace)
        .with_context(|| format!("create workspace: {}", workspace.display()))?;

    let skill = skills::find_skill(&workspace, &name)?;
    let skill_entry = skills::read_skill_entry(&skill)?;

    let agent = runtime_env::build_agent_runtime(&paths, cfg)?;

    let session_id = match session {
        Some(id) => id,
        None => rexos::harness::resolve_session_id(&workspace)?,
    };
    let experimental_mode = skills_cfg.experimental;
    agent.set_session_skill_policy(
        &session_id,
        rexos::agent::SessionSkillPolicy {
            allowlist: skills_cfg.allowlist,
            require_approval: skills_cfg.require_approval,
            auto_approve_readonly: skills_cfg.auto_approve_readonly,
        },
    )?;
    if experimental_mode {
        eprintln!("skills: experimental mode is enabled in config");
    }

    agent.record_skill_discovered(
        &session_id,
        &skill.name,
        skills::source_name(skill.source),
        &skill.manifest.version.to_string(),
    )?;
    agent.authorize_skill(&session_id, &skill.name, &skill.manifest.permissions)?;

    let allowed_tools = skills::permission_tools(&skill.manifest.permissions);
    if !allowed_tools.is_empty() {
        agent.set_session_allowed_tools(&session_id, allowed_tools)?;
    }

    let system = format!(
        "You are executing skill `{}` version {}.\n\
Follow the skill instructions exactly.\n\
If tool permissions are restricted, do not call tools outside the granted scope.\n\n\
--- SKILL INSTRUCTIONS START ---\n{}\n--- SKILL INSTRUCTIONS END ---",
        skill.name, skill.manifest.version, skill_entry
    );

    let out = match agent
        .run_session(workspace, &session_id, Some(&system), &input, kind.into())
        .await
    {
        Ok(out) => {
            agent.record_skill_execution(
                &session_id,
                &skill.name,
                &skill.manifest.permissions,
                true,
                None,
            )?;
            out
        }
        Err(err) => {
            let err_text = err.to_string();
            let _ = agent.record_skill_execution(
                &session_id,
                &skill.name,
                &skill.manifest.permissions,
                false,
                Some(&err_text),
            );
            return Err(err);
        }
    };

    println!("{out}");
    eprintln!("[loopforge] session_id={session_id}");
    Ok(())
}
