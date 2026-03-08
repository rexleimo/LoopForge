use rexos_kernel::security::LeakMode;

use super::detect::LeakMatch;

pub(super) fn redact_content(content: &str, matches: &[LeakMatch]) -> String {
    let mut redacted = String::with_capacity(content.len());
    let mut cursor = 0usize;

    for item in matches {
        if item.start > cursor {
            redacted.push_str(&content[cursor..item.start]);
        }
        redacted.push_str("[redacted:");
        redacted.push_str(&item.detector);
        redacted.push(']');
        cursor = item.end;
    }

    if cursor < content.len() {
        redacted.push_str(&content[cursor..]);
    }

    redacted
}

pub(super) fn mode_label(mode: &LeakMode) -> &'static str {
    match mode {
        LeakMode::Off => "off",
        LeakMode::Warn => "warn",
        LeakMode::Redact => "redact",
        LeakMode::Enforce => "enforce",
    }
}
