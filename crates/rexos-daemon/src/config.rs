#[derive(Debug, Clone)]
pub struct DaemonConfig {
    pub auth_bearer_token: Option<String>,
    pub rate_limit_per_minute: u32,
}

impl Default for DaemonConfig {
    fn default() -> Self {
        let auth_bearer_token = std::env::var("LOOPFORGE_DAEMON_AUTH_TOKEN")
            .ok()
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty());
        let rate_limit_per_minute = std::env::var("LOOPFORGE_DAEMON_RATE_LIMIT_PER_MINUTE")
            .ok()
            .and_then(|value| value.parse::<u32>().ok())
            .filter(|value| *value > 0)
            .unwrap_or(120);

        Self {
            auth_bearer_token,
            rate_limit_per_minute,
        }
    }
}
