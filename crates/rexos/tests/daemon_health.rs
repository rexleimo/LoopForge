use axum::body::Body;
use axum::http::HeaderMap;
use axum::http::Request;
use axum::http::StatusCode;
use tower::ServiceExt;

fn assert_security_headers(headers: &HeaderMap) {
    assert_eq!(
        headers
            .get("x-content-type-options")
            .and_then(|value| value.to_str().ok()),
        Some("nosniff")
    );
    assert_eq!(
        headers
            .get("x-frame-options")
            .and_then(|value| value.to_str().ok()),
        Some("DENY")
    );
    assert_eq!(
        headers
            .get("referrer-policy")
            .and_then(|value| value.to_str().ok()),
        Some("no-referrer")
    );
    assert_eq!(
        headers
            .get("permissions-policy")
            .and_then(|value| value.to_str().ok()),
        Some("geolocation=(), microphone=(), camera=()")
    );
}

#[tokio::test]
async fn healthz_returns_ok() {
    let app = rexos::daemon::app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/healthz")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert_security_headers(response.headers());

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let v: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(v["status"], "ok");
}

#[tokio::test]
async fn status_returns_ok_and_security_headers() {
    let app = rexos::daemon::app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/status")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert_security_headers(response.headers());

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let v: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(v["status"], "ok");
    assert!(v["uptime_ms"].is_u64());
}

#[tokio::test]
async fn healthz_requires_bearer_token_when_auth_is_enabled() {
    let app = rexos::daemon::app_with_config(rexos::daemon::DaemonConfig {
        auth_bearer_token: Some("secret-token".to_string()),
        rate_limit_per_minute: 60,
    });

    let unauthorized = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/healthz")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(unauthorized.status(), StatusCode::UNAUTHORIZED);
    assert_security_headers(unauthorized.headers());

    let authorized = app
        .oneshot(
            Request::builder()
                .uri("/healthz")
                .header("authorization", "Bearer secret-token")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(authorized.status(), StatusCode::OK);
    assert_security_headers(authorized.headers());
}

#[tokio::test]
async fn healthz_applies_rate_limit_per_client() {
    let app = rexos::daemon::app_with_config(rexos::daemon::DaemonConfig {
        auth_bearer_token: None,
        rate_limit_per_minute: 1,
    });

    let first = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/healthz")
                .header("x-forwarded-for", "203.0.113.10")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(first.status(), StatusCode::OK);

    let second = app
        .oneshot(
            Request::builder()
                .uri("/healthz")
                .header("x-forwarded-for", "203.0.113.10")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(second.status(), StatusCode::TOO_MANY_REQUESTS);
    assert_security_headers(second.headers());
}
