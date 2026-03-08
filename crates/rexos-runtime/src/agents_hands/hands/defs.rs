use crate::records::HandDef;
use crate::AgentRuntime;

impl AgentRuntime {
    pub(super) fn hand_defs() -> Vec<HandDef> {
        vec![
            HandDef {
                id: "browser",
                name: "Browser",
                description: "A focused web-browsing helper (use browser_* tools).",
                system_prompt: "You are a focused browser assistant. Use browser_* tools to navigate, read pages, and summarize findings clearly. Be careful with SSRF protections and only browse relevant URLs.",
            },
            HandDef {
                id: "coder",
                name: "Coder",
                description: "A focused coding helper (use fs_* and shell).",
                system_prompt: "You are a focused coding assistant. Use fs_read/fs_write/apply_patch and shell to implement changes safely. Prefer small commits, run tests, and explain how to reproduce.",
            },
            HandDef {
                id: "researcher",
                name: "Researcher",
                description: "A focused research helper (use web_search/web_fetch).",
                system_prompt: "You are a focused research assistant. Use web_search and web_fetch to gather information, then summarize with clear attribution. Avoid speculation and keep outputs concise.",
            },
        ]
    }
}
