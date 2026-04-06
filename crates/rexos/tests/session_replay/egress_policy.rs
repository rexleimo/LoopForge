use rexos::router::TaskKind;
use rexos::security::{EgressConfig, EgressRule, SecurityConfig};
use serde_json::json;
use serial_test::serial;

use super::common::{compact_request, fixture_agent};
use super::support;

#[tokio::test]
#[serial]
async fn replay_fixture_blocks_egress_policy_rule_mismatches() {
    let fixture = support::openai_compat_fixture::load_json_array(include_str!(
        "../fixtures/replay/session_egress_policy_block.json"
    ));
    let server = support::openai_compat_fixture::FixtureServer::spawn(fixture).await;

    let tmp = tempfile::tempdir().unwrap();
    let security = SecurityConfig {
        egress: EgressConfig {
            rules: vec![EgressRule {
                tool: "web_fetch".to_string(),
                host: "example.com".to_string(),
                path_prefix: "/".to_string(),
                methods: vec!["GET".to_string()],
            }],
        },
        ..Default::default()
    };

    let (agent, _paths, workspace_root) = fixture_agent(&tmp, server.base_url.clone(), security);

    let session_id = "s-replay-egress";
    agent
        .set_session_allowed_tools(session_id, vec!["web_fetch".to_string()])
        .unwrap();

    let err = agent
        .run_session(
            workspace_root,
            session_id,
            None,
            "fetch localhost",
            TaskKind::Coding,
        )
        .await
        .unwrap_err();
    let err_text = err.to_string();
    assert!(
        err_text.contains("egress host not allowed"),
        "expected egress host block, got: {err_text}"
    );
    assert!(
        err_text.contains("web_fetch"),
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
                "name": "web_fetch",
                "type": "function",
                "param_type": "object",
                "required": ["url"],
                "properties": ["allow_private", "max_bytes", "timeout_ms", "url"],
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
async fn replay_fixture_blocks_egress_policy_path_prefix_mismatches() {
    let fixture = support::openai_compat_fixture::load_json_array(include_str!(
        "../fixtures/replay/session_egress_policy_path_block.json"
    ));
    let server = support::openai_compat_fixture::FixtureServer::spawn(fixture).await;

    let tmp = tempfile::tempdir().unwrap();
    let security = SecurityConfig {
        egress: EgressConfig {
            rules: vec![EgressRule {
                tool: "web_fetch".to_string(),
                host: "127.0.0.1".to_string(),
                path_prefix: "/ok".to_string(),
                methods: vec!["GET".to_string()],
            }],
        },
        ..Default::default()
    };

    let (agent, _paths, workspace_root) = fixture_agent(&tmp, server.base_url.clone(), security);

    let session_id = "s-replay-egress-path";
    agent
        .set_session_allowed_tools(session_id, vec!["web_fetch".to_string()])
        .unwrap();

    let err = agent
        .run_session(
            workspace_root,
            session_id,
            None,
            "fetch disallowed path",
            TaskKind::Coding,
        )
        .await
        .unwrap_err();
    let err_text = err.to_string();
    assert!(
        err_text.contains("egress path not allowed"),
        "expected egress path block, got: {err_text}"
    );
    assert!(
        err_text.contains("web_fetch"),
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
                "name": "web_fetch",
                "type": "function",
                "param_type": "object",
                "required": ["url"],
                "properties": ["allow_private", "max_bytes", "timeout_ms", "url"],
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
async fn replay_fixture_blocks_egress_policy_method_mismatches() {
    let fixture = support::openai_compat_fixture::load_json_array(include_str!(
        "../fixtures/replay/session_egress_policy_method_block.json"
    ));
    let server = support::openai_compat_fixture::FixtureServer::spawn(fixture).await;

    let tmp = tempfile::tempdir().unwrap();
    let security = SecurityConfig {
        egress: EgressConfig {
            rules: vec![EgressRule {
                tool: "a2a_send".to_string(),
                host: "127.0.0.1".to_string(),
                path_prefix: "/".to_string(),
                methods: vec!["GET".to_string()],
            }],
        },
        ..Default::default()
    };

    let (agent, _paths, workspace_root) = fixture_agent(&tmp, server.base_url.clone(), security);

    let session_id = "s-replay-egress-method";
    agent
        .set_session_allowed_tools(session_id, vec!["a2a_send".to_string()])
        .unwrap();

    let err = agent
        .run_session(
            workspace_root,
            session_id,
            None,
            "send a2a disallowed method",
            TaskKind::Coding,
        )
        .await
        .unwrap_err();
    let err_text = err.to_string();
    assert!(
        err_text.contains("egress method not allowed"),
        "expected egress method block, got: {err_text}"
    );
    assert!(
        err_text.contains("a2a_send"),
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
                "name": "a2a_send",
                "type": "function",
                "param_type": "object",
                "required": ["message"],
                "properties": ["agent_url", "allow_private", "message", "session_id", "url"],
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
async fn replay_fixture_blocks_a2a_discover_egress_path_mismatches() {
    let fixture = support::openai_compat_fixture::load_json_array(include_str!(
        "../fixtures/replay/session_egress_policy_a2a_discover_path_block.json"
    ));
    let server = support::openai_compat_fixture::FixtureServer::spawn(fixture).await;

    let tmp = tempfile::tempdir().unwrap();
    let security = SecurityConfig {
        egress: EgressConfig {
            rules: vec![EgressRule {
                tool: "a2a_discover".to_string(),
                host: "127.0.0.1".to_string(),
                path_prefix: "/ok".to_string(),
                methods: vec!["GET".to_string()],
            }],
        },
        ..Default::default()
    };

    let (agent, _paths, workspace_root) = fixture_agent(&tmp, server.base_url.clone(), security);

    let session_id = "s-replay-egress-a2a-discover";
    agent
        .set_session_allowed_tools(session_id, vec!["a2a_discover".to_string()])
        .unwrap();

    let err = agent
        .run_session(
            workspace_root,
            session_id,
            None,
            "discover agent card",
            TaskKind::Coding,
        )
        .await
        .unwrap_err();
    let err_text = err.to_string();
    assert!(
        err_text.contains("egress path not allowed"),
        "expected egress path block, got: {err_text}"
    );
    assert!(
        err_text.contains("/.well-known/agent.json"),
        "expected agent card path in error, got: {err_text}"
    );
    assert!(
        err_text.contains("a2a_discover"),
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
                "name": "a2a_discover",
                "type": "function",
                "param_type": "object",
                "required": ["url"],
                "properties": ["allow_private", "url"],
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
async fn replay_fixture_blocks_a2a_discover_egress_method_mismatches() {
    let fixture = support::openai_compat_fixture::load_json_array(include_str!(
        "../fixtures/replay/session_egress_policy_a2a_discover_method_block.json"
    ));
    let server = support::openai_compat_fixture::FixtureServer::spawn(fixture).await;

    let tmp = tempfile::tempdir().unwrap();
    let security = SecurityConfig {
        egress: EgressConfig {
            rules: vec![EgressRule {
                tool: "a2a_discover".to_string(),
                host: "127.0.0.1".to_string(),
                path_prefix: "/.well-known/".to_string(),
                methods: vec!["POST".to_string()],
            }],
        },
        ..Default::default()
    };

    let (agent, _paths, workspace_root) = fixture_agent(&tmp, server.base_url.clone(), security);

    let session_id = "s-replay-egress-a2a-discover-method";
    agent
        .set_session_allowed_tools(session_id, vec!["a2a_discover".to_string()])
        .unwrap();

    let err = agent
        .run_session(
            workspace_root,
            session_id,
            None,
            "discover agent card (method mismatch)",
            TaskKind::Coding,
        )
        .await
        .unwrap_err();
    let err_text = err.to_string();
    assert!(
        err_text.contains("egress method not allowed"),
        "expected egress method block, got: {err_text}"
    );
    assert!(
        err_text.contains("GET"),
        "expected method in error, got: {err_text}"
    );
    assert!(
        err_text.contains("a2a_discover"),
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
                "name": "a2a_discover",
                "type": "function",
                "param_type": "object",
                "required": ["url"],
                "properties": ["allow_private", "url"],
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
async fn replay_fixture_blocks_a2a_discover_egress_host_mismatches() {
    let fixture = support::openai_compat_fixture::load_json_array(include_str!(
        "../fixtures/replay/session_egress_policy_a2a_discover_host_block.json"
    ));
    let server = support::openai_compat_fixture::FixtureServer::spawn(fixture).await;

    let tmp = tempfile::tempdir().unwrap();
    let security = SecurityConfig {
        egress: EgressConfig {
            rules: vec![EgressRule {
                tool: "a2a_discover".to_string(),
                host: "example.com".to_string(),
                path_prefix: "/.well-known/".to_string(),
                methods: vec!["GET".to_string()],
            }],
        },
        ..Default::default()
    };

    let (agent, _paths, workspace_root) = fixture_agent(&tmp, server.base_url.clone(), security);

    let session_id = "s-replay-egress-a2a-discover-host";
    agent
        .set_session_allowed_tools(session_id, vec!["a2a_discover".to_string()])
        .unwrap();

    let err = agent
        .run_session(
            workspace_root,
            session_id,
            None,
            "discover agent card (host mismatch)",
            TaskKind::Coding,
        )
        .await
        .unwrap_err();
    let err_text = err.to_string();
    assert!(
        err_text.contains("egress host not allowed"),
        "expected egress host block, got: {err_text}"
    );
    assert!(
        err_text.contains("a2a_discover"),
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
                "name": "a2a_discover",
                "type": "function",
                "param_type": "object",
                "required": ["url"],
                "properties": ["allow_private", "url"],
                "additional_properties": false,
            }],
            "message_roles": ["user"],
            "assistant_tool_calls": [],
            "tool_messages": [],
        })
    );

    server.abort();
}
