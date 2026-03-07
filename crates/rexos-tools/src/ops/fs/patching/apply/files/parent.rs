use anyhow::Context;

pub(super) fn ensure_parent_dir(dest: &std::path::Path) -> anyhow::Result<()> {
    if let Some(parent) = dest.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("create dirs {}", parent.display()))?;
    }
    Ok(())
}
