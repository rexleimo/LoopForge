mod files;
mod ops;
#[cfg(test)]
mod tests;

use crate::patch::{PatchApplyResult, PatchOp};
use crate::Toolset;

pub(super) fn apply_patch_op(
    tools: &Toolset,
    op: PatchOp,
    result: &mut PatchApplyResult,
) -> anyhow::Result<()> {
    ops::apply_patch_op(tools, op, result)
}
