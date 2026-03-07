use std::path::{Path, PathBuf};

use anyhow::{bail, Context};

pub(super) fn validate_pdf_file(path: &Path, user_path: &str) -> anyhow::Result<u64> {
    let meta = std::fs::metadata(path).with_context(|| format!("stat {}", path.display()))?;
    if meta.len() > 20 * 1024 * 1024 {
        bail!("pdf too large: {} bytes", meta.len());
    }
    ensure_pdf_extension(path, user_path)?;
    Ok(meta.len())
}

fn ensure_pdf_extension(path: &Path, user_path: &str) -> anyhow::Result<()> {
    let ext_ok = path
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.eq_ignore_ascii_case("pdf"))
        .unwrap_or(false);
    if !ext_ok {
        bail!("expected a .pdf file: {user_path}");
    }
    Ok(())
}

pub(super) async fn extract_pdf_pages(path: PathBuf) -> anyhow::Result<Vec<String>> {
    tokio::task::spawn_blocking(move || -> anyhow::Result<Vec<String>> {
        pdf_extract::extract_text_by_pages(&path)
            .map_err(|error| anyhow::anyhow!("pdf extract failed: {error}"))
    })
    .await
    .context("join pdf extract task")?
}
