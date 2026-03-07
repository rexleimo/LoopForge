mod add;
mod directive;
mod parse;
#[cfg(test)]
mod tests;
mod update;

pub(crate) fn parse_patch(input: &str) -> anyhow::Result<Vec<crate::patch::PatchOp>> {
    parse::parse_patch(input)
}
