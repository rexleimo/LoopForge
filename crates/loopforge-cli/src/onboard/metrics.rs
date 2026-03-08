use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::Context;
use rexos::paths::RexosPaths;

use super::types::{OnboardEvent, OnboardMetrics};

fn onboard_metrics_path(paths: &RexosPaths) -> PathBuf {
    paths.base_dir.join("onboard-metrics.json")
}

pub(crate) fn onboard_events_path(paths: &RexosPaths) -> PathBuf {
    paths.base_dir.join("onboard-events.jsonl")
}

pub(crate) fn now_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis() as i64)
        .unwrap_or(0)
}

pub(crate) fn classify_onboard_failure(err_msg: &str) -> String {
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
    let path = onboard_metrics_path(paths);
    match std::fs::read_to_string(&path) {
        Ok(raw) => serde_json::from_str::<OnboardMetrics>(&raw).unwrap_or_default(),
        Err(_) => OnboardMetrics::default(),
    }
}

fn save_onboard_metrics(paths: &RexosPaths, metrics: &OnboardMetrics) -> anyhow::Result<()> {
    let path = onboard_metrics_path(paths);
    let raw = serde_json::to_string_pretty(metrics)?;
    std::fs::write(&path, raw).with_context(|| format!("write {}", path.display()))?;
    Ok(())
}

fn append_onboard_event(paths: &RexosPaths, event: &OnboardEvent) -> anyhow::Result<()> {
    let path = onboard_events_path(paths);
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .with_context(|| format!("open {}", path.display()))?;
    let line = serde_json::to_string(event)?;
    writeln!(file, "{line}").with_context(|| format!("append {}", path.display()))?;
    Ok(())
}

pub(crate) fn record_onboard_attempt(
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
        failure_category: failure_category.map(|value| value.to_string()),
        error: error.map(|value| value.to_string()),
    };
    append_onboard_event(paths, &event)?;

    Ok(metrics)
}
