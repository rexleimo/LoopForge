use rexos_skills::loader::SkillSource;

use super::{permission_tools, source_name};

#[test]
fn permission_tools_expands_readonly_and_dedupes() {
    let tools = permission_tools(&[
        "readonly".to_string(),
        "tool:fs_read".to_string(),
        "tool:browser_read_page".to_string(),
        "tool:browser_read_page".to_string(),
        "  ".to_string(),
    ]);

    assert_eq!(
        tools,
        vec![
            "fs_read".to_string(),
            "fs_list".to_string(),
            "web_search".to_string(),
            "web_fetch".to_string(),
            "browser_read_page".to_string(),
        ]
    );
}

#[test]
fn source_name_maps_known_sources() {
    assert_eq!(source_name(SkillSource::Home), "home");
    assert_eq!(source_name(SkillSource::Workspace), "workspace");
}
