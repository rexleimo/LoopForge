use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Instant;

use anyhow::Context;
use axum::extract::State;
use axum::http::header;
use axum::http::header::HeaderName;
use axum::http::HeaderValue;
use axum::http::StatusCode;
use axum::middleware::{self, Next};
use axum::response::IntoResponse;
use axum::response::Response;
use axum::routing::get;
use axum::{Json, Router};
use serde::Serialize;

#[derive(Debug, Clone)]
struct AppState {
    started_at: Instant,
    config: DaemonConfig,
    limiter: RateLimiter,
}

#[derive(Debug, Clone)]
pub struct DaemonConfig {
    pub auth_bearer_token: Option<String>,
    pub rate_limit_per_minute: u32,
}

impl Default for DaemonConfig {
    fn default() -> Self {
        let auth_bearer_token = std::env::var("LOOPFORGE_DAEMON_AUTH_TOKEN")
            .ok()
            .map(|v| v.trim().to_string())
            .filter(|v| !v.is_empty());
        let rate_limit_per_minute = std::env::var("LOOPFORGE_DAEMON_RATE_LIMIT_PER_MINUTE")
            .ok()
            .and_then(|v| v.parse::<u32>().ok())
            .filter(|v| *v > 0)
            .unwrap_or(120);

        Self {
            auth_bearer_token,
            rate_limit_per_minute,
        }
    }
}

#[derive(Debug, Clone)]
struct RateLimiter {
    limit_per_minute: u32,
    state: Arc<tokio::sync::Mutex<RateLimitState>>,
}

#[derive(Debug)]
struct RateLimitState {
    window_started_at: Instant,
    counts: HashMap<String, u32>,
}

impl RateLimiter {
    fn new(limit_per_minute: u32) -> Self {
        Self {
            limit_per_minute: limit_per_minute.max(1),
            state: Arc::new(tokio::sync::Mutex::new(RateLimitState {
                window_started_at: Instant::now(),
                counts: HashMap::new(),
            })),
        }
    }

    async fn allow(&self, client: &str) -> bool {
        let now = Instant::now();
        let mut state = self.state.lock().await;

        if now.duration_since(state.window_started_at).as_secs() >= 60 {
            state.window_started_at = now;
            state.counts.clear();
        }

        let count = state.counts.entry(client.to_string()).or_insert(0);
        if *count >= self.limit_per_minute {
            return false;
        }
        *count += 1;
        true
    }
}

#[derive(Debug, Serialize)]
struct HealthzResponse {
    status: &'static str,
}

#[derive(Debug, Serialize)]
struct StatusResponse {
    status: &'static str,
    uptime_ms: u128,
}

pub fn app() -> Router {
    app_with_config(DaemonConfig::default())
}

pub fn app_with_config(config: DaemonConfig) -> Router {
    let state = AppState {
        started_at: Instant::now(),
        limiter: RateLimiter::new(config.rate_limit_per_minute),
        config,
    };

    Router::new()
        .route("/healthz", get(healthz))
        .route("/status", get(status))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            enforce_auth_and_rate_limit,
        ))
        .layer(middleware::from_fn(add_security_headers))
        .with_state(state)
}

pub async fn serve(addr: SocketAddr) -> anyhow::Result<()> {
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .with_context(|| format!("bind {addr}"))?;
    axum::serve(listener, app())
        .await
        .context("serve http")?;
    Ok(())
}

async fn healthz() -> Json<HealthzResponse> {
    Json(HealthzResponse { status: "ok" })
}

async fn status(State(state): State<AppState>) -> Json<StatusResponse> {
    Json(StatusResponse {
        status: "ok",
        uptime_ms: state.started_at.elapsed().as_millis(),
    })
}

async fn enforce_auth_and_rate_limit(
    State(state): State<AppState>,
    request: axum::http::Request<axum::body::Body>,
    next: Next,
) -> Response {
    if let Some(token) = state.config.auth_bearer_token.as_deref() {
        let expected = format!("Bearer {token}");
        let authorized = request
            .headers()
            .get(header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .map(|value| value == expected)
            .unwrap_or(false);
        if !authorized {
            return (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({"error":"unauthorized"})),
            )
                .into_response();
        }
    }

    let client_key = request
        .headers()
        .get("x-forwarded-for")
        .and_then(|value| value.to_str().ok())
        .and_then(|raw| raw.split(',').next())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or("local");

    if !state.limiter.allow(client_key).await {
        return (
            StatusCode::TOO_MANY_REQUESTS,
            Json(serde_json::json!({"error":"rate_limited"})),
        )
            .into_response();
    }

    next.run(request).await
}

async fn add_security_headers(request: axum::http::Request<axum::body::Body>, next: Next) -> Response {
    let mut response = next.run(request).await;
    let headers = response.headers_mut();
    headers.insert(
        header::X_CONTENT_TYPE_OPTIONS,
        HeaderValue::from_static("nosniff"),
    );
    headers.insert(header::X_FRAME_OPTIONS, HeaderValue::from_static("DENY"));
    headers.insert(
        header::REFERRER_POLICY,
        HeaderValue::from_static("no-referrer"),
    );
    headers.insert(
        HeaderName::from_static("permissions-policy"),
        HeaderValue::from_static("geolocation=(), microphone=(), camera=()"),
    );
    response
}
