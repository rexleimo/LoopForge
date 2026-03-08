use std::path::{Path, PathBuf};

use anyhow::Context;

use super::OnboardReportArtifact;

fn onboard_report_paths(workspace: &Path) -> (PathBuf, PathBuf) {
    let dir = workspace.join(".loopforge");
    (
        dir.join("onboard-report.json"),
        dir.join("onboard-report.md"),
    )
}

fn render_onboard_report_markdown(report: &OnboardReportArtifact) -> String {
    let mut lines = vec![
        "# LoopForge Onboard Report".to_string(),
        "".to_string(),
        format!("- Workspace: `{}`", report.workspace),
        format!("- Starter: `{}`", report.starter),
        format!("- Config valid: {}", report.config_valid),
        format!(
            "- Doctor summary: ok={} warn={} error={}",
            report.doctor_summary.ok, report.doctor_summary.warn, report.doctor_summary.error
        ),
        format!("- First task status: `{}`", report.task.status),
        format!(
            "- Recommended next command: `{}`",
            report.recommended_next_command
        ),
        "".to_string(),
        "## Effective Prompt".to_string(),
        "".to_string(),
        "```text".to_string(),
        report.effective_prompt.clone(),
        "```".to_string(),
        "".to_string(),
    ];

    if !report.doctor_next_actions.is_empty() {
        lines.push("## Suggested Next Steps".to_string());
        lines.push("".to_string());
        for action in &report.doctor_next_actions {
            lines.push(format!("- {}", action));
        }
        lines.push("".to_string());
    }

    lines.push("## Starter Suggestions".to_string());
    lines.push("".to_string());
    for suggestion in &report.starter_suggestions {
        lines.push(format!(
            "- `{}` — {}",
            suggestion.command, suggestion.description
        ));
    }
    lines.push("".to_string());

    if let Some(session_id) = &report.task.session_id {
        lines.push("## Session".to_string());
        lines.push("".to_string());
        lines.push(format!("- Session ID: `{}`", session_id));
        lines.push("".to_string());
    }

    if let Some(error) = &report.task.error {
        lines.push("## Error".to_string());
        lines.push("".to_string());
        lines.push("```text".to_string());
        lines.push(error.clone());
        lines.push("```".to_string());
        lines.push("".to_string());
    }

    lines.join("\n")
}

pub(crate) fn write_onboard_report(
    workspace: &Path,
    report: &OnboardReportArtifact,
) -> anyhow::Result<(PathBuf, PathBuf)> {
    let (json_path, md_path) = onboard_report_paths(workspace);
    if let Some(parent) = json_path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("create onboard report dir: {}", parent.display()))?;
    }
    std::fs::write(&json_path, serde_json::to_string_pretty(report)? + "\n")
        .with_context(|| format!("write {}", json_path.display()))?;
    std::fs::write(&md_path, render_onboard_report_markdown(report) + "\n")
        .with_context(|| format!("write {}", md_path.display()))?;
    Ok((json_path, md_path))
}

pub(crate) fn print_onboard_report_summary(
    report: &OnboardReportArtifact,
    json_path: &Path,
    md_path: &Path,
) {
    println!("onboard summary:");
    println!(
        "- doctor: ok={} warn={} error={}",
        report.doctor_summary.ok, report.doctor_summary.warn, report.doctor_summary.error
    );
    println!("- task status: {}", report.task.status);
    println!("- report json: {}", json_path.display());
    println!("- report md: {}", md_path.display());
    println!("- next command: {}", report.recommended_next_command);
}
