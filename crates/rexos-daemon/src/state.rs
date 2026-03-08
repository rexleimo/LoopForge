use std::time::Instant;

use serde::Serialize;

use crate::config::DaemonConfig;
use crate::rate_limit::RateLimiter;

#[derive(Debug, Clone)]
pub(super) struct AppState {
    pub(super) started_at: Instant,
    pub(super) config: DaemonConfig,
    pub(super) limiter: RateLimiter,
}

#[derive(Debug, Serialize)]
pub(super) struct HealthzResponse {
    pub(super) status: &'static str,
}

#[derive(Debug, Serialize)]
pub(super) struct StatusResponse {
    pub(super) status: &'static str,
    pub(super) uptime_ms: u128,
}
