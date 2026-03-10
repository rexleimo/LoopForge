mod browser;
mod catalog;
mod compat;
mod exports;
mod fs;
mod mcp;
mod media;
mod process;
mod web;

pub(crate) use catalog::{compat_tool_defs, core_tool_defs};
pub(crate) use exports::*;
pub(crate) use mcp::wrapper_tool_defs as mcp_wrapper_tool_defs;

#[cfg(test)]
mod tests {
    use super::{compat_tool_defs, core_tool_defs};

    #[test]
    fn core_tool_defs_include_multiple_domains() {
        let defs = core_tool_defs();
        let names: Vec<&str> = defs.iter().map(|def| def.function.name.as_str()).collect();
        assert!(names.contains(&"fs_read"), "{names:?}");
        assert!(names.contains(&"web_fetch"), "{names:?}");
        assert!(names.contains(&"browser_navigate"), "{names:?}");
    }

    #[test]
    fn compat_tool_defs_include_aliases() {
        let defs = compat_tool_defs();
        let names: Vec<&str> = defs.iter().map(|def| def.function.name.as_str()).collect();
        assert!(names.contains(&"file_read"), "{names:?}");
        assert!(names.contains(&"file_write"), "{names:?}");
        assert!(names.contains(&"workflow_run"), "{names:?}");
    }
}
