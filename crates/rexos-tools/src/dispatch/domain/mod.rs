mod browser;
mod classify;
mod fs;
mod media;
mod process;
mod runtime;
#[cfg(test)]
mod tests;
mod web;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ToolCallDomain {
    Fs,
    Process,
    Web,
    Media,
    Browser,
    RuntimeCompat,
}

pub(crate) fn tool_call_domain(name: &str) -> Option<ToolCallDomain> {
    classify::tool_call_domain(name)
}
