mod apply;
#[cfg(test)]
mod tests;

use anyhow::Context;

use crate::patch::{parse_patch, PatchApplyResult};
use crate::Toolset;

impl Toolset {
    pub(crate) fn apply_patch(&self, patch: &str) -> anyhow::Result<String> {
        let ops = parse_patch(patch).context("parse patch")?;
        let mut result = PatchApplyResult::default();

        for op in ops {
            apply::apply_patch_op(self, op, &mut result)?;
        }

        Ok(result.summary())
    }
}
