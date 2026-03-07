use anyhow::Context;

use super::PatchHunk;

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
