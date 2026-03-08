use super::{LeakMatch, SensitiveEnvValue};

pub(super) fn collect_matches(
    content: &str,
    sensitive_env_values: &[SensitiveEnvValue],
) -> Vec<LeakMatch> {
    let mut out = Vec::new();
    out.extend(find_sensitive_env_matches(content, sensitive_env_values));
    out.extend(find_prefixed_token_matches(content, "sk-", 20, "token:sk"));
    out.extend(find_prefixed_token_matches(
        content,
        "ghp_",
        20,
        "token:github_pat",
    ));
    out.extend(find_prefixed_token_matches(
        content,
        "github_pat_",
        24,
        "token:github_pat",
    ));
    out.extend(find_prefixed_token_matches(
        content,
        "AIza",
        24,
        "token:google_api_key",
    ));
    merge_matches(out)
}

fn find_sensitive_env_matches(
    content: &str,
    sensitive_env_values: &[SensitiveEnvValue],
) -> Vec<LeakMatch> {
    let mut matches = Vec::new();
    for item in sensitive_env_values {
        matches.extend(find_exact_matches(
            content,
            &item.value,
            item.detector.clone(),
        ));
    }
    matches
}

fn find_exact_matches(content: &str, needle: &str, detector: String) -> Vec<LeakMatch> {
    content
        .match_indices(needle)
        .map(|(start, _)| LeakMatch {
            start,
            end: start + needle.len(),
            detector: detector.clone(),
        })
        .collect()
}

fn find_prefixed_token_matches(
    content: &str,
    prefix: &str,
    min_len: usize,
    detector: &str,
) -> Vec<LeakMatch> {
    let mut matches = Vec::new();
    let bytes = content.as_bytes();
    let prefix_bytes = prefix.as_bytes();
    let mut index = 0usize;

    while index + prefix_bytes.len() <= bytes.len() {
        if &bytes[index..index + prefix_bytes.len()] != prefix_bytes {
            index += 1;
            continue;
        }

        let mut end = index + prefix_bytes.len();
        while end < bytes.len() && is_token_byte(bytes[end]) {
            end += 1;
        }

        if end - index >= min_len {
            matches.push(LeakMatch {
                start: index,
                end,
                detector: detector.to_string(),
            });
        }
        index = end.max(index + 1);
    }

    matches
}

fn is_token_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_')
}

fn merge_matches(mut matches: Vec<LeakMatch>) -> Vec<LeakMatch> {
    matches.sort_by(|left, right| {
        left.start
            .cmp(&right.start)
            .then(right.end.cmp(&left.end))
            .then(left.detector.cmp(&right.detector))
    });

    let mut merged: Vec<LeakMatch> = Vec::new();
    for item in matches {
        if let Some(last) = merged.last() {
            if item.start < last.end {
                continue;
            }
        }
        merged.push(item);
    }
    merged
}
