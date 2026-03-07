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
fn browser_python_exe_honors_nonempty_env_value() {
    let _lock = ENV_LOCK.lock().unwrap_or_else(|err| err.into_inner());
    let _guard = EnvVarGuard::set("LOOPFORGE_BROWSER_PYTHON", "uv-python");
    assert_eq!(super::browser_python_exe(), "uv-python");
}

#[test]
fn env_bridge_script_path_rejects_missing_file() {
    let _lock = ENV_LOCK.lock().unwrap_or_else(|err| err.into_inner());
    let _guard = EnvVarGuard::set(
        "LOOPFORGE_BROWSER_BRIDGE_PATH",
        "/tmp/definitely-missing-bridge.py",
    );
    let err = super::script::env_bridge_script_path().unwrap_err();
    assert!(
        err.to_string().contains("LOOPFORGE_BROWSER_BRIDGE_PATH"),
        "{err}"
    );
}

#[test]
fn browser_python_exe_falls_back_when_env_blank() {
    let _lock = ENV_LOCK.lock().unwrap_or_else(|err| err.into_inner());
    let _guard = EnvVarGuard::set("LOOPFORGE_BROWSER_PYTHON", "   ");
    let exe = super::browser_python_exe();
    if cfg!(windows) {
        assert_eq!(exe, "python");
    } else {
        assert_eq!(exe, "python3");
    }
}
