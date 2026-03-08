mod features;
mod git;
mod prompts;
mod scripts;

#[cfg(test)]
mod tests;

use std::path::Path;
use std::process::Command;

use anyhow::{bail, Context};

use features::{
    ensure_features_populated, first_failing_feature, is_initialized, normalize_features_json,
    tail_lines,
};
use git::{
    commit_checkpoint_if_dirty, ensure_git_repo, ensure_gitignore_has_loopforge_dir, git,
    git_with_identity,
};
use prompts::{coding_system_prompt, initializer_system_prompt};
use scripts::{run_init_script, run_init_script_capture};

const FEATURES_JSON: &str = "features.json";
const PROGRESS_MD: &str = "loopforge-progress.md";
const INIT_SH: &str = "init.sh";
const INIT_PS1: &str = "init.ps1";
const LOOPFORGE_DIR: &str = ".loopforge";
const SESSION_ID_FILE: &str = "session_id";

pub fn init_workspace(workspace_dir: &Path) -> anyhow::Result<()> {
    std::fs::create_dir_all(workspace_dir)
        .with_context(|| format!("create workspace dir: {}", workspace_dir.display()))?;

    let features_path = workspace_dir.join(FEATURES_JSON);
    let progress_path = workspace_dir.join(PROGRESS_MD);
    let init_sh_path = workspace_dir.join(INIT_SH);
    let init_ps1_path = workspace_dir.join(INIT_PS1);

    if features_path.exists()
        || progress_path.exists()
        || init_sh_path.exists()
        || init_ps1_path.exists()
    {
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
    std::fs::write(
        &features_path,
        serde_json::to_string_pretty(&features_json)?,
    )
    .with_context(|| format!("write {}", features_path.display()))?;

    std::fs::write(
        &progress_path,
        "# LoopForge Progress Log\n\nThis file is append-only.\n",
    )
    .with_context(|| format!("write {}", progress_path.display()))?;

    std::fs::write(
        &init_sh_path,
        r#"#!/usr/bin/env bash
set -euo pipefail

echo "[loopforge] init.sh: customize this script for your project"
"#,
    )
    .with_context(|| format!("write {}", init_sh_path.display()))?;

    std::fs::write(
        &init_ps1_path,
        r#"$ErrorActionPreference = "Stop"

Write-Output "[loopforge] init.ps1: customize this script for your project"
"#,
    )
    .with_context(|| format!("write {}", init_ps1_path.display()))?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&init_sh_path)?.permissions();
        perms.set_mode(perms.mode() | 0o111);
        std::fs::set_permissions(&init_sh_path, perms)?;
    }

    ensure_git_repo(workspace_dir)?;
    git(
        workspace_dir,
        ["add", FEATURES_JSON, PROGRESS_MD, INIT_SH, INIT_PS1],
    )?;
    git_with_identity(
        workspace_dir,
        [
            "commit",
            "-m",
            "chore: initialize loopforge harness",
            "--no-gpg-sign",
        ],
    )?;

    Ok(())
}

pub fn resolve_session_id(workspace_dir: &Path) -> anyhow::Result<String> {
    let loopforge_dir = workspace_dir.join(LOOPFORGE_DIR);
    std::fs::create_dir_all(&loopforge_dir)
        .with_context(|| format!("create {}", loopforge_dir.display()))?;

    ensure_gitignore_has_loopforge_dir(workspace_dir)?;

    let path = loopforge_dir.join(SESSION_ID_FILE);
    if path.exists() {
        let raw =
            std::fs::read_to_string(&path).with_context(|| format!("read {}", path.display()))?;
        let s = raw.trim().to_string();
        if s.is_empty() {
            bail!("session_id file is empty");
        }
        return Ok(s);
    }

    let id = uuid::Uuid::new_v4().to_string();
    std::fs::write(&path, format!("{id}\n"))
        .with_context(|| format!("write {}", path.display()))?;
    Ok(id)
}

pub async fn bootstrap_with_prompt(
    agent: &rexos_runtime::AgentRuntime,
    workspace_dir: &Path,
    session_id: &str,
    prompt: &str,
) -> anyhow::Result<()> {
    if !is_initialized(workspace_dir) {
        init_workspace(workspace_dir)?;
    }

    preflight(workspace_dir)?;

    let initializer_system = initializer_system_prompt();
    let _ = agent
        .run_session(
            workspace_dir.to_path_buf(),
            session_id,
            Some(initializer_system),
            prompt,
            rexos_kernel::router::TaskKind::Coding,
        )
        .await?;

    normalize_features_json(workspace_dir)?;
    ensure_features_populated(workspace_dir)?;

    run_init_script(workspace_dir)?;
    commit_checkpoint_if_dirty(workspace_dir, "chore: loopforge harness bootstrap")?;
    Ok(())
}

pub async fn run_harness(
    agent: &rexos_runtime::AgentRuntime,
    workspace_dir: &Path,
    session_id: &str,
    user_prompt: &str,
    max_attempts: usize,
) -> anyhow::Result<String> {
    preflight(workspace_dir)?;

    let harness_system = coding_system_prompt();
    let mut prompt = user_prompt.to_string();

    for attempt in 1..=max_attempts.max(1) {
        let out = agent
            .run_session(
                workspace_dir.to_path_buf(),
                session_id,
                Some(harness_system),
                &prompt,
                rexos_kernel::router::TaskKind::Coding,
            )
            .await?;

        match run_init_script_capture(workspace_dir) {
            Ok(_) => {
                commit_checkpoint_if_dirty(workspace_dir, "chore: loopforge harness checkpoint")?;
                return Ok(out);
            }
            Err(e) => {
                if attempt >= max_attempts.max(1) {
                    return Err(e);
                }
                prompt = format!(
                    "Workspace init script failed after your changes.\n\nOutput:\n{}\n\nFix the issues and make the init script pass (`./init.sh`, or `./init.ps1` on Windows).",
                    e
                );
            }
        }
    }

    bail!("unreachable: harness loop exhausted")
}

pub fn preflight(workspace_dir: &Path) -> anyhow::Result<()> {
    let features_path = workspace_dir.join(FEATURES_JSON);
    let progress_path = workspace_dir.join(PROGRESS_MD);
    let init_sh_path = workspace_dir.join(INIT_SH);
    let init_ps1_path = workspace_dir.join(INIT_PS1);

    if !features_path.exists()
        || !progress_path.exists()
        || (!init_sh_path.exists() && !init_ps1_path.exists())
    {
        bail!(
            "workspace not initialized; run `loopforge harness init {}`",
            workspace_dir.display()
        );
    }

    println!("[loopforge] workspace: {}", workspace_dir.display());

    let git_log = Command::new("git")
        .args(["--no-pager", "log", "-5", "--oneline"])
        .current_dir(workspace_dir)
        .output()
        .with_context(|| format!("run git log in {}", workspace_dir.display()))?;
    if git_log.status.success() {
        let s = String::from_utf8_lossy(&git_log.stdout);
        println!("[loopforge] recent commits:\n{s}");
    } else {
        let e = String::from_utf8_lossy(&git_log.stderr);
        println!("[loopforge] git log failed (continuing):\n{e}");
    }

    let progress = std::fs::read_to_string(&progress_path)
        .with_context(|| format!("read {}", progress_path.display()))?;
    println!(
        "[loopforge] progress (tail):\n{}",
        tail_lines(&progress, 20).join("\n")
    );

    let features_raw = std::fs::read_to_string(&features_path)
        .with_context(|| format!("read {}", features_path.display()))?;
    if let Ok(v) = serde_json::from_str::<serde_json::Value>(&features_raw) {
        if let Some(first) = first_failing_feature(&v) {
            println!("[loopforge] next feature: {first}");
        } else {
            println!("[loopforge] next feature: (none pending)");
        }
    } else {
        println!("[loopforge] features.json: could not parse (continuing)");
    }

    run_init_script(workspace_dir)?;

    Ok(())
}
