use crate::doctor::{CheckStatus, DoctorCheck};

fn push_unique(actions: &mut Vec<String>, action: impl Into<String>) {
    let action = action.into();
    if !actions.iter().any(|existing| existing == &action) {
        actions.push(action);
    }
}

fn find_check<'a>(checks: &'a [DoctorCheck], id: &str) -> Option<&'a DoctorCheck> {
    checks.iter().find(|check| check.id == id)
}

fn push_missing_core_paths_action(actions: &mut Vec<String>, checks: &[DoctorCheck]) {
    let missing_config = find_check(checks, "paths.config")
        .map(|check| check.status == CheckStatus::Warn)
        .unwrap_or(false);
    let missing_db = find_check(checks, "paths.db")
        .map(|check| check.status == CheckStatus::Warn)
        .unwrap_or(false);
    if missing_config || missing_db {
        push_unique(
            actions,
            "Run `loopforge init` to create `~/.loopforge/config.toml` and `~/.loopforge/loopforge.db`.",
        );
    }
}

fn push_config_parse_action(actions: &mut Vec<String>, checks: &[DoctorCheck]) {
    if let Some(check) = find_check(checks, "config.parse") {
        if check.status == CheckStatus::Error {
            push_unique(
                actions,
                format!(
                    "Fix `~/.loopforge/config.toml` so it parses cleanly, then rerun `loopforge doctor` ({})",
                    check.message
                ),
            );
        }
    }
}

fn push_router_action(actions: &mut Vec<String>, checks: &[DoctorCheck]) {
    let router_errors: Vec<String> = checks
        .iter()
        .filter(|check| check.id.starts_with("router.") && check.status == CheckStatus::Error)
        .map(|check| check.id.clone())
        .collect();
    if !router_errors.is_empty() {
        push_unique(
            actions,
            format!(
                "Update your `[router.*]` provider names in `~/.loopforge/config.toml` so they match defined providers (failing checks: {}).",
                router_errors.join(", ")
            ),
        );
    }
}

fn push_missing_provider_env_action(actions: &mut Vec<String>, checks: &[DoctorCheck]) {
    if let Some(check) = find_check(checks, "providers.api_keys") {
        if check.status == CheckStatus::Warn {
            push_unique(
                actions,
                format!(
                    "Export the missing provider credentials before rerunning LoopForge ({})",
                    check.message
                ),
            );
        }
    }
}

fn push_security_actions(actions: &mut Vec<String>, checks: &[DoctorCheck]) {
    if let Some(check) = find_check(checks, "security.leaks.mode") {
        if check.status == CheckStatus::Warn {
            push_unique(
                actions,
                format!(
                    "Set `[security.leaks].mode = \"redact\"` or `\"enforce\"` in `~/.loopforge/config.toml` to keep secret-like tool output out of follow-up model context and audits ({})",
                    check.message
                ),
            );
        }
    }

    if let Some(check) = find_check(checks, "security.egress.rules") {
        if check.status == CheckStatus::Warn {
            push_unique(
                actions,
                format!(
                    "Add `[security.egress.rules]` entries in `~/.loopforge/config.toml` to allow only the outbound hosts your workflows need ({})",
                    check.message
                ),
            );
        }
    }
}

fn push_runtime_dependency_actions(actions: &mut Vec<String>, checks: &[DoctorCheck]) {
    if let Some(check) = find_check(checks, "ollama.http") {
        if check.status != CheckStatus::Ok {
            push_unique(
                actions,
                "Start Ollama with `ollama serve`, verify the configured base URL, or switch `[router.*]` away from `ollama` if you are using another provider.".to_string(),
            );
        }
    }

    if let Some(check) = find_check(checks, "browser.cdp_http") {
        if check.status != CheckStatus::Ok {
            push_unique(
                actions,
                format!(
                    "Verify `LOOPFORGE_BROWSER_CDP_HTTP` points to a live Chromium DevTools endpoint ({})",
                    check.message
                ),
            );
        }
    }

    if let Some(check) = find_check(checks, "browser.chromium") {
        if check.status != CheckStatus::Ok {
            push_unique(
                actions,
                format!(
                    "Install a Chromium-based browser or set `LOOPFORGE_BROWSER_CHROME_PATH` / `LOOPFORGE_BROWSER_CDP_HTTP` ({})",
                    check.message
                ),
            );
        }
    }

    if let Some(check) = find_check(checks, "tools.git") {
        if check.status == CheckStatus::Error {
            push_unique(
                actions,
                format!(
                    "Install Git so LoopForge can work with repositories ({})",
                    check.message
                ),
            );
        }
    }
}

pub(super) fn derive_next_actions(checks: &[DoctorCheck]) -> Vec<String> {
    let mut actions: Vec<String> = Vec::new();
    push_missing_core_paths_action(&mut actions, checks);
    push_config_parse_action(&mut actions, checks);
    push_router_action(&mut actions, checks);
    push_missing_provider_env_action(&mut actions, checks);
    push_security_actions(&mut actions, checks);
    push_runtime_dependency_actions(&mut actions, checks);
    actions
}
