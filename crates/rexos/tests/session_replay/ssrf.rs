use rexos::router::TaskKind;
use serde_json::json;
use serial_test::serial;

use super::common::{compact_request, fixture_agent, EnvVarGuard};
use super::support;

#[tokio::test]
#[serial]
async fn replay_fixture_denies_web_fetch_loopback_when_allow_private_false() {
    // Ensure this test doesn't depend on external approval-mode env.
    let _mode = EnvVarGuard::set("LOOPFORGE_APPROVAL_MODE", "off");
    let _allow = EnvVarGuard::set("LOOPFORGE_APPROVAL_ALLOW", "");

    let fixture = support::openai_compat_fixture::load_json_array(include_str!(
        "../fixtures/replay/session_ssrf_web_fetch_loopback_denied.json"
    ));
    let server = support::openai_compat_fixture::FixtureServer::spawn(fixture).await;

    let tmp = tempfile::tempdir().unwrap();
    let (agent, _paths, workspace_root) = fixture_agent(
        &tmp,
        server.base_url.clone(),
        rexos::security::SecurityConfig::default(),
    );

    let session_id = "s-replay-ssrf-web-fetch";
    agent
        .set_session_allowed_tools(session_id, vec!["web_fetch".to_string()])
        .unwrap();

    let err = agent
        .run_session(
            workspace_root,
            session_id,
            None,
            "fetch loopback",
            TaskKind::Coding,
        )
        .await
        .unwrap_err();
    let err_text = err.to_string();
    assert!(
        err_text.contains("loopback/private address"),
        "expected loopback/private deny, got: {err_text}"
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
async fn replay_fixture_denies_a2a_discover_loopback_when_allow_private_false() {
    let _mode = EnvVarGuard::set("LOOPFORGE_APPROVAL_MODE", "off");
    let _allow = EnvVarGuard::set("LOOPFORGE_APPROVAL_ALLOW", "");

    let fixture = support::openai_compat_fixture::load_json_array(include_str!(
        "../fixtures/replay/session_ssrf_a2a_discover_loopback_denied.json"
    ));
    let server = support::openai_compat_fixture::FixtureServer::spawn(fixture).await;

    let tmp = tempfile::tempdir().unwrap();
    let (agent, _paths, workspace_root) = fixture_agent(
        &tmp,
        server.base_url.clone(),
        rexos::security::SecurityConfig::default(),
    );

    let session_id = "s-replay-ssrf-a2a-discover";
    agent
        .set_session_allowed_tools(session_id, vec!["a2a_discover".to_string()])
        .unwrap();

    let err = agent
        .run_session(
            workspace_root,
            session_id,
            None,
            "discover loopback",
            TaskKind::Coding,
        )
        .await
        .unwrap_err();
    let err_text = err.to_string();
    assert!(
        err_text.contains("loopback/private address"),
        "expected loopback/private deny, got: {err_text}"
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
