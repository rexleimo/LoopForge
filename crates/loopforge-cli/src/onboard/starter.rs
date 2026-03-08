use std::path::Path;

use super::types::{OnboardStarter, OnboardStarterSuggestion};

pub(crate) fn resolve_onboard_prompt(prompt: Option<&str>, starter: OnboardStarter) -> String {
    prompt
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(|value| value.to_string())
        .unwrap_or_else(|| starter.default_prompt().to_string())
}

pub(crate) fn starter_expected_artifact(
    prompt: Option<&str>,
    starter: OnboardStarter,
) -> Option<&'static str> {
    let has_explicit_prompt = prompt
        .map(str::trim)
        .map(|value| !value.is_empty())
        .unwrap_or(false);
    if has_explicit_prompt {
        None
    } else {
        Some(starter.expected_artifact())
    }
}

pub(crate) fn verify_onboard_artifact(
    workspace: &Path,
    prompt: Option<&str>,
    starter: OnboardStarter,
) -> anyhow::Result<Option<String>> {
    let Some(relative_path) = starter_expected_artifact(prompt, starter) else {
        return Ok(None);
    };

    let artifact_path = workspace.join(relative_path);
    if artifact_path.is_file() {
        return Ok(Some(relative_path.to_string()));
    }

    anyhow::bail!(
        "starter `{}` did not create expected artifact `{}` in `{}`",
        starter.as_str(),
        relative_path,
        workspace.display()
    );
}

pub(crate) fn recommended_retry_command(
    workspace: &Path,
    session_id: &str,
    prompt: &str,
) -> String {
    format!(
        "loopforge agent run --workspace {} --session {} --prompt {}",
        shell_quote(&workspace.display().to_string()),
        shell_quote(session_id),
        shell_quote(prompt)
    )
}

pub(crate) fn recommended_next_command(workspace: &Path, success: bool) -> String {
    if success {
        format!(
            "loopforge agent run --workspace {} --prompt {}",
            shell_quote(&workspace.display().to_string()),
            shell_quote(
                "Continue from the current workspace and write notes/next-steps.md with 3 follow-up actions."
            )
        )
    } else {
        "loopforge doctor".to_string()
    }
}

pub(crate) fn build_onboard_starter_suggestions(workspace: &Path) -> Vec<OnboardStarterSuggestion> {
    [
        OnboardStarter::Hello,
        OnboardStarter::WorkspaceBrief,
        OnboardStarter::RepoOnboarding,
    ]
    .into_iter()
    .map(|starter| OnboardStarterSuggestion {
        starter: starter.as_str().to_string(),
        description: starter.description().to_string(),
        command: format!(
            "loopforge onboard --workspace {} --starter {}",
            shell_quote(&workspace.display().to_string()),
            starter.as_str()
        ),
    })
    .collect()
}

pub(crate) fn shell_quote(value: &str) -> String {
    format!("'{}'", value.replace('\'', "'\"'\"'"))
}
