use std::process::Command;

#[test]
fn harness_init_creates_artifacts_and_git_commit() {
    let tmp = tempfile::tempdir().unwrap();
    let workspace = tmp.path();

    rexos::harness::init_workspace(workspace).unwrap();

    assert!(workspace.join("features.json").exists());
    assert!(workspace.join("rexos-progress.md").exists());
    assert!(workspace.join("init.sh").exists());
    assert!(workspace.join(".git").exists());

    let commit_count = Command::new("git")
        .arg("-C")
        .arg(workspace)
        .args(["rev-list", "--count", "HEAD"])
        .output()
        .unwrap();
    assert!(commit_count.status.success());
    assert_eq!(String::from_utf8_lossy(&commit_count.stdout).trim(), "1");

    let subject = Command::new("git")
        .arg("-C")
        .arg(workspace)
        .args(["log", "-1", "--pretty=%s"])
        .output()
        .unwrap();
    assert!(subject.status.success());
    assert_eq!(
        String::from_utf8_lossy(&subject.stdout).trim(),
        "chore: initialize rexos harness"
    );

    let err = rexos::harness::init_workspace(workspace).unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("already initialized"), "{msg}");
}

