use super::result::normalize_wait_arg;

pub(super) fn normalized_wait_args<'a>(
    selector: Option<&'a str>,
    text: Option<&'a str>,
) -> (Option<&'a str>, Option<&'a str>) {
    (normalize_wait_arg(selector), normalize_wait_arg(text))
}

pub(super) fn clamped_timeout_ms(timeout_ms: Option<u64>) -> u64 {
    timeout_ms.unwrap_or(30_000).min(30_000)
}
