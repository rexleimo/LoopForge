use std::process::Stdio;

use anyhow::Context;

use crate::browser_runtime::config::{
    browser_bridge_script_path, browser_python_exe, sandbox_python_env,
};

pub(super) fn bridge_headless_arg(headless: bool) -> &'static str {
    if headless {
        "--headless"
    } else {
        "--no-headless"
    }
}

pub(super) fn bridge_viewport_args() -> [&'static str; 6] {
    ["--width", "1280", "--height", "720", "--timeout", "30"]
}

pub(super) fn spawn_bridge_process(headless: bool) -> anyhow::Result<tokio::process::Child> {
    let python = browser_python_exe();
    let script_path = browser_bridge_script_path()?;

    let mut cmd = tokio::process::Command::new(python);
    cmd.arg("-u")
        .arg(script_path)
        .arg(bridge_headless_arg(headless));
    cmd.args(bridge_viewport_args());
    cmd.stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null());
    sandbox_python_env(&mut cmd);

    cmd.spawn().context("spawn browser bridge")
}
