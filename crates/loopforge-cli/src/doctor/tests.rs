use super::*;
use std::time::Duration;

use axum::routing::get;
use axum::{Json, Router};
use rexos::config::{ProviderKind, RexosConfig};
use rexos::paths::RexosPaths;
use serde_json::json;

#[tokio::test]
async fn doctor_suggests_running_init_when_core_files_are_missing() {
    let tmp = tempfile::tempdir().unwrap();
    let paths = RexosPaths {
        base_dir: tmp.path().join(".loopforge"),
    };
    std::fs::create_dir_all(&paths.base_dir).unwrap();

    let report = run_doctor(DoctorOptions {
        paths,
        timeout: Duration::from_millis(200),
    })
    .await
    .unwrap();

    let value = serde_json::to_value(&report).unwrap();
    let next_actions = value
        .get("next_actions")
        .and_then(|item| item.as_array())
        .cloned()
        .unwrap_or_default();
    assert!(
        next_actions
            .iter()
            .any(|item| item.as_str().unwrap_or("").contains("loopforge init")),
        "expected init guidance in next_actions, got: {next_actions:?}"
    );
    assert!(
        report.to_text().contains("Suggested next steps"),
        "expected text output to include suggested next steps, got: {}",
        report.to_text()
    );
}

#[tokio::test]
async fn doctor_suggests_missing_provider_env_vars() {
    let tmp = tempfile::tempdir().unwrap();
    let paths = RexosPaths {
        base_dir: tmp.path().join(".loopforge"),
    };
    std::fs::create_dir_all(&paths.base_dir).unwrap();

    let mut cfg = RexosConfig::default();
    cfg.providers.insert(
        "anthropic".to_string(),
        rexos::config::ProviderConfig {
            kind: ProviderKind::Anthropic,
            base_url: "https://api.anthropic.com".to_string(),
            api_key_env: "ANTHROPIC_API_KEY".to_string(),
            default_model: "claude-3-5-sonnet-latest".to_string(),
        },
    );
    std::fs::write(paths.config_path(), toml::to_string(&cfg).unwrap()).unwrap();
    std::env::remove_var("ANTHROPIC_API_KEY");

    let report = run_doctor(DoctorOptions {
        paths,
        timeout: Duration::from_millis(200),
    })
    .await
    .unwrap();

    let value = serde_json::to_value(&report).unwrap();
    let next_actions = value
        .get("next_actions")
        .and_then(|item| item.as_array())
        .cloned()
        .unwrap_or_default();
    assert!(
        next_actions
            .iter()
            .any(|item| item.as_str().unwrap_or("").contains("ANTHROPIC_API_KEY")),
        "expected provider env guidance in next_actions, got: {next_actions:?}"
    );
}

#[tokio::test]
async fn doctor_probes_local_ollama_models_and_cdp_version() {
    async fn models() -> Json<serde_json::Value> {
        Json(json!({ "data": [] }))
    }
    async fn cdp_version() -> Json<serde_json::Value> {
        Json(json!({ "Browser": "Chrome/1.0" }))
    }

    let app = Router::new()
        .route("/v1/models", get(models))
        .route("/json/version", get(cdp_version));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    let tmp = tempfile::tempdir().unwrap();
    let paths = RexosPaths {
        base_dir: tmp.path().join(".loopforge"),
    };
    std::fs::create_dir_all(&paths.base_dir).unwrap();

    let cfg = RexosConfig {
        llm: rexos::config::LlmConfig::default(),
        providers: [(
            "ollama".to_string(),
            rexos::config::ProviderConfig {
                kind: ProviderKind::OpenAiCompatible,
                base_url: format!("http://{addr}/v1"),
                api_key_env: "".to_string(),
                default_model: "x".to_string(),
            },
        )]
        .into_iter()
        .collect(),
        router: rexos::config::RouterConfig::default(),
        security: Default::default(),
    };
    std::fs::write(paths.config_path(), toml::to_string(&cfg).unwrap()).unwrap();
    std::env::set_var("LOOPFORGE_BROWSER_CDP_HTTP", format!("http://{addr}"));

    let report = run_doctor(DoctorOptions {
        paths,
        timeout: Duration::from_millis(500),
    })
    .await
    .unwrap();

    let statuses: std::collections::BTreeMap<String, CheckStatus> = report
        .checks
        .iter()
        .map(|check| (check.id.clone(), check.status))
        .collect();
    assert_eq!(statuses.get("ollama.http"), Some(&CheckStatus::Ok));
    assert_eq!(statuses.get("browser.cdp_http"), Some(&CheckStatus::Ok));

    std::env::remove_var("LOOPFORGE_BROWSER_CDP_HTTP");
    server.abort();
}

#[tokio::test]
async fn doctor_reports_security_posture_checks() {
    let tmp = tempfile::tempdir().unwrap();
    let paths = RexosPaths {
        base_dir: tmp.path().join(".loopforge"),
    };
    std::fs::create_dir_all(&paths.base_dir).unwrap();

    let mut cfg = RexosConfig {
        llm: rexos::config::LlmConfig::default(),
        providers: [(
            "ollama".to_string(),
            rexos::config::ProviderConfig {
                kind: ProviderKind::OpenAiCompatible,
                base_url: "http://127.0.0.1:11434/v1".to_string(),
                api_key_env: "".to_string(),
                default_model: "x".to_string(),
            },
        )]
        .into_iter()
        .collect(),
        router: rexos::config::RouterConfig::default(),
        security: Default::default(),
    };
    cfg.security.leaks.mode = rexos::security::LeakMode::Redact;
    cfg.security.egress.rules.push(rexos::security::EgressRule {
        tool: "web_fetch".to_string(),
        host: "docs.rs".to_string(),
        path_prefix: "/".to_string(),
        methods: vec!["GET".to_string()],
    });
    std::fs::write(paths.config_path(), toml::to_string(&cfg).unwrap()).unwrap();

    let report = run_doctor(DoctorOptions {
        paths,
        timeout: Duration::from_millis(200),
    })
    .await
    .unwrap();

    let statuses: std::collections::BTreeMap<String, CheckStatus> = report
        .checks
        .iter()
        .map(|check| (check.id.clone(), check.status))
        .collect();
    assert_eq!(
        statuses.get("security.secrets.mode"),
        Some(&CheckStatus::Ok)
    );
    assert_eq!(statuses.get("security.leaks.mode"), Some(&CheckStatus::Ok));
    assert_eq!(
        statuses.get("security.egress.rules"),
        Some(&CheckStatus::Ok)
    );
}

#[tokio::test]
async fn doctor_suggests_leak_guard_and_egress_hardening_when_defaults_are_open() {
    let tmp = tempfile::tempdir().unwrap();
    let paths = RexosPaths {
        base_dir: tmp.path().join(".loopforge"),
    };
    std::fs::create_dir_all(&paths.base_dir).unwrap();

    let cfg = RexosConfig {
        llm: rexos::config::LlmConfig::default(),
        providers: [(
            "ollama".to_string(),
            rexos::config::ProviderConfig {
                kind: ProviderKind::OpenAiCompatible,
                base_url: "http://127.0.0.1:11434/v1".to_string(),
                api_key_env: "".to_string(),
                default_model: "x".to_string(),
            },
        )]
        .into_iter()
        .collect(),
        router: rexos::config::RouterConfig::default(),
        security: Default::default(),
    };
    std::fs::write(paths.config_path(), toml::to_string(&cfg).unwrap()).unwrap();

    let report = run_doctor(DoctorOptions {
        paths,
        timeout: Duration::from_millis(200),
    })
    .await
    .unwrap();

    let statuses: std::collections::BTreeMap<String, CheckStatus> = report
        .checks
        .iter()
        .map(|check| (check.id.clone(), check.status))
        .collect();
    assert_eq!(
        statuses.get("security.leaks.mode"),
        Some(&CheckStatus::Warn)
    );
    assert_eq!(
        statuses.get("security.egress.rules"),
        Some(&CheckStatus::Warn)
    );
    assert!(
        report
            .next_actions
            .iter()
            .any(|item| item.contains("security.leaks")),
        "expected leak-guard guidance, got: {:?}",
        report.next_actions
    );
    assert!(
        report
            .next_actions
            .iter()
            .any(|item| item.contains("security.egress")),
        "expected egress guidance, got: {:?}",
        report.next_actions
    );
}
