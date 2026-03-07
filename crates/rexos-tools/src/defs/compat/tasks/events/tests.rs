use super::{channel_send_def, event_publish_def, event_tool_defs};

#[test]
fn event_tool_defs_match_individual_defs() {
    let defs = event_tool_defs();
    let names: Vec<&str> = defs.iter().map(|def| def.function.name.as_str()).collect();

    assert_eq!(names, ["event_publish", "channel_send"]);
    assert_eq!(defs[0].function.name, event_publish_def().function.name);
    assert_eq!(defs[1].function.name, channel_send_def().function.name);
}

#[test]
fn event_and_channel_defs_keep_expected_required_fields() {
    let event_required = event_publish_def().function.parameters["required"].clone();
    let channel_required = channel_send_def().function.parameters["required"].clone();

    assert_eq!(event_required, serde_json::json!(["event_type"]));
    assert_eq!(
        channel_required,
        serde_json::json!(["channel", "recipient", "message"])
    );
}
