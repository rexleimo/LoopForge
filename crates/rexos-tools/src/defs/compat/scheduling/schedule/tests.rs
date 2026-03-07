use super::{
    schedule_create_schema_def, schedule_delete_schema_def, schedule_list_schema_def, tool_defs,
};

#[test]
fn schedule_tool_defs_match_individual_schema_defs() {
    let defs = tool_defs();
    let names: Vec<&str> = defs.iter().map(|def| def.function.name.as_str()).collect();

    assert_eq!(
        names,
        ["schedule_create", "schedule_list", "schedule_delete"]
    );
    assert_eq!(
        defs[0].function.name,
        schedule_create_schema_def().function.name
    );
    assert_eq!(
        defs[1].function.name,
        schedule_list_schema_def().function.name
    );
    assert_eq!(
        defs[2].function.name,
        schedule_delete_schema_def().function.name
    );
}
