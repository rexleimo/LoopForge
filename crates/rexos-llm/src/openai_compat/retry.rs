use std::time::Duration;

pub(super) fn openai_compat_timeout() -> Duration {
    const DEFAULT_SECS: u64 = 600;
    match std::env::var("LOOPFORGE_OPENAI_COMPAT_TIMEOUT_SECS") {
        Ok(raw) => match raw.trim().parse::<u64>() {
            Ok(secs) if secs > 0 => Duration::from_secs(secs),
            _ => Duration::from_secs(DEFAULT_SECS),
        },
        Err(_) => Duration::from_secs(DEFAULT_SECS),
    }
}

pub(super) fn llm_retry_max() -> u32 {
    const DEFAULT: u32 = 2;
    match std::env::var("LOOPFORGE_LLM_RETRY_MAX") {
        Ok(value) => value.trim().parse::<u32>().ok().unwrap_or(DEFAULT),
        Err(_) => DEFAULT,
    }
}

pub(super) fn is_retryable_status(status: reqwest::StatusCode) -> bool {
    matches!(status.as_u16(), 429 | 500 | 502 | 503 | 504)
}

pub(super) fn is_retryable_reqwest_error(err: &reqwest::Error) -> bool {
    err.is_timeout() || err.is_connect()
}

pub(super) async fn sleep_retry_backoff(retry_number: u32) {
    let base_ms: u64 = 250;
    let cap_ms: u64 = 2_000;

    let shift: u32 = retry_number.saturating_sub(1).min(31);
    let exp = 1u64.checked_shl(shift).unwrap_or(u64::MAX);
    let delay_ms = base_ms.saturating_mul(exp).min(cap_ms);

    let jitter_window = (delay_ms / 4).max(1);
    let jitter = match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
        Ok(duration) => (duration.subsec_nanos() as u64) % jitter_window,
        Err(_) => 0,
    };

    tokio::time::sleep(Duration::from_millis(delay_ms + jitter)).await;
}

pub(super) fn truncate_one_line(value: &str, max_len: usize) -> String {
    let mut out = String::new();
    for ch in value.chars() {
        if ch == '\n' || ch == '\r' {
            break;
        }
        out.push(ch);
        if out.len() >= max_len {
            out.push('…');
            break;
        }
    }
    out.trim().to_string()
}
