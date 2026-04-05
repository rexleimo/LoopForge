use std::collections::BTreeMap;

use rexos::config::{ProviderConfig, ProviderKind, RexosConfig, RouteConfig, RouterConfig};
use rexos::paths::RexosPaths;
use rexos::router::TaskKind;
use rexos::security::SecurityConfig;

mod support;

fn fixture_agent(
    tmp: &tempfile::TempDir,
    fixture_base_url: String,
    security: SecurityConfig,
) -> (rexos::agent::AgentRuntime, RexosPaths, std::path::PathBuf) {
    let paths = RexosPaths {
        base_dir: tmp.path().join(".loopforge"),
    };
    paths.ensure_dirs().unwrap();

    let workspace_root = tmp.path().join("workspace");
    std::fs::create_dir_all(&workspace_root).unwrap();

    let mut providers = BTreeMap::new();
    providers.insert(
        "fixture".to_string(),
        ProviderConfig {
            kind: ProviderKind::OpenAiCompatible,
            base_url: fixture_base_url,
            api_key_env: String::new(),
            default_model: "fixture-model".to_string(),
            aws_bedrock: None,
        },
    );

    let cfg = RexosConfig {
        llm: Default::default(),
        providers,
        router: RouterConfig {
            planning: RouteConfig {
                provider: "fixture".to_string(),
                model: "fixture-model".to_string(),
            },
            coding: RouteConfig {
                provider: "fixture".to_string(),
                model: "fixture-model".to_string(),
            },
            summary: RouteConfig {
                provider: "fixture".to_string(),
                model: "fixture-model".to_string(),
            },
        },
        security: security.clone(),
    };

    let llms = rexos::llm::registry::LlmRegistry::from_config(&cfg).unwrap();
    let router = rexos::router::ModelRouter::new(cfg.router);
    let memory = rexos::memory::MemoryStore::open_or_create(&paths).unwrap();
    let agent =
        rexos::agent::AgentRuntime::new_with_security_config(memory, llms, router, security);

    (agent, paths, workspace_root)
}

#[tokio::test]
async fn replay_fixture_drives_session_and_tool_calls() {
    let fixture = support::openai_compat_fixture::load_json_array(include_str!(
        "fixtures/replay/session_write_file.json"
    ));
    let server = support::openai_compat_fixture::FixtureServer::spawn(fixture).await;

    let tmp = tempfile::tempdir().unwrap();
    let (agent, _paths, workspace_root) = fixture_agent(
        &tmp,
        server.base_url.clone(),
        rexos::security::SecurityConfig::default(),
    );

    let session_id = "s-replay";
    agent
        .set_session_allowed_tools(session_id, vec!["fs_write".to_string()])
        .unwrap();

    let out = agent
        .run_session(
            workspace_root.clone(),
            session_id,
            None,
            "write hello.txt",
            TaskKind::Coding,
        )
        .await
        .unwrap();

    assert_eq!(out, "done");
    assert_eq!(
        std::fs::read_to_string(workspace_root.join("hello.txt")).unwrap(),
        "hello"
    );

    let requests = server.requests.lock().unwrap().clone();
    assert_eq!(requests.len(), 2, "expected two chat completions calls");
    assert_eq!(requests[0]["messages"][0]["role"], "user");
    assert_eq!(requests[1]["messages"][2]["role"], "tool");
    assert_eq!(requests[1]["messages"][2]["name"], "fs_write");
    assert_eq!(requests[1]["messages"][2]["tool_call_id"], "call_1");
    assert_eq!(requests[1]["messages"][2]["content"], "ok");

    server.abort();
}

#[tokio::test]
async fn replay_fixture_blocks_tool_not_in_allowed_tools() {
    let fixture = support::openai_compat_fixture::load_json_array(include_str!(
        "fixtures/replay/session_tool_not_allowed.json"
    ));
    let server = support::openai_compat_fixture::FixtureServer::spawn(fixture).await;

    let tmp = tempfile::tempdir().unwrap();
    let (agent, _paths, workspace_root) = fixture_agent(
        &tmp,
        server.base_url.clone(),
        rexos::security::SecurityConfig::default(),
    );

    let session_id = "s-replay-deny";
    agent
        .set_session_allowed_tools(session_id, vec!["fs_read".to_string()])
        .unwrap();

    let err = agent
        .run_session(
            workspace_root,
            session_id,
            None,
            "try write",
            TaskKind::Coding,
        )
        .await
        .unwrap_err();
    let err_text = err.to_string();
    assert!(
        err_text.contains("tool not allowed"),
        "expected deny error, got: {err_text}"
    );

    let requests = server.requests.lock().unwrap().clone();
    assert_eq!(requests.len(), 1, "expected one chat completions call");

    server.abort();
}

#[tokio::test]
async fn replay_fixture_surfaces_tool_failure_errors() {
    let fixture = support::openai_compat_fixture::load_json_array(include_str!(
        "fixtures/replay/session_tool_failed_invalid_path.json"
    ));
    let server = support::openai_compat_fixture::FixtureServer::spawn(fixture).await;

    let tmp = tempfile::tempdir().unwrap();
    let (agent, _paths, workspace_root) = fixture_agent(
        &tmp,
        server.base_url.clone(),
        rexos::security::SecurityConfig::default(),
    );

    let session_id = "s-replay-tool-failed";
    agent
        .set_session_allowed_tools(session_id, vec!["fs_write".to_string()])
        .unwrap();

    let err = agent
        .run_session(
            workspace_root,
            session_id,
            None,
            "write outside workspace",
            TaskKind::Coding,
        )
        .await
        .unwrap_err();
    let err_text = err.to_string();
    assert!(
        err_text.contains("parent traversal"),
        "expected relative-path validation error, got: {err_text}"
    );

    let requests = server.requests.lock().unwrap().clone();
    assert_eq!(requests.len(), 1, "expected one chat completions call");

    server.abort();
}

#[tokio::test]
async fn replay_fixture_executes_mcp_tool_calls() {
    let fixture = support::openai_compat_fixture::load_json_array(include_str!(
        "fixtures/replay/session_mcp_echo.json"
    ));
    let server = support::openai_compat_fixture::FixtureServer::spawn(fixture).await;

    let tmp = tempfile::tempdir().unwrap();
    let (agent, _paths, workspace_root) = fixture_agent(
        &tmp,
        server.base_url.clone(),
        rexos::security::SecurityConfig::default(),
    );

    let mcp_stub = support::mcp_stub::write_mcp_stub(&workspace_root);

    let session_id = "s-replay-mcp";
    agent
        .set_session_mcp_config(session_id, support::mcp_stub::mcp_config_json(&mcp_stub))
        .unwrap();
    agent
        .set_session_allowed_tools(session_id, vec!["mcp_stub__echo".to_string()])
        .unwrap();

    let out = agent
        .run_session(
            workspace_root,
            session_id,
            None,
            "call mcp",
            TaskKind::Coding,
        )
        .await
        .unwrap();
    assert_eq!(out, "done");

    let requests = server.requests.lock().unwrap().clone();
    assert_eq!(requests.len(), 2, "expected two chat completions calls");
    assert_eq!(requests[1]["messages"][2]["role"], "tool");
    assert_eq!(requests[1]["messages"][2]["name"], "mcp_stub__echo");
    assert_eq!(requests[1]["messages"][2]["tool_call_id"], "call_1");

    let tool_content = requests[1]["messages"][2]["content"]
        .as_str()
        .expect("tool message content");
    let tool_json: serde_json::Value = serde_json::from_str(tool_content).expect("tool JSON");
    assert_eq!(tool_json["content"][0]["text"], "yo");

    server.abort();
}
