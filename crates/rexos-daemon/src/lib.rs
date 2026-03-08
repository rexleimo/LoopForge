mod config;
mod handlers;
mod middleware;
mod rate_limit;
mod state;

use std::net::SocketAddr;
use std::time::Instant;

use anyhow::Context;
use axum::middleware as axum_middleware;
use axum::routing::get;
use axum::Router;

pub use config::DaemonConfig;
use handlers::{healthz, status};
use middleware::{add_security_headers, enforce_auth_and_rate_limit};
use rate_limit::RateLimiter;
use state::AppState;

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
        .layer(axum_middleware::from_fn_with_state(
            state.clone(),
            enforce_auth_and_rate_limit,
        ))
        .layer(axum_middleware::from_fn(add_security_headers))
        .with_state(state)
}

pub async fn serve(addr: SocketAddr) -> anyhow::Result<()> {
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .with_context(|| format!("bind {addr}"))?;
    axum::serve(listener, app()).await.context("serve http")?;
    Ok(())
}
