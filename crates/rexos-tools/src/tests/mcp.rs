use std::collections::BTreeSet;
use std::path::Path;

use super::*;

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
    elif method == "resources/list":
        send({
            "jsonrpc": "2.0",
            "id": msg_id,
            "result": {
                "resources": [{"uri": "mem://hello", "name": "hello", "mimeType": "text/plain"}],
                "nextCursor": None,
            }
        })
    elif method == "resources/read":
        uri = (params.get("uri") or "").strip()
        if uri == "mem://hello":
            send({"jsonrpc": "2.0", "id": msg_id, "result": {"contents": [{"uri": uri, "mimeType": "text/plain", "text": "hello"}]}})
        else:
            send({"jsonrpc": "2.0", "id": msg_id, "error": {"code": -32602, "message": "unknown uri"}})
    elif method == "prompts/list":
        send({
            "jsonrpc": "2.0",
            "id": msg_id,
            "result": {
                "prompts": [{"name": "greet", "description": "greet prompt"}],
                "nextCursor": None,
            }
        })
    elif method == "prompts/get":
        name = (params.get("name") or "").strip()
        if name == "greet":
            send({"jsonrpc": "2.0", "id": msg_id, "result": {"messages": [{"role": "user", "content": [{"type": "text", "text": "hi"}]}]}})
        else:
            send({"jsonrpc": "2.0", "id": msg_id, "error": {"code": -32602, "message": "unknown prompt"}})
    else:
        send({"jsonrpc": "2.0", "id": msg_id, "error": {"code": -32601, "message": "unknown method"}})
"#;

fn python_exe() -> &'static str {
    if cfg!(windows) {
        "python"
    } else {
        "python3"
    }
}

fn write_mcp_stub(root: &Path) -> std::path::PathBuf {
    let path = root.join("mcp_stub.py");
    std::fs::write(&path, MCP_STUB_PY).expect("write mcp stub script");
    path
}

fn mcp_config_json(script: &Path) -> String {
    serde_json::json!({
        "servers": {
            "stub": {
                "command": python_exe(),
                "args": ["-u", script.to_string_lossy()],
                "cwd": ".",
            }
        }
    })
    .to_string()
}

#[tokio::test]
async fn toolset_mcp_definitions_include_wrappers_and_remote_tools() {
    let tmp = tempfile::tempdir().unwrap();
    let stub = write_mcp_stub(tmp.path());
    let config_json = mcp_config_json(&stub);

    let mut tools = Toolset::new(tmp.path().to_path_buf()).unwrap();
    tools.enable_mcp_from_json(&config_json).await.unwrap();

    let defs = tools.definitions();
    let names: BTreeSet<String> = defs.into_iter().map(|d| d.function.name).collect();

    for name in [
        "mcp_servers_list",
        "mcp_resources_list",
        "mcp_resources_read",
        "mcp_prompts_list",
        "mcp_prompts_get",
        "mcp_stub__echo",
    ] {
        assert!(names.contains(name), "missing mcp tool definition: {name}");
    }
}

#[tokio::test]
async fn toolset_can_call_mcp_tools_and_wrappers() {
    let tmp = tempfile::tempdir().unwrap();
    let stub = write_mcp_stub(tmp.path());
    let config_json = mcp_config_json(&stub);

    let mut tools = Toolset::new(tmp.path().to_path_buf()).unwrap();
    tools.enable_mcp_from_json(&config_json).await.unwrap();

    let echoed = tools
        .call("mcp_stub__echo", r#"{ "text": "yo" }"#)
        .await
        .unwrap();
    let echoed: serde_json::Value = serde_json::from_str(&echoed).unwrap();
    assert_eq!(echoed["content"][0]["text"], "yo");

    let servers = tools.call("mcp_servers_list", r#"{}"#).await.unwrap();
    let servers: Vec<String> = serde_json::from_str(&servers).unwrap();
    assert_eq!(servers, vec!["stub".to_string()]);

    let resources = tools.call("mcp_resources_list", r#"{}"#).await.unwrap();
    let resources: serde_json::Value = serde_json::from_str(&resources).unwrap();
    assert_eq!(resources[0]["server"], "stub");
    assert_eq!(resources[0]["resources"][0]["uri"], "mem://hello");

    let read = tools
        .call("mcp_resources_read", r#"{ "uri": "mem://hello" }"#)
        .await
        .unwrap();
    let read: serde_json::Value = serde_json::from_str(&read).unwrap();
    assert_eq!(read["server"], "stub");
    assert_eq!(read["result"]["contents"][0]["text"], "hello");

    let prompts = tools.call("mcp_prompts_list", r#"{}"#).await.unwrap();
    let prompts: serde_json::Value = serde_json::from_str(&prompts).unwrap();
    assert_eq!(prompts[0]["server"], "stub");
    assert_eq!(prompts[0]["prompts"][0]["name"], "greet");

    let prompt = tools
        .call("mcp_prompts_get", r#"{ "name": "greet" }"#)
        .await
        .unwrap();
    let prompt: serde_json::Value = serde_json::from_str(&prompt).unwrap();
    assert_eq!(prompt["server"], "stub");
    assert_eq!(prompt["result"]["messages"][0]["content"][0]["text"], "hi");
}
