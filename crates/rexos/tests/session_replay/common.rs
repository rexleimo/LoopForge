use std::collections::BTreeMap;

use rexos::config::{ProviderConfig, ProviderKind, RexosConfig, RouteConfig, RouterConfig};
use rexos::paths::RexosPaths;
use rexos::security::SecurityConfig;
use serde_json::{json, Value};

pub(super) struct EnvVarGuard {
    key: &'static str,
    prev: Option<String>,
}

impl EnvVarGuard {
    pub(super) fn set(key: &'static str, value: &str) -> Self {
        let prev = std::env::var(key).ok();
        std::env::set_var(key, value);
        Self { key, prev }
    }
}

impl Drop for EnvVarGuard {
    fn drop(&mut self) {
        match self.prev.as_ref() {
            Some(value) => std::env::set_var(self.key, value),
            None => std::env::remove_var(self.key),
        }
    }
}

pub(super) fn fixture_agent(
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

pub(super) fn compact_request(req: &Value) -> Value {
    fn sorted_string_array(value: &Value) -> Vec<String> {
        let mut out: Vec<String> = value
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect();
        out.sort();
        out
    }

    fn sorted_object_keys(value: &Value) -> Vec<String> {
        let mut out: Vec<String> = value
            .as_object()
            .into_iter()
            .flatten()
            .map(|(k, _)| k.to_string())
            .collect();
        out.sort();
        out
    }

    fn tool_schema_snapshot(tool: &Value) -> Value {
        let name = tool
            .get("function")
            .and_then(|f| f.get("name"))
            .and_then(|v| v.as_str())
            .unwrap_or("<missing>");
        let params = tool
            .get("function")
            .and_then(|f| f.get("parameters"))
            .unwrap_or(&Value::Null);

        json!({
            "name": name,
            "type": tool.get("type").and_then(|v| v.as_str()).unwrap_or("<missing>"),
            "param_type": params.get("type").and_then(|v| v.as_str()).unwrap_or("<missing>"),
            "required": sorted_string_array(params.get("required").unwrap_or(&Value::Null)),
            "properties": sorted_object_keys(params.get("properties").unwrap_or(&Value::Null)),
            "additional_properties": params.get("additionalProperties").cloned().unwrap_or(Value::Null),
        })
    }

    let tools: Vec<Value> = req
        .get("tools")
        .and_then(|v| v.as_array())
        .into_iter()
        .flatten()
        .map(tool_schema_snapshot)
        .collect();
    let mut tools = tools;
    tools.sort_by(|a, b| {
        a["name"]
            .as_str()
            .unwrap_or("")
            .cmp(b["name"].as_str().unwrap_or(""))
    });

    let messages: Vec<&Value> = req
        .get("messages")
        .and_then(|v| v.as_array())
        .into_iter()
        .flatten()
        .collect();

    let message_roles: Vec<String> = messages
        .iter()
        .filter_map(|m| {
            m.get("role")
                .and_then(|r| r.as_str())
                .map(|s| s.to_string())
        })
        .collect();

    let mut assistant_tool_calls: Vec<Value> = Vec::new();
    let mut tool_messages: Vec<Value> = Vec::new();

    for msg in messages {
        let role = msg.get("role").and_then(|r| r.as_str()).unwrap_or("");
        if role == "assistant" {
            let calls = msg.get("tool_calls").and_then(|v| v.as_array());
            for call in calls.into_iter().flatten() {
                let args_raw = call
                    .get("function")
                    .and_then(|f| f.get("arguments"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let args = serde_json::from_str::<Value>(args_raw)
                    .unwrap_or_else(|_| Value::String(args_raw.to_string()));

                assistant_tool_calls.push(json!({
                    "id": call.get("id").cloned().unwrap_or(Value::Null),
                    "name": call.get("function").and_then(|f| f.get("name")).cloned().unwrap_or(Value::Null),
                    "arguments": args,
                }));
            }
        }

        if role == "tool" {
            let content_raw = msg.get("content").and_then(|v| v.as_str()).unwrap_or("");
            let content = serde_json::from_str::<Value>(content_raw)
                .unwrap_or_else(|_| Value::String(content_raw.to_string()));
            tool_messages.push(json!({
                "name": msg.get("name").cloned().unwrap_or(Value::Null),
                "tool_call_id": msg.get("tool_call_id").cloned().unwrap_or(Value::Null),
                "content": content,
            }));
        }
    }

    json!({
        "model": req.get("model").cloned().unwrap_or(Value::Null),
        "temperature": req.get("temperature").and_then(|v| v.as_f64()),
        "tools": tools,
        "message_roles": message_roles,
        "assistant_tool_calls": assistant_tool_calls,
        "tool_messages": tool_messages,
    })
}
