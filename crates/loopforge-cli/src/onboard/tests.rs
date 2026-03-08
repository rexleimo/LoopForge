use super::*;
use crate::doctor;
use rexos::paths::RexosPaths;
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
        doctor_next_actions: vec!["Run `loopforge doctor` again after fixing config.".to_string()],
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

    assert!(doctor_gate::is_onboard_blocking_doctor_error(&config_error));
    assert!(doctor_gate::is_onboard_blocking_doctor_error(&router_error));
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

    assert!(!doctor_gate::is_onboard_blocking_doctor_error(&git_error));
    assert!(!doctor_gate::is_onboard_blocking_doctor_error(
        &browser_warn
    ));
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
