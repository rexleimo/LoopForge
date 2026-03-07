use std::path::Path;

mod exec;
mod managed;

fn configure_command_env(cmd: &mut tokio::process::Command, workspace_root: &Path) {
    cmd.current_dir(workspace_root).env_clear();

    if let Ok(path) = std::env::var("PATH") {
        cmd.env("PATH", path);
    }

    if cfg!(windows) {
        for key in ["SystemRoot", "USERPROFILE", "TEMP", "TMP"] {
            if let Ok(v) = std::env::var(key) {
                cmd.env(key, v);
            }
        }
    } else {
        for key in ["HOME", "USER"] {
            if let Ok(v) = std::env::var(key) {
                cmd.env(key, v);
            }
        }
    }

    for key in ["CARGO_HOME", "RUSTUP_HOME"] {
        if let Ok(v) = std::env::var(key) {
            cmd.env(key, v);
        }
    }
}
