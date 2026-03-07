use std::process::Stdio;

pub(super) fn chromium_command(
    chrome_path: &std::path::Path,
    args: &[String],
) -> tokio::process::Command {
    let mut cmd = tokio::process::Command::new(chrome_path);
    cmd.args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .env_clear();

    for key in super::allow::allowed_chromium_env_keys() {
        if let Ok(value) = std::env::var(key) {
            cmd.env(key, value);
        }
    }

    cmd
}
