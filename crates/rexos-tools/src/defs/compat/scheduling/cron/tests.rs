use super::{cron_cancel_schema_def, cron_create_schema_def, cron_list_schema_def, tool_defs};

#[test]
fn cron_tool_defs_match_individual_schema_defs() {
    let defs = tool_defs();
    let names: Vec<&str> = defs.iter().map(|def| def.function.name.as_str()).collect();

    assert_eq!(names, ["cron_create", "cron_list", "cron_cancel"]);
    assert_eq!(
        defs[0].function.name,
        cron_create_schema_def().function.name
    );
    assert_eq!(defs[1].function.name, cron_list_schema_def().function.name);
    assert_eq!(
        defs[2].function.name,
        cron_cancel_schema_def().function.name
    );
}
