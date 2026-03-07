use crate::patch::PatchHunk;

pub(super) fn parse_hunk(body: &[&str], index: &mut usize) -> PatchHunk {
    let mut old_lines = Vec::new();
    let mut new_lines = Vec::new();

    while *index < body.len()
        && !body[*index].trim().starts_with("@@")
        && !body[*index].trim().starts_with("***")
    {
        let line = body[*index];
        if let Some(stripped) = line.strip_prefix('-') {
            old_lines.push(stripped.to_string());
        } else if let Some(stripped) = line.strip_prefix('+') {
            new_lines.push(stripped.to_string());
        }
        *index += 1;
    }

    PatchHunk {
        old_lines,
        new_lines,
    }
}
