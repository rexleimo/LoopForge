use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;

#[derive(Debug, Clone)]
pub(super) struct RateLimiter {
    limit_per_minute: u32,
    state: Arc<tokio::sync::Mutex<RateLimitState>>,
}

#[derive(Debug)]
struct RateLimitState {
    window_started_at: Instant,
    counts: HashMap<String, u32>,
}

impl RateLimiter {
    pub(super) fn new(limit_per_minute: u32) -> Self {
        Self {
            limit_per_minute: limit_per_minute.max(1),
            state: Arc::new(tokio::sync::Mutex::new(RateLimitState {
                window_started_at: Instant::now(),
                counts: HashMap::new(),
            })),
        }
    }

    pub(super) async fn allow(&self, client: &str) -> bool {
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
