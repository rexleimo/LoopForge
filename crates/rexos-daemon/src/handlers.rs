use axum::extract::State;
use axum::Json;

use crate::state::{AppState, HealthzResponse, StatusResponse};

pub(super) async fn healthz() -> Json<HealthzResponse> {
    Json(HealthzResponse { status: "ok" })
}

pub(super) async fn status(State(state): State<AppState>) -> Json<StatusResponse> {
    Json(StatusResponse {
        status: "ok",
        uptime_ms: state.started_at.elapsed().as_millis(),
    })
}
