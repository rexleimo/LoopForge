use std::path::Path;
use std::process::{Command, Output};

use anyhow::{bail, Context};

use crate::{INIT_PS1, INIT_SH};

#[derive(Debug, Clone, Copy)]
enum InitScript {
    Bash,
    PowerShell,
}

fn select_init_script(workspace_dir: &Path) -> anyhow::Result<InitScript> {
    let sh_exists = workspace_dir.join(INIT_SH).exists();
    let ps1_exists = workspace_dir.join(INIT_PS1).exists();

    if cfg!(windows) {
        if ps1_exists {
            return Ok(InitScript::PowerShell);
        }
        if sh_exists {
            bail!("init.ps1 is required on Windows (bash/WSL is not assumed to be available)");
        }
    } else if sh_exists {
        return Ok(InitScript::Bash);
    }

    if ps1_exists && !sh_exists {
        bail!("init.ps1 exists but init.sh is missing");
    }

    bail!("no init script found (expected init.sh and/or init.ps1)")
}

pub(super) fn run_init_script(workspace_dir: &Path) -> anyhow::Result<()> {
    match select_init_script(workspace_dir)? {
        InitScript::Bash => {
            let output = Command::new("bash")
                .arg(INIT_SH)
                .current_dir(workspace_dir)
                .output()
                .with_context(|| format!("run {}", workspace_dir.join(INIT_SH).display()))?;

            if output.status.success() {
                return Ok(());
            }

            let combined = format!(
                "{}{}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );

            if cfg!(windows) && workspace_dir.join(INIT_PS1).exists() {
                if let Ok(ps1) = run_powershell_script_output(workspace_dir, INIT_PS1) {
                    if ps1.status.success() {
                        return Ok(());
                    }
                }
            }

            bail!("init.sh failed: {}", combined.trim());
        }
        InitScript::PowerShell => {
            let output = run_powershell_script_output(workspace_dir, INIT_PS1)?;
            let mut combined = String::new();
            combined.push_str(&String::from_utf8_lossy(&output.stdout));
            combined.push_str(&String::from_utf8_lossy(&output.stderr));
            if !output.status.success() {
                bail!("init.ps1 failed: {}", combined.trim());
            }
            Ok(())
        }
    }
}

pub(super) fn run_init_script_capture(workspace_dir: &Path) -> anyhow::Result<String> {
    let output = match select_init_script(workspace_dir)? {
        InitScript::Bash => Command::new("bash")
            .arg(INIT_SH)
            .current_dir(workspace_dir)
            .output()
            .with_context(|| format!("run {}", workspace_dir.join(INIT_SH).display()))?,
        InitScript::PowerShell => run_powershell_script_output(workspace_dir, INIT_PS1)?,
    };

    let mut combined = String::new();
    combined.push_str(&String::from_utf8_lossy(&output.stdout));
    combined.push_str(&String::from_utf8_lossy(&output.stderr));

    if !output.status.success() {
        bail!("init failed: {}", combined.trim());
    }

    Ok(combined)
}

fn run_powershell_script_output(workspace_dir: &Path, script: &str) -> anyhow::Result<Output> {
    let args = [
        "-NoProfile",
        "-NonInteractive",
        "-ExecutionPolicy",
        "Bypass",
        "-File",
        script,
    ];

    if let Ok(output) = Command::new("powershell")
        .args(args)
        .current_dir(workspace_dir)
        .output()
    {
        return Ok(output);
    }

    Command::new("pwsh")
        .args(args)
        .current_dir(workspace_dir)
        .output()
        .with_context(|| format!("run {}", workspace_dir.join(script).display()))
}
