use rexos::router::TaskKind;
use rexos::security::LeakMode;
use serde_json::json;
use serial_test::serial;

use super::common::{compact_request, fixture_agent, EnvVarGuard};
use super::support;

#[tokio::test]
#[serial]
async fn replay_fixture_drives_session_and_tool_calls() {
    let fixture = support::openai_compat_fixture::load_json_array(include_str!(
        "../fixtures/replay/session_write_file.json"
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
    assert_eq!(
        compact_request(&requests[0]),
        json!({
            "model": "fixture-model",
            "temperature": 0.0,
            "tools": [{
                "name": "fs_write",
                "type": "function",
                "param_type": "object",
                "required": ["content", "path"],
                "properties": ["content", "path"],
                "additional_properties": false,
            }],
            "message_roles": ["user"],
            "assistant_tool_calls": [],
            "tool_messages": [],
        })
    );
    assert_eq!(
        compact_request(&requests[1]),
        json!({
            "model": "fixture-model",
            "temperature": 0.0,
            "tools": [{
                "name": "fs_write",
                "type": "function",
                "param_type": "object",
                "required": ["content", "path"],
                "properties": ["content", "path"],
                "additional_properties": false,
            }],
            "message_roles": ["user", "assistant", "tool"],
            "assistant_tool_calls": [{
                "id": "call_1",
                "name": "fs_write",
                "arguments": { "path": "hello.txt", "content": "hello" },
            }],
            "tool_messages": [{
                "name": "fs_write",
                "tool_call_id": "call_1",
                "content": "ok",
            }],
        })
    );

    server.abort();
}

#[tokio::test]
#[serial]
async fn replay_fixture_blocks_tool_not_in_allowed_tools() {
    let fixture = support::openai_compat_fixture::load_json_array(include_str!(
        "../fixtures/replay/session_tool_not_allowed.json"
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
    assert_eq!(
        compact_request(&requests[0]),
        json!({
            "model": "fixture-model",
            "temperature": 0.0,
            "tools": [{
                "name": "fs_read",
                "type": "function",
                "param_type": "object",
                "required": ["path"],
                "properties": ["path"],
                "additional_properties": false,
            }],
            "message_roles": ["user"],
            "assistant_tool_calls": [],
            "tool_messages": [],
        })
    );

    server.abort();
}

#[tokio::test]
#[serial]
async fn replay_fixture_surfaces_tool_failure_errors() {
    let fixture = support::openai_compat_fixture::load_json_array(include_str!(
        "../fixtures/replay/session_tool_failed_invalid_path.json"
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
    assert!(
        err_text.contains("fs_write"),
        "expected tool name in error, got: {err_text}"
    );

    let requests = server.requests.lock().unwrap().clone();
    assert_eq!(requests.len(), 1, "expected one chat completions call");
    assert_eq!(
        compact_request(&requests[0]),
        json!({
            "model": "fixture-model",
            "temperature": 0.0,
            "tools": [{
                "name": "fs_write",
                "type": "function",
                "param_type": "object",
                "required": ["content", "path"],
                "properties": ["content", "path"],
                "additional_properties": false,
            }],
            "message_roles": ["user"],
            "assistant_tool_calls": [],
            "tool_messages": [],
        })
    );

    server.abort();
}

#[tokio::test]
#[serial]
async fn replay_fixture_executes_mcp_tool_calls() {
    let fixture = support::openai_compat_fixture::load_json_array(include_str!(
        "../fixtures/replay/session_mcp_echo.json"
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
    assert_eq!(
        compact_request(&requests[0]),
        json!({
            "model": "fixture-model",
            "temperature": 0.0,
            "tools": [{
                "name": "mcp_stub__echo",
                "type": "function",
                "param_type": "object",
                "required": ["text"],
                "properties": ["text"],
                "additional_properties": false,
            }],
            "message_roles": ["user"],
            "assistant_tool_calls": [],
            "tool_messages": [],
        })
    );
    assert_eq!(
        compact_request(&requests[1]),
        json!({
            "model": "fixture-model",
            "temperature": 0.0,
            "tools": [{
                "name": "mcp_stub__echo",
                "type": "function",
                "param_type": "object",
                "required": ["text"],
                "properties": ["text"],
                "additional_properties": false,
            }],
            "message_roles": ["user", "assistant", "tool"],
            "assistant_tool_calls": [{
                "id": "call_1",
                "name": "mcp_stub__echo",
                "arguments": { "text": "yo" },
            }],
            "tool_messages": [{
                "name": "mcp_stub__echo",
                "tool_call_id": "call_1",
                "content": { "content": [{ "type": "text", "text": "yo" }] },
            }],
        })
    );

    server.abort();
}

#[tokio::test]
#[serial]
async fn replay_fixture_enforces_tool_approval_for_dangerous_tools() {
    let _mode = EnvVarGuard::set("LOOPFORGE_APPROVAL_MODE", "enforce");
    let _allow = EnvVarGuard::set("LOOPFORGE_APPROVAL_ALLOW", "");

    let fixture = support::openai_compat_fixture::load_json_array(include_str!(
        "../fixtures/replay/session_tool_approval_required.json"
    ));
    let server = support::openai_compat_fixture::FixtureServer::spawn(fixture).await;

    let tmp = tempfile::tempdir().unwrap();
    let (agent, _paths, workspace_root) = fixture_agent(
        &tmp,
        server.base_url.clone(),
        rexos::security::SecurityConfig::default(),
    );

    let session_id = "s-replay-approval";
    agent
        .set_session_allowed_tools(session_id, vec!["shell".to_string()])
        .unwrap();

    let err = agent
        .run_session(
            workspace_root,
            session_id,
            None,
            "run shell",
            TaskKind::Coding,
        )
        .await
        .unwrap_err();
    let err_text = err.to_string();
    assert!(
        err_text.contains("approval required for dangerous tool `shell`"),
        "expected approval error, got: {err_text}"
    );

    let requests = server.requests.lock().unwrap().clone();
    assert_eq!(requests.len(), 1, "expected one chat completions call");
    assert_eq!(
        compact_request(&requests[0]),
        json!({
            "model": "fixture-model",
            "temperature": 0.0,
            "tools": [{
                "name": "shell",
                "type": "function",
                "param_type": "object",
                "required": ["command"],
                "properties": ["command", "timeout_ms"],
                "additional_properties": false,
            }],
            "message_roles": ["user"],
            "assistant_tool_calls": [],
            "tool_messages": [],
        })
    );

    server.abort();
}

#[tokio::test]
#[serial]
async fn replay_fixture_blocks_leak_guard_in_enforce_mode() {
    let fixture = support::openai_compat_fixture::load_json_array(include_str!(
        "../fixtures/replay/session_leak_guard_enforce.json"
    ));
    let server = support::openai_compat_fixture::FixtureServer::spawn(fixture).await;

    let tmp = tempfile::tempdir().unwrap();
    let mut security = rexos::security::SecurityConfig::default();
    security.leaks.mode = LeakMode::Enforce;

    let (agent, _paths, workspace_root) = fixture_agent(&tmp, server.base_url.clone(), security);
    std::fs::write(
        workspace_root.join("secret.txt"),
        "secret=sk-01234567890123456789",
    )
    .unwrap();

    let session_id = "s-replay-leak-guard";
    agent
        .set_session_allowed_tools(session_id, vec!["fs_read".to_string()])
        .unwrap();

    let err = agent
        .run_session(
            workspace_root,
            session_id,
            None,
            "read secret",
            TaskKind::Coding,
        )
        .await
        .unwrap_err();
    let err_text = err.to_string();
    assert!(
        err_text.contains("tool output blocked by leak guard"),
        "expected leak guard error, got: {err_text}"
    );

    let requests = server.requests.lock().unwrap().clone();
    assert_eq!(requests.len(), 1, "expected one chat completions call");
    assert_eq!(
        compact_request(&requests[0]),
        json!({
            "model": "fixture-model",
            "temperature": 0.0,
            "tools": [{
                "name": "fs_read",
                "type": "function",
                "param_type": "object",
                "required": ["path"],
                "properties": ["path"],
                "additional_properties": false,
            }],
            "message_roles": ["user"],
            "assistant_tool_calls": [],
            "tool_messages": [],
        })
    );

    server.abort();
}
