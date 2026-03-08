use std::path::Path;

use rexos::paths::RexosPaths;

use crate::doctor;

use super::{
    build_onboard_starter_suggestions, now_ms, print_onboard_report_summary,
    record_onboard_attempt, write_onboard_report, OnboardReportArtifact, OnboardStarter,
    OnboardStarterSuggestion, OnboardTaskReport,
};

pub(crate) struct OnboardReportBase {
    workspace: String,
    config_path: String,
    config_valid: bool,
    starter: String,
    effective_prompt: String,
    doctor_summary: doctor::DoctorSummary,
    doctor_next_actions: Vec<String>,
    starter_suggestions: Vec<OnboardStarterSuggestion>,
}

impl OnboardReportBase {
    pub(crate) fn new(
        workspace: &Path,
        config_path: String,
        config_valid: bool,
        starter: OnboardStarter,
        effective_prompt: &str,
        doctor_summary: doctor::DoctorSummary,
        doctor_next_actions: Vec<String>,
    ) -> Self {
        Self {
            workspace: workspace.display().to_string(),
            config_path,
            config_valid,
            starter: starter.as_str().to_string(),
            effective_prompt: effective_prompt.to_string(),
            doctor_summary,
            doctor_next_actions,
            starter_suggestions: build_onboard_starter_suggestions(workspace),
        }
    }

    pub(crate) fn build_report(
        &self,
        task: OnboardTaskReport,
        recommended_next_command: String,
    ) -> OnboardReportArtifact {
        OnboardReportArtifact {
            generated_at_ms: now_ms(),
            workspace: self.workspace.clone(),
            config_path: self.config_path.clone(),
            config_valid: self.config_valid,
            starter: self.starter.clone(),
            effective_prompt: self.effective_prompt.clone(),
            doctor_summary: self.doctor_summary.clone(),
            doctor_next_actions: self.doctor_next_actions.clone(),
            task,
            recommended_next_command,
            starter_suggestions: self.starter_suggestions.clone(),
        }
    }
}

pub(crate) fn emit_onboard_report(
    workspace: &Path,
    report: &OnboardReportArtifact,
) -> anyhow::Result<()> {
    let (json_path, md_path) = write_onboard_report(workspace, report)?;
    print_onboard_report_summary(report, &json_path, &md_path);
    Ok(())
}

pub(crate) fn record_and_print_onboard_attempt(
    paths: &RexosPaths,
    workspace: &Path,
    session_id: &str,
    success: bool,
    failure_category: Option<&str>,
    error: Option<&str>,
) {
    match record_onboard_attempt(
        paths,
        workspace,
        session_id,
        success,
        failure_category,
        error,
    ) {
        Ok(metrics) => {
            eprintln!(
                "onboard metrics: success_rate={}/{}",
                metrics.first_task_success, metrics.attempted_first_task
            );
            eprintln!(
                "onboard metrics path: {}",
                paths.base_dir.join("onboard-metrics.json").display()
            );
            eprintln!(
                "onboard events path: {}",
                paths.base_dir.join("onboard-events.jsonl").display()
            );
        }
        Err(log_err) => {
            eprintln!("onboard: failed to persist metrics: {log_err}");
        }
    }
}
