use anyhow::bail;

use super::super::super::directives::{add_file_path, delete_file_path, update_file_path};

pub(super) enum PatchDirective {
    Add(String),
    Update(String),
    Delete(String),
}

pub(super) fn parse_patch_directive(line: &str) -> anyhow::Result<PatchDirective> {
    if let Some(path) = add_file_path(line)? {
        return Ok(PatchDirective::Add(path));
    }
    if let Some(path) = update_file_path(line)? {
        return Ok(PatchDirective::Update(path));
    }
    if let Some(path) = delete_file_path(line)? {
        return Ok(PatchDirective::Delete(path));
    }
    bail!("unknown patch directive: {line}")
}
