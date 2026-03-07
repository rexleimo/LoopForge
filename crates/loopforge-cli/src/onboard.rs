use anyhow::Context;
use std::collections::BTreeMap;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use rexos::{
    config::{ProviderKind, RexosConfig},
    memory::MemoryStore,
    paths::RexosPaths,
};

use crate::{config_validation::validate_config, doctor};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
struct OnboardMetrics {
    attempted_first_task: u64,
    first_task_success: u64,
    first_task_failed: u64,
    failure_by_category: BTreeMap<String, u64>,
    updated_at_ms: i64,
}

#[derive(Debug, Clone, serde::Serialize)]
struct OnboardEvent {
    ts_ms: i64,
    workspace: String,
    session_id: String,
    outcome: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    failure_category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

#[derive(Debug, Clone, Copy, clap::ValueEnum, serde::Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum OnboardStarter {
    Hello,
    WorkspaceBrief,
    RepoOnboarding,
}

impl OnboardStarter {
    fn as_str(self) -> &'static str {
        match self {
            OnboardStarter::Hello => "hello",
            OnboardStarter::WorkspaceBrief => "workspace-brief",
            OnboardStarter::RepoOnboarding => "repo-onboarding",
        }
    }

    fn default_prompt(self) -> &'static str {
        match self {
            OnboardStarter::Hello => "Create hello.txt with the word hi",
            OnboardStarter::WorkspaceBrief => {
                "Create notes/workspace-brief.md with: what this workspace is for, 3 risks, and 3 next actions."
            }
            OnboardStarter::RepoOnboarding => {
                "Read README.md plus the most important project metadata files you can find. Create notes/repo-onboarding.md with: project purpose, how to run it, what to verify first, and 3 next actions."
            }
        }
    }

    fn description(self) -> &'static str {
        match self {
            OnboardStarter::Hello => "Minimal smoke check that proves file creation works.",
            OnboardStarter::WorkspaceBrief => {
                "Create a short workspace brief with risks and next actions."
            }
            OnboardStarter::RepoOnboarding => {
                "Read the repo and produce an onboarding brief for real project work."
            }
        }
    }

    fn expected_artifact(self) -> &'static str {
        match self {
            OnboardStarter::Hello => "hello.txt",
            OnboardStarter::WorkspaceBrief => "notes/workspace-brief.md",
            OnboardStarter::RepoOnboarding => "notes/repo-onboarding.md",
        }
    }
}

#[derive(Debug, Clone, serde::Serialize)]
struct OnboardStarterSuggestion {
    starter: String,
    description: String,
    command: String,
}

#[derive(Debug, Clone, serde::Serialize)]
struct OnboardTaskReport {
    status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    session_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    failure_category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize)]
struct OnboardReportArtifact {
    generated_at_ms: i64,
    workspace: String,
    config_path: String,
    config_valid: bool,
    starter: String,
    effective_prompt: String,
    doctor_summary: doctor::DoctorSummary,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    doctor_next_actions: Vec<String>,
    task: OnboardTaskReport,
    recommended_next_command: String,
    starter_suggestions: Vec<OnboardStarterSuggestion>,
}

pub(crate) async fn run(
    workspace: PathBuf,
    prompt: Option<String>,
    starter: OnboardStarter,
    skip_agent: bool,
    timeout_ms: u64,
) -> anyhow::Result<()> {
    let paths = RexosPaths::discover()?;
    paths.ensure_dirs()?;
    RexosConfig::ensure_default(&paths)?;
    MemoryStore::open_or_create(&paths)?;
    println!("Initialized {}", paths.base_dir.display());

    std::fs::create_dir_all(&workspace)
        .with_context(|| format!("create workspace: {}", workspace.display()))?;
    println!("workspace ready: {}", workspace.display());

    let effective_prompt = resolve_onboard_prompt(prompt.as_deref(), starter);
    let config_report = validate_config(&paths);
    if !config_report.valid {
        println!("config invalid: {}", config_report.config_path);
        for err in &config_report.errors {
            println!("- {err}");
        }
        let report = OnboardReportArtifact {
            generated_at_ms: now_ms(),
            workspace: workspace.display().to_string(),
            config_path: config_report.config_path.clone(),
            config_valid: false,
            starter: starter.as_str().to_string(),
            effective_prompt: effective_prompt.clone(),
            doctor_summary: doctor::DoctorSummary {
                ok: 0,
                warn: 0,
                error: 1,
            },
            doctor_next_actions: vec![format!(
                "Fix `{}` and rerun `loopforge config validate`.",
                config_report.config_path
            )],
            task: OnboardTaskReport {
                status: "blocked".to_string(),
                session_id: None,
                failure_category: Some("config_invalid".to_string()),
                error: Some(config_report.errors.join("; ")),
            },
            recommended_next_command: "loopforge config validate".to_string(),
            starter_suggestions: build_onboard_starter_suggestions(&workspace),
        };
        let (json_path, md_path) = write_onboard_report(&workspace, &report)?;
        print_onboard_report_summary(&report, &json_path, &md_path);
        std::process::exit(1);
    }
    println!("config valid: {}", config_report.config_path);

    let doctor_report = doctor::run_doctor(doctor::DoctorOptions {
        paths: paths.clone(),
        timeout: std::time::Duration::from_millis(timeout_ms),
    })
    .await?;
    println!("{}", doctor_report.to_text());
    let blocking_errors: Vec<&doctor::DoctorCheck> = doctor_report
        .checks
        .iter()
        .filter(|c| is_onboard_blocking_doctor_error(c))
        .collect();
    if !blocking_errors.is_empty() {
        eprintln!("onboard blocked by critical setup errors:");
        for c in &blocking_errors {
            eprintln!("- {}: {}", c.id, c.message);
        }
        let report = OnboardReportArtifact {
            generated_at_ms: now_ms(),
            workspace: workspace.display().to_string(),
            config_path: config_report.config_path.clone(),
            config_valid: true,
            starter: starter.as_str().to_string(),
            effective_prompt: effective_prompt.clone(),
            doctor_summary: doctor_report.summary.clone(),
            doctor_next_actions: doctor_report.next_actions.clone(),
            task: OnboardTaskReport {
                status: "blocked".to_string(),
                session_id: None,
                failure_category: Some("doctor_blocked".to_string()),
                error: Some(
                    blocking_errors
                        .iter()
                        .map(|check| format!("{}: {}", check.id, check.message))
                        .collect::<Vec<_>>()
                        .join("; "),
                ),
            },
            recommended_next_command: "loopforge doctor".to_string(),
            starter_suggestions: build_onboard_starter_suggestions(&workspace),
        };
        let (json_path, md_path) = write_onboard_report(&workspace, &report)?;
        print_onboard_report_summary(&report, &json_path, &md_path);
        std::process::exit(1);
    }
    let non_blocking_errors = doctor_report
        .checks
        .iter()
        .filter(|c| c.status == doctor::CheckStatus::Error)
        .count()
        .saturating_sub(blocking_errors.len());
    if non_blocking_errors > 0 {
        eprintln!(
            "onboard: continuing despite {} non-blocking doctor error(s)",
            non_blocking_errors
        );
    }

    if skip_agent {
        let report = OnboardReportArtifact {
            generated_at_ms: now_ms(),
            workspace: workspace.display().to_string(),
            config_path: config_report.config_path.clone(),
            config_valid: true,
            starter: starter.as_str().to_string(),
            effective_prompt: effective_prompt.clone(),
            doctor_summary: doctor_report.summary.clone(),
            doctor_next_actions: doctor_report.next_actions.clone(),
            task: OnboardTaskReport {
                status: "skipped".to_string(),
                session_id: None,
                failure_category: None,
                error: None,
            },
            recommended_next_command: format!(
                "loopforge onboard --workspace {} --starter {}",
                shell_quote(&workspace.display().to_string()),
                starter.as_str()
            ),
            starter_suggestions: build_onboard_starter_suggestions(&workspace),
        };
        let (json_path, md_path) = write_onboard_report(&workspace, &report)?;
        print_onboard_report_summary(&report, &json_path, &md_path);
        println!("onboard done (skipped first agent run)");
        return Ok(());
    }

    let cfg = RexosConfig::load(&paths)?;
    let mut cfg = cfg;
    if cfg.router.coding.provider.trim() == "ollama" {
        let maybe_ollama = cfg.providers.get("ollama").cloned();
        if let Some(ollama) = maybe_ollama {
            if ollama.kind == ProviderKind::OpenAiCompatible {
                if let Ok(models) = fetch_openai_compat_models(&ollama.base_url, timeout_ms).await {
                    if let Some(selected) = select_onboard_model(&ollama.default_model, &models) {
                        if selected != ollama.default_model {
                            if let Some(p) = cfg.providers.get_mut("ollama") {
                                p.default_model = selected.clone();
                            }
                            println!(
                                "onboard: ollama default model '{}' not available, using '{}'",
                                ollama.default_model, selected
                            );
                        }
                    }
                }
            }
        }
    }

    let memory = MemoryStore::open_or_create(&paths)?;
    let llms = rexos::llm::registry::LlmRegistry::from_config(&cfg)?;
    let security = cfg.security.clone();
    let router = rexos::router::ModelRouter::new(cfg.router);
    let agent =
        rexos::agent::AgentRuntime::new_with_security_config(memory, llms, router, security);

    let session_id = rexos::harness::resolve_session_id(&workspace)?;
    let out = match agent
        .run_session(
            workspace.clone(),
            &session_id,
            None,
            &effective_prompt,
            rexos::router::TaskKind::Coding,
        )
        .await
    {
        Ok(out) => out,
        Err(e) => {
            let err_msg = e.to_string();
            let failure_category = classify_onboard_failure(&err_msg);
            match record_onboard_attempt(
                &paths,
                &workspace,
                &session_id,
                false,
                Some(&failure_category),
                Some(&err_msg),
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
            let report = OnboardReportArtifact {
                generated_at_ms: now_ms(),
                workspace: workspace.display().to_string(),
                config_path: config_report.config_path.clone(),
                config_valid: true,
                starter: starter.as_str().to_string(),
                effective_prompt: effective_prompt.clone(),
                doctor_summary: doctor_report.summary.clone(),
                doctor_next_actions: doctor_report.next_actions.clone(),
                task: OnboardTaskReport {
                    status: "failed".to_string(),
                    session_id: Some(session_id.clone()),
                    failure_category: Some(failure_category.clone()),
                    error: Some(err_msg.clone()),
                },
                recommended_next_command: recommended_next_command(&workspace, false),
                starter_suggestions: build_onboard_starter_suggestions(&workspace),
            };
            let (json_path, md_path) = write_onboard_report(&workspace, &report)?;
            print_onboard_report_summary(&report, &json_path, &md_path);
            eprintln!("onboard: first agent run failed: {e}");
            eprintln!(
                "hint: run `ollama list` and set [providers.ollama].default_model in ~/.loopforge/config.toml to an available chat model"
            );
            return Err(e);
        }
    };
    println!("{out}");
    eprintln!("[loopforge] session_id={session_id}");

    if let Err(err) = verify_onboard_artifact(&workspace, prompt.as_deref(), starter) {
        let err_msg = err.to_string();
        let failure_category = "expected_artifact_missing".to_string();
        match record_onboard_attempt(
            &paths,
            &workspace,
            &session_id,
            false,
            Some(&failure_category),
            Some(&err_msg),
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
        let report = OnboardReportArtifact {
            generated_at_ms: now_ms(),
            workspace: workspace.display().to_string(),
            config_path: config_report.config_path.clone(),
            config_valid: true,
            starter: starter.as_str().to_string(),
            effective_prompt: effective_prompt.clone(),
            doctor_summary: doctor_report.summary.clone(),
            doctor_next_actions: doctor_report.next_actions.clone(),
            task: OnboardTaskReport {
                status: "failed".to_string(),
                session_id: Some(session_id.clone()),
                failure_category: Some(failure_category.clone()),
                error: Some(err_msg.clone()),
            },
            recommended_next_command: recommended_retry_command(
                &workspace,
                &session_id,
                &effective_prompt,
            ),
            starter_suggestions: build_onboard_starter_suggestions(&workspace),
        };
        let (json_path, md_path) = write_onboard_report(&workspace, &report)?;
        print_onboard_report_summary(&report, &json_path, &md_path);
        eprintln!("onboard: first agent run finished without the expected starter artifact");
        return Err(err);
    }

    match record_onboard_attempt(&paths, &workspace, &session_id, true, None, None) {
        Ok(metrics) => {
            println!(
                "onboard metrics: success_rate={}/{}",
                metrics.first_task_success, metrics.attempted_first_task
            );
        }
        Err(log_err) => {
            eprintln!("onboard: failed to persist metrics: {log_err}");
        }
    }
    let report = OnboardReportArtifact {
        generated_at_ms: now_ms(),
        workspace: workspace.display().to_string(),
        config_path: config_report.config_path.clone(),
        config_valid: true,
        starter: starter.as_str().to_string(),
        effective_prompt: effective_prompt.clone(),
        doctor_summary: doctor_report.summary.clone(),
        doctor_next_actions: doctor_report.next_actions.clone(),
        task: OnboardTaskReport {
            status: "succeeded".to_string(),
            session_id: Some(session_id.clone()),
            failure_category: None,
            error: None,
        },
        recommended_next_command: recommended_next_command(&workspace, true),
        starter_suggestions: build_onboard_starter_suggestions(&workspace),
    };
    let (json_path, md_path) = write_onboard_report(&workspace, &report)?;
    print_onboard_report_summary(&report, &json_path, &md_path);
    println!("onboard done (first agent run completed)");
    Ok(())
}

fn select_onboard_model(preferred: &str, available: &[String]) -> Option<String> {
    if available.is_empty() {
        return None;
    }
    let preferred = preferred.trim();
    if !preferred.is_empty() {
        if let Some(hit) = available
            .iter()
            .find(|m| m.trim().eq_ignore_ascii_case(preferred))
        {
            return Some(hit.clone());
        }
    }

    if let Some(chat_like) = available.iter().find(|m| {
        let lower = m.to_ascii_lowercase();
        !lower.contains("embed")
    }) {
        return Some(chat_like.clone());
    }
    Some(available[0].clone())
}

fn is_onboard_blocking_doctor_error(check: &doctor::DoctorCheck) -> bool {
    if check.status != doctor::CheckStatus::Error {
        return false;
    }
    check.id == "config.parse" || check.id.starts_with("router.")
}

fn onboard_metrics_path(paths: &RexosPaths) -> PathBuf {
    paths.base_dir.join("onboard-metrics.json")
}

fn onboard_events_path(paths: &RexosPaths) -> PathBuf {
    paths.base_dir.join("onboard-events.jsonl")
}

fn now_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}

fn classify_onboard_failure(err_msg: &str) -> String {
    let lower = err_msg.to_ascii_lowercase();

    let looks_like_model =
        lower.contains("model") && (lower.contains("not found") || lower.contains("unknown"));
    if looks_like_model || lower.contains("embedding-only") || lower.contains("no chat model") {
        return "model_unavailable".to_string();
    }

    let looks_like_connectivity = lower.contains("timed out")
        || lower.contains("connection refused")
        || lower.contains("failed to send request")
        || lower.contains("dns")
        || lower.contains("name or service not known")
        || lower.contains("http");
    if looks_like_connectivity {
        return "provider_unreachable".to_string();
    }

    if lower.contains("tool") {
        return "tool_runtime_error".to_string();
    }

    if lower.contains("sandbox") || lower.contains("permission denied") {
        return "sandbox_restriction".to_string();
    }

    "unknown".to_string()
}

fn load_onboard_metrics(paths: &RexosPaths) -> OnboardMetrics {
    let p = onboard_metrics_path(paths);
    match std::fs::read_to_string(&p) {
        Ok(raw) => serde_json::from_str::<OnboardMetrics>(&raw).unwrap_or_default(),
        Err(_) => OnboardMetrics::default(),
    }
}

fn save_onboard_metrics(paths: &RexosPaths, metrics: &OnboardMetrics) -> anyhow::Result<()> {
    let p = onboard_metrics_path(paths);
    let raw = serde_json::to_string_pretty(metrics)?;
    std::fs::write(&p, raw).with_context(|| format!("write {}", p.display()))?;
    Ok(())
}

fn append_onboard_event(paths: &RexosPaths, event: &OnboardEvent) -> anyhow::Result<()> {
    let p = onboard_events_path(paths);
    let mut f = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&p)
        .with_context(|| format!("open {}", p.display()))?;
    let line = serde_json::to_string(event)?;
    writeln!(f, "{line}").with_context(|| format!("append {}", p.display()))?;
    Ok(())
}

fn record_onboard_attempt(
    paths: &RexosPaths,
    workspace: &Path,
    session_id: &str,
    success: bool,
    failure_category: Option<&str>,
    error: Option<&str>,
) -> anyhow::Result<OnboardMetrics> {
    let mut metrics = load_onboard_metrics(paths);
    metrics.attempted_first_task += 1;
    if success {
        metrics.first_task_success += 1;
    } else {
        metrics.first_task_failed += 1;
        if let Some(category) = failure_category {
            let entry = metrics
                .failure_by_category
                .entry(category.to_string())
                .or_insert(0);
            *entry += 1;
        }
    }
    metrics.updated_at_ms = now_ms();
    save_onboard_metrics(paths, &metrics)?;

    let event = OnboardEvent {
        ts_ms: metrics.updated_at_ms,
        workspace: workspace.display().to_string(),
        session_id: session_id.to_string(),
        outcome: if success {
            "success".to_string()
        } else {
            "failed".to_string()
        },
        failure_category: failure_category.map(|s| s.to_string()),
        error: error.map(|s| s.to_string()),
    };
    append_onboard_event(paths, &event)?;

    Ok(metrics)
}

fn resolve_onboard_prompt(prompt: Option<&str>, starter: OnboardStarter) -> String {
    prompt
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(|value| value.to_string())
        .unwrap_or_else(|| starter.default_prompt().to_string())
}

fn starter_expected_artifact(
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

fn verify_onboard_artifact(
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

fn recommended_retry_command(workspace: &Path, session_id: &str, prompt: &str) -> String {
    format!(
        "loopforge agent run --workspace {} --session {} --prompt {}",
        shell_quote(&workspace.display().to_string()),
        shell_quote(session_id),
        shell_quote(prompt)
    )
}

fn shell_quote(value: &str) -> String {
    format!("'{}'", value.replace('\'', "'\"'\"'"))
}

fn recommended_next_command(workspace: &Path, success: bool) -> String {
    if success {
        format!(
            "loopforge agent run --workspace {} --prompt {}",
            shell_quote(&workspace.display().to_string()),
            shell_quote("Continue from the current workspace and write notes/next-steps.md with 3 follow-up actions.")
        )
    } else {
        "loopforge doctor".to_string()
    }
}

fn build_onboard_starter_suggestions(workspace: &Path) -> Vec<OnboardStarterSuggestion> {
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

fn write_onboard_report(
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

fn print_onboard_report_summary(report: &OnboardReportArtifact, json_path: &Path, md_path: &Path) {
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

async fn fetch_openai_compat_models(
    base_url: &str,
    timeout_ms: u64,
) -> anyhow::Result<Vec<String>> {
    let endpoint = format!("{}/models", base_url.trim_end_matches('/'));
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_millis(timeout_ms.max(500)))
        .build()
        .context("build model probe http client")?;
    let res = client.get(&endpoint).send().await?;
    if !res.status().is_success() {
        anyhow::bail!("GET {endpoint} -> {}", res.status());
    }
    let v: serde_json::Value = res.json().await?;
    let mut out = Vec::new();
    if let Some(arr) = v.get("data").and_then(|x| x.as_array()) {
        for item in arr {
            if let Some(id) = item.get("id").and_then(|x| x.as_str()) {
                let id = id.trim();
                if !id.is_empty() {
                    out.push(id.to_string());
                    continue;
                }
            }
            if let Some(name) = item.get("name").and_then(|x| x.as_str()) {
                let name = name.trim();
                if !name.is_empty() {
                    out.push(name.to_string());
                }
            }
        }
    }
    out.sort();
    out.dedup();
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn select_onboard_model_prefers_configured_when_available() {
        let selected = select_onboard_model(
            "llama3.2",
            &["qwen3:4b".to_string(), "llama3.2".to_string()],
        );
        assert_eq!(selected.as_deref(), Some("llama3.2"));
    }

    #[test]
    fn select_onboard_model_falls_back_to_first_non_embedding() {
        let selected = select_onboard_model(
            "llama3.2",
            &[
                "nomic-embed-text:latest".to_string(),
                "qwen3:4b".to_string(),
            ],
        );
        assert_eq!(selected.as_deref(), Some("qwen3:4b"));
    }

    #[test]
    fn select_onboard_model_uses_first_when_only_embedding_exists() {
        let selected = select_onboard_model("llama3.2", &["nomic-embed-text:latest".to_string()]);
        assert_eq!(selected.as_deref(), Some("nomic-embed-text:latest"));
    }

    #[test]
    fn resolve_onboard_prompt_uses_starter_when_prompt_missing() {
        let prompt = resolve_onboard_prompt(None, OnboardStarter::WorkspaceBrief);
        assert!(
            prompt.contains("workspace-brief") || prompt.contains("notes/workspace-brief.md"),
            "expected workspace brief prompt, got: {prompt}"
        );
    }

    #[test]
    fn resolve_onboard_prompt_prefers_explicit_prompt() {
        let prompt = resolve_onboard_prompt(Some("Create notes/custom.md"), OnboardStarter::Hello);
        assert_eq!(prompt, "Create notes/custom.md");
    }

    #[test]
    fn starter_expected_artifact_tracks_default_starters() {
        assert_eq!(
            starter_expected_artifact(None, OnboardStarter::WorkspaceBrief),
            Some("notes/workspace-brief.md")
        );
        assert_eq!(
            starter_expected_artifact(None, OnboardStarter::RepoOnboarding),
            Some("notes/repo-onboarding.md")
        );
        assert_eq!(
            starter_expected_artifact(Some("Create notes/custom.md"), OnboardStarter::Hello),
            None
        );
    }

    #[test]
    fn verify_onboard_artifact_requires_default_starter_output() {
        let tmp = tempdir().unwrap();
        let workspace = tmp.path().join("demo-work");
        std::fs::create_dir_all(&workspace).unwrap();

        let err = verify_onboard_artifact(&workspace, None, OnboardStarter::WorkspaceBrief)
            .expect_err("missing starter artifact should fail");
        assert!(err.to_string().contains("notes/workspace-brief.md"));

        std::fs::create_dir_all(workspace.join("notes")).unwrap();
        std::fs::write(workspace.join("notes/workspace-brief.md"), "brief").unwrap();
        assert_eq!(
            verify_onboard_artifact(&workspace, None, OnboardStarter::WorkspaceBrief).unwrap(),
            Some("notes/workspace-brief.md".to_string())
        );
        assert_eq!(
            verify_onboard_artifact(
                &workspace,
                Some("Create notes/custom.md"),
                OnboardStarter::WorkspaceBrief,
            )
            .unwrap(),
            None
        );
    }

    #[test]
    fn write_onboard_report_writes_json_and_markdown() {
        let tmp = tempdir().unwrap();
        let workspace = tmp.path().join("demo-work");
        std::fs::create_dir_all(&workspace).unwrap();
        let report = OnboardReportArtifact {
            generated_at_ms: 1,
            workspace: workspace.display().to_string(),
            config_path: "~/.loopforge/config.toml".to_string(),
            config_valid: true,
            starter: OnboardStarter::Hello.as_str().to_string(),
            effective_prompt: OnboardStarter::Hello.default_prompt().to_string(),
            doctor_summary: doctor::DoctorSummary {
                ok: 2,
                warn: 1,
                error: 0,
            },
            doctor_next_actions: vec![
                "Run `loopforge doctor` again after fixing config.".to_string()
            ],
            task: OnboardTaskReport {
                status: "skipped".to_string(),
                session_id: None,
                failure_category: None,
                error: None,
            },
            recommended_next_command: "loopforge onboard --workspace demo-work".to_string(),
            starter_suggestions: build_onboard_starter_suggestions(&workspace),
        };

        let (json_path, md_path) = write_onboard_report(&workspace, &report).unwrap();
        assert!(
            json_path.exists(),
            "expected json report at {}",
            json_path.display()
        );
        assert!(
            md_path.exists(),
            "expected markdown report at {}",
            md_path.display()
        );

        let md = std::fs::read_to_string(md_path).unwrap();
        assert!(md.contains("LoopForge Onboard Report"));
        assert!(md.contains("Suggested Next Steps"));
    }

    #[test]
    fn onboard_blocks_config_and_router_errors() {
        let config_error = doctor::DoctorCheck {
            id: "config.parse".to_string(),
            status: doctor::CheckStatus::Error,
            message: "bad toml".to_string(),
        };
        let router_error = doctor::DoctorCheck {
            id: "router.coding.provider".to_string(),
            status: doctor::CheckStatus::Error,
            message: "unknown provider".to_string(),
        };

        assert!(is_onboard_blocking_doctor_error(&config_error));
        assert!(is_onboard_blocking_doctor_error(&router_error));
    }

    #[test]
    fn onboard_does_not_block_non_critical_errors() {
        let git_error = doctor::DoctorCheck {
            id: "tools.git".to_string(),
            status: doctor::CheckStatus::Error,
            message: "git not found".to_string(),
        };
        let browser_warn = doctor::DoctorCheck {
            id: "browser.chromium".to_string(),
            status: doctor::CheckStatus::Warn,
            message: "missing".to_string(),
        };

        assert!(!is_onboard_blocking_doctor_error(&git_error));
        assert!(!is_onboard_blocking_doctor_error(&browser_warn));
    }

    #[test]
    fn classify_onboard_failure_groups_common_causes() {
        assert_eq!(
            classify_onboard_failure("model llama3.2 not found"),
            "model_unavailable"
        );
        assert_eq!(
            classify_onboard_failure("request timed out while calling http provider"),
            "provider_unreachable"
        );
        assert_eq!(
            classify_onboard_failure("tool call failed with invalid arguments"),
            "tool_runtime_error"
        );
    }

    #[test]
    fn record_onboard_attempt_updates_metrics_and_events() {
        let tmp = tempdir().unwrap();
        let paths = RexosPaths {
            base_dir: tmp.path().join(".loopforge"),
        };
        std::fs::create_dir_all(&paths.base_dir).unwrap();
        let workspace = tmp.path().join("demo-work");
        std::fs::create_dir_all(&workspace).unwrap();

        let m1 = record_onboard_attempt(&paths, &workspace, "s1", true, None, None).unwrap();
        assert_eq!(m1.attempted_first_task, 1);
        assert_eq!(m1.first_task_success, 1);
        assert_eq!(m1.first_task_failed, 0);

        let m2 = record_onboard_attempt(
            &paths,
            &workspace,
            "s2",
            false,
            Some("provider_unreachable"),
            Some("timeout"),
        )
        .unwrap();
        assert_eq!(m2.attempted_first_task, 2);
        assert_eq!(m2.first_task_success, 1);
        assert_eq!(m2.first_task_failed, 1);
        assert_eq!(m2.failure_by_category.get("provider_unreachable"), Some(&1));

        let events_raw = std::fs::read_to_string(onboard_events_path(&paths)).unwrap();
        assert_eq!(events_raw.lines().count(), 2);
    }
}
