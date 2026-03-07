mod apply;
mod parse;
mod types;

pub(crate) use apply::apply_hunks_to_text;
pub(crate) use parse::parse_patch;
pub(crate) use types::{PatchApplyResult, PatchHunk, PatchOp};

#[cfg(test)]
mod tests;
