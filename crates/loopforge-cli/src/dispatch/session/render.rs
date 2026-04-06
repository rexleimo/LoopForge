use serde_json::Value;

pub(super) struct SessionPolicyView<'a> {
    pub(super) session_id: &'a str,
    pub(super) allowed_tools: &'a Option<Vec<String>>,
    pub(super) allowed_skills: &'a Option<Vec<String>>,
    pub(super) skill_policy: &'a rexos::agent::SessionSkillPolicy,
    pub(super) has_mcp_config: bool,
    pub(super) mcp_servers: Option<Vec<String>>,
    pub(super) mcp_config_sanitized: Option<Value>,
    pub(super) mcp_parse_error: Option<String>,
}

pub(super) fn print_session_policy(view: SessionPolicyView<'_>) -> anyhow::Result<()> {
    println!("Session policy");
    println!();
    println!("session_id: {}", view.session_id);
    match view.allowed_tools.as_ref() {
        Some(tools) => println!("allowed_tools: {}", tools.join(", ")),
        None => println!("allowed_tools: (none)"),
    }
    match view.allowed_skills.as_ref() {
        Some(skills) => println!("allowed_skills: {}", skills.join(", ")),
        None => println!("allowed_skills: (none)"),
    }
    println!(
        "skill_policy: allowlist={} require_approval={} auto_approve_readonly={}",
        view.skill_policy.allowlist.len(),
        view.skill_policy.require_approval,
        view.skill_policy.auto_approve_readonly
    );
    if !view.skill_policy.allowlist.is_empty() {
        println!(
            "skill_policy_allowlist: {}",
            view.skill_policy.allowlist.join(", ")
        );
    }
    println!();
    if !view.has_mcp_config {
        println!("mcp: (none)");
        return Ok(());
    }
    println!("mcp: present");
    if let Some(servers) = view.mcp_servers {
        if servers.is_empty() {
            println!("mcp_servers: (none)");
        } else {
            println!("mcp_servers: {}", servers.join(", "));
        }
    }
    if let Some(err) = view.mcp_parse_error {
        println!("mcp_config_parse_error: {err}");
    }
    if let Some(cfg) = view.mcp_config_sanitized {
        println!();
        println!("mcp_config (sanitized):");
        println!("{}", serde_json::to_string_pretty(&cfg)?);
    }
    Ok(())
}
