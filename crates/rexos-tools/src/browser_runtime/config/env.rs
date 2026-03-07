pub(crate) fn sandbox_python_env(cmd: &mut tokio::process::Command) {
    cmd.env_clear();

    if let Ok(path) = std::env::var("PATH") {
        cmd.env("PATH", path);
    }

    cmd.env("PYTHONIOENCODING", "utf-8");

    if cfg!(windows) {
        for key in [
            "SystemRoot",
            "USERPROFILE",
            "TEMP",
            "TMP",
            "APPDATA",
            "LOCALAPPDATA",
        ] {
            if let Ok(v) = std::env::var(key) {
                cmd.env(key, v);
            }
        }
    } else {
        for key in ["HOME", "USER", "TMPDIR", "XDG_CACHE_HOME"] {
            if let Ok(v) = std::env::var(key) {
                cmd.env(key, v);
            }
        }
    }
}
