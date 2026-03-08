mod agent_flow;
mod bootstrap;
mod doctor_gate;
mod flow_types;
mod metrics;
mod model_probe;
mod outcome;
mod preflight;
mod report;
mod runtime;
mod starter;
mod types;

use std::path::PathBuf;

#[cfg(test)]
pub(crate) use metrics::onboard_events_path;
pub(crate) use metrics::{classify_onboard_failure, now_ms, record_onboard_attempt};
#[cfg(test)]
pub(crate) use model_probe::select_onboard_model;
pub(crate) use report::{print_onboard_report_summary, write_onboard_report};
#[cfg(test)]
pub(crate) use starter::starter_expected_artifact;
pub(crate) use starter::verify_onboard_artifact;
pub(crate) use starter::{build_onboard_starter_suggestions, resolve_onboard_prompt};
pub(crate) use types::{
    OnboardReportArtifact, OnboardStarter, OnboardStarterSuggestion, OnboardTaskReport,
};

pub(crate) async fn run(
    workspace: PathBuf,
    prompt: Option<String>,
    starter: OnboardStarter,
    skip_agent: bool,
    timeout_ms: u64,
) -> anyhow::Result<()> {
    let bootstrap = bootstrap::bootstrap_onboard(workspace, prompt.as_deref(), starter)?;
    let prepared = preflight::run_preflight(bootstrap, timeout_ms).await?;

    if skip_agent {
        agent_flow::emit_skipped_report(&prepared)?;
        println!("onboard done (skipped first agent run)");
        return Ok(());
    }

    agent_flow::run_first_agent_task(prepared, prompt.as_deref(), timeout_ms).await
}

#[cfg(test)]
mod tests;
