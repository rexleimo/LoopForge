use anyhow::bail;

pub(super) fn ensure_docker_exec_enabled() -> anyhow::Result<()> {
    let enabled = std::env::var("LOOPFORGE_DOCKER_EXEC_ENABLED")
        .ok()
        .map(|value| value.trim() == "1")
        .unwrap_or(false);
    if !enabled {
        bail!("docker_exec is disabled (set LOOPFORGE_DOCKER_EXEC_ENABLED=1 to enable)");
    }
    Ok(())
}

pub(super) fn docker_exec_image() -> String {
    std::env::var("LOOPFORGE_DOCKER_EXEC_IMAGE")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| "alpine:3.20".to_string())
}
