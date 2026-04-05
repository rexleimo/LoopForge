use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use serde_json::{json, Value};

#[derive(Clone)]
struct FixtureState {
    responses: Arc<Mutex<VecDeque<Value>>>,
    requests: Arc<Mutex<Vec<Value>>>,
}

pub struct FixtureServer {
    pub base_url: String,
    pub requests: Arc<Mutex<Vec<Value>>>,
    handle: tokio::task::JoinHandle<()>,
}

impl FixtureServer {
    pub async fn spawn(responses: Vec<Value>) -> Self {
        async fn handler(
            State(state): State<FixtureState>,
            Json(payload): Json<Value>,
        ) -> impl IntoResponse {
            state.requests.lock().unwrap().push(payload);
            let next = state.responses.lock().unwrap().pop_front();
            match next {
                Some(value) => (StatusCode::OK, Json(value)).into_response(),
                None => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": "fixture exhausted" })),
                )
                    .into_response(),
            }
        }

        let state = FixtureState {
            responses: Arc::new(Mutex::new(responses.into())),
            requests: Arc::new(Mutex::new(Vec::new())),
        };

        let app = Router::new()
            .route("/v1/chat/completions", post(handler))
            .with_state(state.clone());

        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let handle = tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });

        Self {
            base_url: format!("http://{addr}/v1"),
            requests: state.requests,
            handle,
        }
    }

    pub fn abort(self) {
        self.handle.abort();
    }
}

pub fn load_json_array(raw: &str) -> Vec<Value> {
    serde_json::from_str(raw).expect("fixture JSON array")
}
