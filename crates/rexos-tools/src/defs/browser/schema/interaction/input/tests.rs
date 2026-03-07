use super::{
    browser_click_schema_def, browser_press_key_schema_def, browser_type_schema_def, tool_defs,
};

#[test]
fn input_tool_defs_match_individual_schema_defs() {
    let defs = tool_defs();
    let names: Vec<&str> = defs.iter().map(|def| def.function.name.as_str()).collect();

    assert_eq!(
        names,
        ["browser_click", "browser_type", "browser_press_key"]
    );
    assert_eq!(
        defs[0].function.name,
        browser_click_schema_def().function.name
    );
    assert_eq!(
        defs[1].function.name,
        browser_type_schema_def().function.name
    );
    assert_eq!(
        defs[2].function.name,
        browser_press_key_schema_def().function.name
    );
}
