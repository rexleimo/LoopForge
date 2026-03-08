use std::path::Path;
use std::process::Command;

use anyhow::{bail, Context};

pub(super) fn ensure_git_repo(workspace_dir: &Path) -> anyhow::Result<()> {
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

pub(super) fn git<const N: usize>(workspace_dir: &Path, args: [&str; N]) -> anyhow::Result<()> {
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

pub(super) fn git_with_identity<const N: usize>(
    workspace_dir: &Path,
    args: [&str; N],
) -> anyhow::Result<()> {
    let output = Command::new("git")
        .arg("-c")
        .arg("user.name=LoopForge")
        .arg("-c")
        .arg("user.email=loopforge@localhost")
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

pub(super) fn ensure_gitignore_has_loopforge_dir(workspace_dir: &Path) -> anyhow::Result<()> {
    if !workspace_dir.join(".git").exists() {
        return Ok(());
    }

    let path = workspace_dir.join(".gitignore");
    let line = ".loopforge/";

    let mut content = if path.exists() {
        std::fs::read_to_string(&path).with_context(|| format!("read {}", path.display()))?
    } else {
        String::new()
    };

    if content.lines().any(|l| l.trim() == line) {
        return Ok(());
    }

    if !content.is_empty() && !content.ends_with('\n') {
        content.push('\n');
    }
    content.push_str(line);
    content.push('\n');

    std::fs::write(&path, content).with_context(|| format!("write {}", path.display()))?;
    Ok(())
}

pub(super) fn commit_checkpoint_if_dirty(
    workspace_dir: &Path,
    message: &str,
) -> anyhow::Result<()> {
    let output = Command::new("git")
        .args(["status", "--porcelain"])
        .current_dir(workspace_dir)
        .output()
        .context("git status")?;

    if !output.status.success() {
        bail!(
            "git status failed:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    if output.stdout.is_empty() {
        return Ok(());
    }

    git(workspace_dir, ["add", "-A"])?;
    git_with_identity(workspace_dir, ["commit", "-m", message, "--no-gpg-sign"])?;
    Ok(())
}
