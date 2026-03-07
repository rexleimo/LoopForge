use super::chromium_args;

static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

struct EnvVarGuard {
    key: &'static str,
    previous: Option<std::ffi::OsString>,
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
fn chromium_args_include_headless_and_optional_no_sandbox_flags() {
    let _lock = ENV_LOCK.lock().unwrap_or_else(|err| err.into_inner());
    let _guard = EnvVarGuard::set("LOOPFORGE_BROWSER_NO_SANDBOX", "true");
    let dir = tempfile::tempdir().unwrap();
    let args = chromium_args(9222, dir.path(), true);

    assert!(args.iter().any(|arg| arg == "--headless=new"));
    assert!(args.iter().any(|arg| arg == "--disable-gpu"));
    assert!(args.iter().any(|arg| arg == "--no-sandbox"));
    assert!(args.iter().any(|arg| arg == "--disable-setuid-sandbox"));
}

#[test]
fn chromium_args_keep_base_flags_when_headed_and_sandboxed() {
    let _lock = ENV_LOCK.lock().unwrap_or_else(|err| err.into_inner());
    let _guard = EnvVarGuard::remove("LOOPFORGE_BROWSER_NO_SANDBOX");
    let dir = tempfile::tempdir().unwrap();
    let args = chromium_args(9223, dir.path(), false);

    assert!(!args.iter().any(|arg| arg == "--headless=new"));
    assert!(!args.iter().any(|arg| arg == "--no-sandbox"));
    assert!(args
        .iter()
        .any(|arg| arg == "--remote-debugging-address=127.0.0.1"));
    assert!(args.iter().any(|arg| arg == "about:blank"));
}
