use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

#[derive(Clone, Default)]
pub(super) struct TestState {
    pub(super) calls: Arc<Mutex<u32>>,
    pub(super) payloads: Arc<Mutex<Vec<serde_json::Value>>>,
}

pub(super) struct EnvVarGuard {
    key: &'static str,
    previous: Option<String>,
}

impl EnvVarGuard {
    pub(super) fn set(key: &'static str, value: &str) -> Self {
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

pub(super) fn test_agent(
    base_url: String,
    memory: rexos::memory::MemoryStore,
) -> rexos::agent::AgentRuntime {
    test_agent_with_security(base_url, memory, rexos::security::SecurityConfig::default())
}

pub(super) fn test_agent_with_security(
    base_url: String,
    memory: rexos::memory::MemoryStore,
    security: rexos::security::SecurityConfig,
) -> rexos::agent::AgentRuntime {
    let mut providers = BTreeMap::new();
    providers.insert(
        "ollama".to_string(),
        rexos::config::ProviderConfig {
            kind: rexos::config::ProviderKind::OpenAiCompatible,
            base_url,
            api_key_env: "".to_string(),
            default_model: "x".to_string(),
            aws_bedrock: None,
        },
    );

    let cfg = rexos::config::RexosConfig {
        llm: rexos::config::LlmConfig::default(),
        providers,
        router: rexos::config::RouterConfig::default(),
        security: security.clone(),
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
    rexos::agent::AgentRuntime::new_with_security_config(memory, llms, router, security)
}
