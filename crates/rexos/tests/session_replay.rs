use std::collections::BTreeMap;

use rexos::config::{ProviderConfig, ProviderKind, RexosConfig, RouteConfig, RouterConfig};
use rexos::paths::RexosPaths;
use rexos::router::TaskKind;
use rexos::security::SecurityConfig;

mod support;

#[tokio::test]
async fn replay_fixture_drives_session_and_tool_calls() {
    let fixture = support::openai_compat_fixture::load_json_array(include_str!(
        "fixtures/replay/session_write_file.json"
    ));
    let server = support::openai_compat_fixture::FixtureServer::spawn(fixture).await;

    let tmp = tempfile::tempdir().unwrap();
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
            base_url: server.base_url.clone(),
            api_key_env: String::new(),
            default_model: "fixture-model".to_string(),
            aws_bedrock: None,
        },
    );

    let security = SecurityConfig::default();
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
