use std::path::{Path, PathBuf};

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

pub fn write_mcp_stub(workspace: &Path) -> PathBuf {
    let path = workspace.join("mcp_stub.py");
    std::fs::write(&path, MCP_STUB_PY).expect("write mcp stub script");
    path
}

pub fn mcp_config_json(script: &Path) -> String {
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
