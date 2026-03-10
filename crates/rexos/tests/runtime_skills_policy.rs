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
            aws_bedrock: None,
        },
    );

    let cfg = rexos::config::RexosConfig {
        llm: rexos::config::LlmConfig::default(),
        providers,
        router: rexos::config::RouterConfig::default(),
        security: Default::default(),
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

struct EnvVarGuard {
    key: &'static str,
    previous: Option<String>,
}

impl EnvVarGuard {
    fn set(key: &'static str, value: &str) -> Self {
        let previous = std::env::var(key).ok();
        std::env::set_var(key, value);
        Self { key, previous }
    }
}

impl Drop for EnvVarGuard {
    fn drop(&mut self) {
        if let Some(v) = self.previous.take() {
            std::env::set_var(self.key, v);
        } else {
            std::env::remove_var(self.key);
        }
    }
}

#[test]
fn blocks_skill_when_not_in_session_allowed_skills() {
    let tmp = tempfile::tempdir().unwrap();
    let paths = rexos::paths::RexosPaths {
        base_dir: tmp.path().join(".loopforge"),
    };
    paths.ensure_dirs().unwrap();

    let memory = rexos::memory::MemoryStore::open_or_create(&paths).unwrap();
    let agent = build_agent(memory);

    agent
        .set_session_allowed_skills("s1", vec!["safe-skill".to_string()])
        .unwrap();

    let err = agent
        .authorize_skill("s1", "danger-skill", &["tool:shell".to_string()])
        .unwrap_err();
    let err_text = err.to_string();
    assert!(err_text.contains("skill not allowed"), "got: {err_text}");
}

#[test]
fn blocks_skill_when_policy_requires_approval() {
    let tmp = tempfile::tempdir().unwrap();
    let paths = rexos::paths::RexosPaths {
        base_dir: tmp.path().join(".loopforge"),
    };
    paths.ensure_dirs().unwrap();

    let memory = rexos::memory::MemoryStore::open_or_create(&paths).unwrap();
    let agent = build_agent(memory);

    agent
        .set_session_skill_policy(
            "s2",
            rexos::agent::SessionSkillPolicy {
                allowlist: vec!["shell-helper".to_string()],
                require_approval: true,
                auto_approve_readonly: false,
            },
        )
        .unwrap();

    let _allow_guard = EnvVarGuard::set("LOOPFORGE_SKILL_APPROVAL_ALLOW", "");
    let err = agent
        .authorize_skill("s2", "shell-helper", &["tool:shell".to_string()])
        .unwrap_err();
    let err_text = err.to_string();
    assert!(
        err_text.contains("approval required for skill"),
        "got: {err_text}"
    );
}

#[test]
fn auto_approves_readonly_skill_when_policy_enabled() {
    let tmp = tempfile::tempdir().unwrap();
    let paths = rexos::paths::RexosPaths {
        base_dir: tmp.path().join(".loopforge"),
    };
    paths.ensure_dirs().unwrap();

    let memory = rexos::memory::MemoryStore::open_or_create(&paths).unwrap();
    let agent = build_agent(memory);

    agent
        .set_session_skill_policy(
            "s3",
            rexos::agent::SessionSkillPolicy {
                allowlist: vec!["reader".to_string()],
                require_approval: true,
                auto_approve_readonly: true,
            },
        )
        .unwrap();

    agent
        .authorize_skill(
            "s3",
            "reader",
            &["readonly".to_string(), "tool:fs_read".to_string()],
        )
        .unwrap();
}
