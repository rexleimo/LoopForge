use std::ffi::OsString;

use super::config::{configured_remote_base, parse_and_validate_remote_base};

static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

struct EnvVarGuard {
    key: &'static str,
    previous: Option<OsString>,
}

impl EnvVarGuard {
    fn set(key: &'static str, value: impl AsRef<std::ffi::OsStr>) -> Self {
        let previous = std::env::var_os(key);
        std::env::set_var(key, value);
        Self { key, previous }
    }

    fn remove(key: &'static str) -> Self {
        let previous = std::env::var_os(key);
        std::env::remove_var(key);
        Self { key, previous }
    }
}

impl Drop for EnvVarGuard {
    fn drop(&mut self) {
        match &self.previous {
            Some(value) => std::env::set_var(self.key, value),
            None => std::env::remove_var(self.key),
        }
    }
}

#[test]
fn configured_remote_base_ignores_missing_or_blank_env() {
    let _lock = ENV_LOCK.lock().unwrap_or_else(|err| err.into_inner());
    let _guard = EnvVarGuard::remove("LOOPFORGE_BROWSER_CDP_HTTP");
    assert_eq!(configured_remote_base().unwrap(), None);

    let _guard = EnvVarGuard::set("LOOPFORGE_BROWSER_CDP_HTTP", "   ");
    assert_eq!(configured_remote_base().unwrap(), None);
}

#[test]
fn configured_remote_base_trims_and_validates_env() {
    let _lock = ENV_LOCK.lock().unwrap_or_else(|err| err.into_inner());
    let _guard = EnvVarGuard::set("LOOPFORGE_BROWSER_CDP_HTTP", " http://127.0.0.1:9222 ");
    assert_eq!(
        configured_remote_base().unwrap(),
        Some("http://127.0.0.1:9222".to_string())
    );
}

#[test]
fn parse_and_validate_remote_base_rejects_invalid_urls() {
    let err = parse_and_validate_remote_base("not a url").unwrap_err();
    assert!(
        err.to_string().contains("LOOPFORGE_BROWSER_CDP_HTTP"),
        "{err}"
    );
}
