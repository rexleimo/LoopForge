use super::compat_tool_defs;

#[test]
fn memory_tool_defs_keep_expected_names_and_required_fields() {
    let defs = compat_tool_defs();
    let names: Vec<&str> = defs.iter().map(|def| def.function.name.as_str()).collect();

    assert_eq!(names, ["memory_store", "memory_recall"]);
    assert_eq!(
        defs[0].function.parameters["required"],
        serde_json::json!(["key", "value"])
    );
    assert_eq!(
        defs[1].function.parameters["required"],
        serde_json::json!(["key"])
    );
}
