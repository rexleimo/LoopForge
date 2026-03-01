use std::path::Path;
use std::process::Command;

use anyhow::{bail, Context};

const FEATURES_JSON: &str = "features.json";
const PROGRESS_MD: &str = "rexos-progress.md";
const INIT_SH: &str = "init.sh";

pub fn init_workspace(workspace_dir: &Path) -> anyhow::Result<()> {
    std::fs::create_dir_all(workspace_dir)
        .with_context(|| format!("create workspace dir: {}", workspace_dir.display()))?;

    let features_path = workspace_dir.join(FEATURES_JSON);
    let progress_path = workspace_dir.join(PROGRESS_MD);
    let init_sh_path = workspace_dir.join(INIT_SH);

    if features_path.exists() || progress_path.exists() || init_sh_path.exists() {
        bail!("workspace already initialized");
    }

    let features_json = serde_json::json!({
        "version": 1,
        "updated_at": "",
        "rules": {
            "editing": "Only change `passes` (false -> true) and optionally `notes`. Do not delete or reorder items.",
            "completion": "A feature can only be marked passing after required tests/smoke checks are run."
        },
        "features": []
    });
    std::fs::write(&features_path, serde_json::to_string_pretty(&features_json)?)
        .with_context(|| format!("write {}", features_path.display()))?;

    std::fs::write(
        &progress_path,
        "# RexOS Progress Log\n\nThis file is append-only.\n",
    )
    .with_context(|| format!("write {}", progress_path.display()))?;

    std::fs::write(
        &init_sh_path,
        r#"#!/usr/bin/env bash
set -euo pipefail

echo "[rexos] init.sh: customize this script for your project"
"#,
    )
    .with_context(|| format!("write {}", init_sh_path.display()))?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&init_sh_path)?.permissions();
        perms.set_mode(perms.mode() | 0o111);
        std::fs::set_permissions(&init_sh_path, perms)?;
    }

    ensure_git_repo(workspace_dir)?;
    git(workspace_dir, ["add", FEATURES_JSON, PROGRESS_MD, INIT_SH])?;
    git_with_identity(
        workspace_dir,
        [
            "commit",
            "-m",
            "chore: initialize rexos harness",
            "--no-gpg-sign",
        ],
    )?;

    Ok(())
}

pub fn preflight(workspace_dir: &Path) -> anyhow::Result<()> {
    let features_path = workspace_dir.join(FEATURES_JSON);
    let progress_path = workspace_dir.join(PROGRESS_MD);
    let init_sh_path = workspace_dir.join(INIT_SH);

    if !features_path.exists() || !progress_path.exists() || !init_sh_path.exists() {
        bail!(
            "workspace not initialized; run `rexos harness init {}`",
            workspace_dir.display()
        );
    }

    println!("[rexos] workspace: {}", workspace_dir.display());

    let git_log = Command::new("git")
        .args(["--no-pager", "log", "-5", "--oneline"])
        .current_dir(workspace_dir)
        .output()
        .with_context(|| format!("run git log in {}", workspace_dir.display()))?;
    if git_log.status.success() {
        let s = String::from_utf8_lossy(&git_log.stdout);
        println!("[rexos] recent commits:\n{s}");
    } else {
        let e = String::from_utf8_lossy(&git_log.stderr);
        println!("[rexos] git log failed (continuing):\n{e}");
    }

    let progress = std::fs::read_to_string(&progress_path)
        .with_context(|| format!("read {}", progress_path.display()))?;
    println!(
        "[rexos] progress (tail):\n{}",
        tail_lines(&progress, 20).join("\n")
    );

    let features_raw = std::fs::read_to_string(&features_path)
        .with_context(|| format!("read {}", features_path.display()))?;
    if let Ok(v) = serde_json::from_str::<serde_json::Value>(&features_raw) {
        if let Some(first) = first_failing_feature(&v) {
            println!("[rexos] next feature: {first}");
        } else {
            println!("[rexos] next feature: (none pending)");
        }
    } else {
        println!("[rexos] features.json: could not parse (continuing)");
    }

    let status = Command::new("bash")
        .arg(&init_sh_path)
        .current_dir(workspace_dir)
        .status()
        .with_context(|| format!("run {}", init_sh_path.display()))?;
    if !status.success() {
        bail!("init.sh failed");
    }

    Ok(())
}

fn ensure_git_repo(workspace_dir: &Path) -> anyhow::Result<()> {
    if workspace_dir.join(".git").exists() {
        return Ok(());
    }

    if git(workspace_dir, ["init", "-b", "main"]).is_ok() {
        return Ok(());
    }

    git(workspace_dir, ["init"])?;
    git(workspace_dir, ["checkout", "-b", "main"])?;
    Ok(())
}

fn git<const N: usize>(workspace_dir: &Path, args: [&str; N]) -> anyhow::Result<()> {
    let output = Command::new("git")
        .args(args)
        .current_dir(workspace_dir)
        .output()
        .with_context(|| format!("run git {:?}", args))?;

    if !output.status.success() {
        bail!(
            "git failed: {:?}\nstdout:\n{}\nstderr:\n{}",
            args,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }
    Ok(())
}

fn git_with_identity<const N: usize>(
    workspace_dir: &Path,
    args: [&str; N],
) -> anyhow::Result<()> {
    let output = Command::new("git")
        .arg("-c")
        .arg("user.name=RexOS")
        .arg("-c")
        .arg("user.email=rexos@localhost")
        .args(args)
        .current_dir(workspace_dir)
        .output()
        .with_context(|| format!("run git (identity) {:?}", args))?;

    if !output.status.success() {
        bail!(
            "git failed: {:?}\nstdout:\n{}\nstderr:\n{}",
            args,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }
    Ok(())
}

fn tail_lines(s: &str, n: usize) -> Vec<&str> {
    let mut lines: Vec<&str> = s.lines().collect();
    if lines.len() > n {
        lines.drain(0..lines.len() - n);
    }
    lines
}

fn first_failing_feature(v: &serde_json::Value) -> Option<String> {
    let arr = v.get("features")?.as_array()?;
    for f in arr {
        if f.get("passes").and_then(|p| p.as_bool()) == Some(false) {
            let id = f.get("id").and_then(|x| x.as_str()).unwrap_or("<no id>");
            let desc = f
                .get("description")
                .and_then(|x| x.as_str())
                .unwrap_or("");
            return Some(format!("{id} - {desc}"));
        }
    }
    None
}
