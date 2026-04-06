use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

#[derive(Clone, Default)]
pub(super) struct TestState {
    pub(super) calls: Arc<Mutex<u32>>,
    pub(super) last_request: Arc<Mutex<Option<serde_json::Value>>>,
}

const MCP_STUB_PY: &str = r#"
import json
import sys

def send(obj):
    sys.stdout.write(json.dumps(obj) + "\n")
    sys.stdout.flush()

for line in sys.stdin:
    line = line.strip()
    if not line:
        continue
    try:
        msg = json.loads(line)
    except Exception:
        continue

    method = msg.get("method")
    if not method:
        continue

    # Notifications have no id; ignore them.
    if "id" not in msg:
        continue

    msg_id = msg.get("id")
    params = msg.get("params") or {}

    if method == "initialize":
        send({"jsonrpc": "2.0", "id": msg_id, "result": {}})
    elif method == "tools/list":
        send({
            "jsonrpc": "2.0",
            "id": msg_id,
            "result": {
                "tools": [
                    {
                        "name": "echo",
                        "description": "Echo input text",
                        "inputSchema": {
                            "type": "object",
                            "properties": {"text": {"type": "string"}},
                            "required": ["text"],
                            "additionalProperties": False
                        }
                    }
                ]
            }
        })
    elif method == "tools/call":
        name = params.get("name")
        arguments = params.get("arguments") or {}
        if name == "echo":
            send({
                "jsonrpc": "2.0",
                "id": msg_id,
                "result": {"content": [{"type": "text", "text": arguments.get("text", "")}]}
            })
        else:
            send({"jsonrpc": "2.0", "id": msg_id, "error": {"code": -32601, "message": "unknown tool"}})
    else:
        send({"jsonrpc": "2.0", "id": msg_id, "error": {"code": -32601, "message": "unknown method"}})
"#;

fn mcp_python_exe() -> &'static str {
    if cfg!(windows) {
        "python"
    } else {
        "python3"
    }
}

pub(super) fn write_mcp_stub(workspace: &std::path::Path) -> std::path::PathBuf {
    let path = workspace.join("mcp_stub.py");
    std::fs::write(&path, MCP_STUB_PY).expect("write mcp stub script");
    path
}

pub(super) fn mcp_config_json(script: &std::path::Path) -> String {
    serde_json::json!({
        "servers": {
            "stub": {
                "command": mcp_python_exe(),
                "args": ["-u", script.to_string_lossy()],
                "cwd": ".",
            }
        }
    })
    .to_string()
}

pub(super) fn workspace_and_paths(
    tmp: &tempfile::TempDir,
) -> (std::path::PathBuf, rexos::paths::RexosPaths) {
    let workspace = tmp.path().join("workspace");
    std::fs::create_dir_all(&workspace).unwrap();

    let home = tmp.path().join("home");
    let paths = rexos::paths::RexosPaths {
        base_dir: home.join(".loopforge"),
    };
    paths.ensure_dirs().unwrap();

    (workspace, paths)
}

pub(super) fn default_agent(
    paths: &rexos::paths::RexosPaths,
    base_url: String,
) -> rexos::agent::AgentRuntime {
    let memory = rexos::memory::MemoryStore::open_or_create(paths).unwrap();
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
