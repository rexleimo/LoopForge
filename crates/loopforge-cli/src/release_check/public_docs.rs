use anyhow::Context;
use std::path::{Path, PathBuf};

const PUBLIC_DOCS_FORBIDDEN_TERMS: &[&str] = &["openfang", "openclaw"];

fn collect_public_text_files(root: &Path, out: &mut Vec<PathBuf>) -> anyhow::Result<()> {
    if !root.exists() {
        return Ok(());
    }
    for entry in std::fs::read_dir(root).with_context(|| format!("read_dir {}", root.display()))? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_public_text_files(&path, out)?;
            continue;
        }
        let ext = path
            .extension()
            .and_then(|value| value.to_str())
            .unwrap_or("");
        if matches!(ext, "md" | "markdown" | "yml" | "yaml") {
            out.push(path);
        }
    }
    Ok(())
}

pub(super) fn find_public_competitor_content(repo_root: &Path) -> anyhow::Result<Vec<String>> {
    let mut files = Vec::new();
    let mkdocs = repo_root.join("mkdocs.yml");
    if mkdocs.exists() {
        files.push(mkdocs);
    }
    collect_public_text_files(&repo_root.join("docs-site"), &mut files)?;

    let mut hits = Vec::new();
    for path in files {
        let raw =
            std::fs::read_to_string(&path).with_context(|| format!("read {}", path.display()))?;
        let lower = raw.to_ascii_lowercase();
        for term in PUBLIC_DOCS_FORBIDDEN_TERMS {
            if lower.contains(term) {
                let display = path.strip_prefix(repo_root).unwrap_or(&path).display();
                hits.push(format!("{} ({term})", display));
            }
        }
    }
    hits.sort();
    hits.dedup();
    Ok(hits)
}
