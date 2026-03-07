use anyhow::{bail, Context};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum PatchOp {
    AddFile { path: String, content: String },
    UpdateFile { path: String, hunks: Vec<PatchHunk> },
    DeleteFile { path: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PatchHunk {
    old_lines: Vec<String>,
    new_lines: Vec<String>,
}

#[derive(Debug, Default)]
pub(crate) struct PatchApplyResult {
    pub(crate) files_added: u32,
    pub(crate) files_updated: u32,
    pub(crate) files_deleted: u32,
}

impl PatchApplyResult {
    pub(crate) fn summary(&self) -> String {
        let mut parts = Vec::new();
        if self.files_added > 0 {
            parts.push(format!("{} added", self.files_added));
        }
        if self.files_updated > 0 {
            parts.push(format!("{} updated", self.files_updated));
        }
        if self.files_deleted > 0 {
            parts.push(format!("{} deleted", self.files_deleted));
        }
        if parts.is_empty() {
            "No changes applied".to_string()
        } else {
            parts.join(", ")
        }
    }
}

pub(crate) fn parse_patch(input: &str) -> anyhow::Result<Vec<PatchOp>> {
    let lines: Vec<&str> = input.lines().collect();
    let begin = lines
        .iter()
        .position(|l| l.trim() == "*** Begin Patch")
        .context("missing '*** Begin Patch' marker")?;
    let end = lines
        .iter()
        .rposition(|l| l.trim() == "*** End Patch")
        .context("missing '*** End Patch' marker")?;
    if end <= begin {
        bail!("'*** End Patch' must come after '*** Begin Patch'");
    }

    let body = &lines[begin + 1..end];
    let mut ops = Vec::new();
    let mut i = 0usize;

    while i < body.len() {
        let line = body[i].trim();
        if line.is_empty() {
            i += 1;
            continue;
        }

        if let Some(rest) = line.strip_prefix("*** Add File:") {
            let path = rest.trim().to_string();
            if path.is_empty() {
                bail!("empty path in '*** Add File:'");
            }
            i += 1;

            let mut content_lines = Vec::new();
            while i < body.len() && !body[i].trim().starts_with("***") {
                let raw = body[i];
                if let Some(stripped) = raw.strip_prefix('+') {
                    content_lines.push(stripped.to_string());
                } else if raw.trim().is_empty() {
                    content_lines.push(String::new());
                } else {
                    bail!("expected '+' prefix in Add File content, got: {}", raw);
                }
                i += 1;
            }

            ops.push(PatchOp::AddFile {
                path,
                content: content_lines.join("\n"),
            });
            continue;
        }

        if let Some(rest) = line.strip_prefix("*** Update File:") {
            let path = rest.trim().to_string();
            if path.is_empty() {
                bail!("empty path in '*** Update File:'");
            }
            i += 1;

            let mut hunks = Vec::new();
            while i < body.len() && !body[i].trim().starts_with("***") {
                let cur = body[i].trim();
                if cur.starts_with("@@") {
                    i += 1;
                    let mut old_lines = Vec::new();
                    let mut new_lines = Vec::new();
                    while i < body.len()
                        && !body[i].trim().starts_with("@@")
                        && !body[i].trim().starts_with("***")
                    {
                        let hl = body[i];
                        if let Some(stripped) = hl.strip_prefix('-') {
                            old_lines.push(stripped.to_string());
                        } else if let Some(stripped) = hl.strip_prefix('+') {
                            new_lines.push(stripped.to_string());
                        }
                        i += 1;
                    }
                    hunks.push(PatchHunk {
                        old_lines,
                        new_lines,
                    });
                } else {
                    i += 1;
                }
            }

            if hunks.is_empty() {
                bail!("Update File '{path}' has no hunks");
            }

            ops.push(PatchOp::UpdateFile { path, hunks });
            continue;
        }

        if let Some(rest) = line.strip_prefix("*** Delete File:") {
            let path = rest.trim().to_string();
            if path.is_empty() {
                bail!("empty path in '*** Delete File:'");
            }
            i += 1;
            ops.push(PatchOp::DeleteFile { path });
            continue;
        }

        bail!("unknown patch directive: {line}");
    }

    Ok(ops)
}

pub(crate) fn apply_hunks_to_text(before: &str, hunks: &[PatchHunk]) -> anyhow::Result<String> {
    let trailing_newline = before.ends_with('\n');
    let mut lines: Vec<String> = before.lines().map(|l| l.to_string()).collect();

    for hunk in hunks {
        if hunk.old_lines.is_empty() {
            lines.extend(hunk.new_lines.clone());
            continue;
        }

        let mut found = None;
        for idx in 0..=lines.len().saturating_sub(hunk.old_lines.len()) {
            if lines[idx..idx + hunk.old_lines.len()] == hunk.old_lines {
                found = Some(idx);
                break;
            }
        }

        let idx = found.context("hunk target not found in file")?;
        lines.splice(idx..idx + hunk.old_lines.len(), hunk.new_lines.clone());
    }

    let mut out = lines.join("\n");
    if trailing_newline {
        out.push('\n');
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_patch_handles_add_update_and_delete_ops() {
        let patch = r#"*** Begin Patch
*** Add File: greet.txt
+hi
*** Update File: greet.txt
@@
-hi
+hello
*** Delete File: old.txt
*** End Patch"#;

        let ops = parse_patch(patch).unwrap();
        assert_eq!(ops.len(), 3);
        assert!(
            matches!(&ops[0], PatchOp::AddFile { path, content } if path == "greet.txt" && content == "hi")
        );
        assert!(
            matches!(&ops[1], PatchOp::UpdateFile { path, hunks } if path == "greet.txt" && hunks.len() == 1)
        );
        assert!(matches!(&ops[2], PatchOp::DeleteFile { path } if path == "old.txt"));
    }

    #[test]
    fn apply_hunks_to_text_replaces_matching_block() {
        let before = "alpha
beta
gamma
";
        let hunks = vec![PatchHunk {
            old_lines: vec!["beta".to_string()],
            new_lines: vec!["delta".to_string()],
        }];

        let after = apply_hunks_to_text(before, &hunks).unwrap();
        assert_eq!(
            after,
            "alpha
delta
gamma
"
        );
    }
}
