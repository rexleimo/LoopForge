use std::collections::BTreeMap;

fn build_agent(memory: rexos::memory::MemoryStore) -> rexos::agent::AgentRuntime {
    let mut providers = BTreeMap::new();
    providers.insert(
        "ollama".to_string(),
        rexos::config::ProviderConfig {
            kind: rexos::config::ProviderKind::OpenAiCompatible,
            base_url: "http://127.0.0.1:11434/v1".to_string(),
            api_key_env: "".to_string(),
            default_model: "x".to_string(),
        },
    );

    let cfg = rexos::config::RexosConfig {
        llm: rexos::config::LlmConfig::default(),
        providers,
        router: rexos::config::RouterConfig::default(),
    };
    let llms = rexos::llm::registry::LlmRegistry::from_config(&cfg).unwrap();
    let router = rexos::router::ModelRouter::new(rexos::config::RouterConfig {
        planning: rexos::config::RouteConfig {
            provider: "ollama".to_string(),
            model: "x".to_string(),
        },
        coding: rexos::config::RouteConfig {
            provider: "ollama".to_string(),
            model: "x".to_string(),
        },
        summary: rexos::config::RouteConfig {
            provider: "ollama".to_string(),
            model: "x".to_string(),
        },
    });

    rexos::agent::AgentRuntime::new(memory, llms, router)
}

#[test]
fn emits_skill_events_and_audit_records() {
    let tmp = tempfile::tempdir().unwrap();
    let paths = rexos::paths::RexosPaths {
        base_dir: tmp.path().join(".rexos"),
    };
    paths.ensure_dirs().unwrap();

    let memory = rexos::memory::MemoryStore::open_or_create(&paths).unwrap();
    let agent = build_agent(memory);

    let session_id = "skills-audit";
    let perms = vec!["tool:fs_read".to_string()];

    agent
        .record_skill_discovered(session_id, "reader", "workspace", "0.1.0")
        .unwrap();
    agent.authorize_skill(session_id, "reader", &perms).unwrap();
    agent
        .record_skill_execution(session_id, "reader", &perms, true, None)
        .unwrap();
    agent
        .record_skill_execution(session_id, "reader", &perms, false, Some("boom"))
        .unwrap();

    let events = agent.list_acp_events(Some(session_id), 100).unwrap();
    assert!(events.iter().any(|e| e.event_type == "skill.discovered"));
    assert!(events.iter().any(|e| e.event_type == "skill.loaded"));
    assert!(events.iter().any(|e| e.event_type == "skill.executed"));
    assert!(events.iter().any(|e| e.event_type == "skill.failed"));

    let memory2 = rexos::memory::MemoryStore::open_or_create(&paths).unwrap();
    let raw = memory2
        .kv_get("rexos.audit.skill_runs")
        .unwrap()
        .unwrap_or_default();
    let records: serde_json::Value = serde_json::from_str(&raw).unwrap();
    let arr = records.as_array().unwrap();
    assert!(arr.iter().any(|v| v["session_id"] == session_id && v["success"] == true));
    assert!(arr.iter().any(|v| {
        v["session_id"] == session_id
            && v["success"] == false
            && v["error"].as_str().unwrap_or("").contains("boom")
    }));
}
