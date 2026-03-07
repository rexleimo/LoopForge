use anyhow::{bail, Context};
use rexos_kernel::security::{EgressRule, SecurityConfig};

pub(crate) fn egress_rule_allows(
    tool_name: &str,
    method: &str,
    url: &reqwest::Url,
    config: &SecurityConfig,
) -> anyhow::Result<()> {
    if config.egress.rules.is_empty() {
        bail!("no egress rules configured");
    }

    let tool_name = tool_name.trim();
    let method = method.trim().to_ascii_uppercase();
    let host = url
        .host_str()
        .context("url missing host")?
        .to_ascii_lowercase();
    let path = normalized_path(url);

    let tool_rules: Vec<&EgressRule> = config
        .egress
        .rules
        .iter()
        .filter(|rule| rule_tool_matches(rule, tool_name))
        .collect();
    if tool_rules.is_empty() {
        bail!("no egress rule configured for tool: {tool_name}");
    }

    let host_rules: Vec<&EgressRule> = tool_rules
        .iter()
        .copied()
        .filter(|rule| rule.host.trim().eq_ignore_ascii_case(&host))
        .collect();
    if host_rules.is_empty() {
        bail!("egress host not allowed: {host}");
    }

    let path_rules: Vec<&EgressRule> = host_rules
        .iter()
        .copied()
        .filter(|rule| rule_path_matches(rule, &path))
        .collect();
    if path_rules.is_empty() {
        bail!("egress path not allowed for host {host}: {path}");
    }

    if path_rules
        .iter()
        .any(|rule| rule_method_matches(rule, &method))
    {
        return Ok(());
    }

    bail!("egress method not allowed for host {host}: {method}")
}

fn normalized_path(url: &reqwest::Url) -> String {
    let path = url.path();
    if path.is_empty() {
        "/".to_string()
    } else {
        path.to_string()
    }
}

fn rule_tool_matches(rule: &EgressRule, tool_name: &str) -> bool {
    let rule_tool = rule.tool.trim();
    rule_tool.is_empty() || rule_tool == "*" || rule_tool.eq_ignore_ascii_case(tool_name)
}

fn rule_path_matches(rule: &EgressRule, path: &str) -> bool {
    let prefix = rule.path_prefix.trim();
    prefix.is_empty() || path.starts_with(prefix)
}

fn rule_method_matches(rule: &EgressRule, method: &str) -> bool {
    if rule.methods.is_empty() {
        return true;
    }

    rule.methods
        .iter()
        .any(|allowed| allowed.trim().eq_ignore_ascii_case(method))
}
