use super::{LeakGuard, LeakGuardVerdict};
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
        if let Some(value) = self.previous.take() {
            std::env::set_var(self.key, value);
        } else {
            std::env::remove_var(self.key);
        }
    }
}

fn leak_guard(mode: LeakMode) -> LeakGuard {
    let mut cfg = SecurityConfig::default();
    cfg.leaks.mode = mode;
    LeakGuard::from_security(&cfg)
}

#[test]
fn warn_mode_reports_detected_env_secret_without_mutating_output() {
    let _guard = EnvVarGuard::set(
        "LOOPFORGE_TEST_SECRET_WARN",
        "super-secret-warn-value-12345",
    );
    let verdict = leak_guard(LeakMode::Warn)
        .inspect_tool_output("value=super-secret-warn-value-12345".to_string());

    match verdict {
        LeakGuardVerdict::Allowed {
            content,
            audit: Some(audit),
        } => {
            assert_eq!(content, "value=super-secret-warn-value-12345");
            assert_eq!(audit.mode, "warn");
            assert!(audit
                .detectors
                .iter()
                .any(|d| d == "env:LOOPFORGE_TEST_SECRET_WARN"));
            assert!(!audit.redacted);
            assert!(!audit.blocked);
        }
        other => panic!("expected warn verdict, got: {other:?}"),
    }
}

#[test]
fn redact_mode_masks_detected_secret_before_returning_content() {
    let _guard = EnvVarGuard::set(
        "LOOPFORGE_TEST_SECRET_REDACT",
        "super-secret-redact-value-12345",
    );
    let verdict = leak_guard(LeakMode::Redact)
        .inspect_tool_output("prefix super-secret-redact-value-12345 suffix".to_string());

    match verdict {
        LeakGuardVerdict::Allowed {
            content,
            audit: Some(audit),
        } => {
            assert!(
                !content.contains("super-secret-redact-value-12345"),
                "{content}"
            );
            assert!(
                content.contains("[redacted:env:LOOPFORGE_TEST_SECRET_REDACT]"),
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
    let _guard = EnvVarGuard::set(
        "LOOPFORGE_TEST_SECRET_ENFORCE",
        "super-secret-enforce-value-12345",
    );
    let verdict = leak_guard(LeakMode::Enforce)
        .inspect_tool_output("prefix super-secret-enforce-value-12345 suffix".to_string());

    match verdict {
        LeakGuardVerdict::Blocked { error, audit } => {
            assert_eq!(error, "tool output blocked by leak guard");
            assert_eq!(audit.mode, "enforce");
            assert!(audit.blocked);
            assert!(audit
                .detectors
                .iter()
                .any(|d| d == "env:LOOPFORGE_TEST_SECRET_ENFORCE"));
        }
        other => panic!("expected enforce verdict, got: {other:?}"),
    }
}

#[test]
fn leak_guard_snapshots_env_values_at_construction() {
    let _guard = EnvVarGuard::set(
        "LOOPFORGE_TEST_SECRET_SNAPSHOT",
        "super-secret-snapshot-value-12345",
    );
    let leak_guard = leak_guard(LeakMode::Redact);
    std::env::remove_var("LOOPFORGE_TEST_SECRET_SNAPSHOT");

    let verdict = leak_guard
        .inspect_tool_output("prefix super-secret-snapshot-value-12345 suffix".to_string());

    match verdict {
        LeakGuardVerdict::Allowed { content, .. } => {
            assert!(content.contains("[redacted:env:LOOPFORGE_TEST_SECRET_SNAPSHOT]"));
        }
        other => panic!("expected snapshot redact verdict, got: {other:?}"),
    }
}

#[test]
fn redact_mode_masks_common_sk_prefixed_tokens() {
    let verdict = leak_guard(LeakMode::Redact)
        .inspect_tool_output("token=sk-test-abcdefghijklmnopqrstuvwxyz123456".to_string());

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
