use std::collections::BTreeSet;

use rexos_kernel::security::{LeakMode, SecurityConfig};

const BLOCKED_ERROR: &str = "tool output blocked by leak guard";

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, Default)]
pub(crate) struct LeakGuardAudit {
    pub(crate) mode: String,
    #[serde(default)]
    pub(crate) detectors: Vec<String>,
    #[serde(default)]
    pub(crate) redacted: bool,
    #[serde(default)]
    pub(crate) blocked: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum LeakGuardVerdict {
    Allowed {
        content: String,
        audit: Option<LeakGuardAudit>,
    },
    Blocked {
        error: String,
        audit: LeakGuardAudit,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct LeakMatch {
    start: usize,
    end: usize,
    detector: String,
}

pub(crate) fn inspect_tool_output(content: String, security: &SecurityConfig) -> LeakGuardVerdict {
    if matches!(security.leaks.mode, LeakMode::Off) {
        return LeakGuardVerdict::Allowed {
            content,
            audit: None,
        };
    }

    let matches = collect_matches(&content);
    if matches.is_empty() {
        return LeakGuardVerdict::Allowed {
            content,
            audit: None,
        };
    }

    let detectors = detector_labels(&matches);
    let mode = mode_label(&security.leaks.mode).to_string();

    match security.leaks.mode {
        LeakMode::Off => LeakGuardVerdict::Allowed {
            content,
            audit: None,
        },
        LeakMode::Warn => LeakGuardVerdict::Allowed {
            content,
            audit: Some(LeakGuardAudit {
                mode,
                detectors,
                redacted: false,
                blocked: false,
            }),
        },
        LeakMode::Redact => LeakGuardVerdict::Allowed {
            content: redact_content(&content, &matches),
            audit: Some(LeakGuardAudit {
                mode,
                detectors,
                redacted: true,
                blocked: false,
            }),
        },
        LeakMode::Enforce => LeakGuardVerdict::Blocked {
            error: BLOCKED_ERROR.to_string(),
            audit: LeakGuardAudit {
                mode,
                detectors,
                redacted: false,
                blocked: true,
            },
        },
    }
}

fn detector_labels(matches: &[LeakMatch]) -> Vec<String> {
    let mut out = BTreeSet::new();
    for item in matches {
        out.insert(item.detector.clone());
    }
    out.into_iter().collect()
}

fn collect_matches(content: &str) -> Vec<LeakMatch> {
    let mut out = Vec::new();
    out.extend(find_sensitive_env_matches(content));
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

fn find_sensitive_env_matches(content: &str) -> Vec<LeakMatch> {
    let mut matches = Vec::new();
    let mut seen_values = BTreeSet::new();

    for (name, value) in std::env::vars() {
        if !is_sensitive_env_name(&name) {
            continue;
        }
        if value.trim().len() < 8 || value.contains(char::is_whitespace) {
            continue;
        }
        if !seen_values.insert(value.clone()) {
            continue;
        }

        matches.extend(find_exact_matches(content, &value, format!("env:{name}")));
    }

    matches
}

fn is_sensitive_env_name(name: &str) -> bool {
    let upper = name.to_ascii_uppercase();
    if matches!(upper.as_str(), "PATH" | "PWD" | "HOME" | "SHELL" | "TERM") {
        return false;
    }

    upper.contains("TOKEN")
        || upper.contains("SECRET")
        || upper.contains("PASSWORD")
        || upper.contains("BEARER")
        || upper.ends_with("_KEY")
        || upper.contains("_KEY_")
        || upper.contains("API_KEY")
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

fn redact_content(content: &str, matches: &[LeakMatch]) -> String {
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

fn mode_label(mode: &LeakMode) -> &'static str {
    match mode {
        LeakMode::Off => "off",
        LeakMode::Warn => "warn",
        LeakMode::Redact => "redact",
        LeakMode::Enforce => "enforce",
    }
}

#[cfg(test)]
mod tests {
    use super::{inspect_tool_output, LeakGuardVerdict};
    use rexos_kernel::security::{LeakMode, SecurityConfig};

    struct EnvVarGuard {
        key: &'static str,
        previous: Option<String>,
    }

    impl EnvVarGuard {
        fn set(key: &'static str, value: &str) -> Self {
            let previous = std::env::var(key).ok();
            std::env::set_var(key, value);
            Self { key, previous }
        }
    }

    impl Drop for EnvVarGuard {
        fn drop(&mut self) {
            if let Some(v) = self.previous.take() {
                std::env::set_var(self.key, v);
            } else {
                std::env::remove_var(self.key);
            }
        }
    }

    fn security(mode: LeakMode) -> SecurityConfig {
        let mut cfg = SecurityConfig::default();
        cfg.leaks.mode = mode;
        cfg
    }

    #[test]
    fn warn_mode_reports_detected_env_secret_without_mutating_output() {
        let _guard = EnvVarGuard::set("LOOPFORGE_TEST_SECRET", "super-secret-value-12345");
        let verdict = inspect_tool_output(
            "value=super-secret-value-12345".to_string(),
            &security(LeakMode::Warn),
        );

        match verdict {
            LeakGuardVerdict::Allowed {
                content,
                audit: Some(audit),
            } => {
                assert_eq!(content, "value=super-secret-value-12345");
                assert_eq!(audit.mode, "warn");
                assert!(audit
                    .detectors
                    .iter()
                    .any(|d| d == "env:LOOPFORGE_TEST_SECRET"));
                assert!(!audit.redacted);
                assert!(!audit.blocked);
            }
            other => panic!("expected warn verdict, got: {other:?}"),
        }
    }

    #[test]
    fn redact_mode_masks_detected_secret_before_returning_content() {
        let _guard = EnvVarGuard::set("LOOPFORGE_TEST_SECRET", "super-secret-value-12345");
        let verdict = inspect_tool_output(
            "prefix super-secret-value-12345 suffix".to_string(),
            &security(LeakMode::Redact),
        );

        match verdict {
            LeakGuardVerdict::Allowed {
                content,
                audit: Some(audit),
            } => {
                assert!(!content.contains("super-secret-value-12345"), "{content}");
                assert!(
                    content.contains("[redacted:env:LOOPFORGE_TEST_SECRET]"),
                    "{content}"
                );
                assert_eq!(audit.mode, "redact");
                assert!(audit.redacted);
            }
            other => panic!("expected redact verdict, got: {other:?}"),
        }
    }

    #[test]
    fn enforce_mode_blocks_detected_secret_with_stable_error() {
        let _guard = EnvVarGuard::set("LOOPFORGE_TEST_SECRET", "super-secret-value-12345");
        let verdict = inspect_tool_output(
            "prefix super-secret-value-12345 suffix".to_string(),
            &security(LeakMode::Enforce),
        );

        match verdict {
            LeakGuardVerdict::Blocked { error, audit } => {
                assert_eq!(error, "tool output blocked by leak guard");
                assert_eq!(audit.mode, "enforce");
                assert!(audit.blocked);
                assert!(audit
                    .detectors
                    .iter()
                    .any(|d| d == "env:LOOPFORGE_TEST_SECRET"));
            }
            other => panic!("expected enforce verdict, got: {other:?}"),
        }
    }

    #[test]
    fn redact_mode_masks_common_sk_prefixed_tokens() {
        let verdict = inspect_tool_output(
            "token=sk-test-abcdefghijklmnopqrstuvwxyz123456".to_string(),
            &security(LeakMode::Redact),
        );

        match verdict {
            LeakGuardVerdict::Allowed {
                content,
                audit: Some(audit),
            } => {
                assert!(!content.contains("sk-test-abcdefghijklmnopqrstuvwxyz123456"));
                assert!(content.contains("[redacted:token:sk]"), "{content}");
                assert!(audit.detectors.iter().any(|d| d == "token:sk"));
            }
            other => panic!("expected sk redact verdict, got: {other:?}"),
        }
    }
}
