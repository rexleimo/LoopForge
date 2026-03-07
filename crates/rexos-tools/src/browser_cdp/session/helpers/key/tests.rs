use super::key_event_fields;

#[test]
fn key_event_fields_maps_enter_and_escape_aliases() {
    let enter = key_event_fields("Enter").expect("Enter mapping");
    assert_eq!(enter.key, "Enter");
    assert_eq!(enter.code, "Enter");
    assert_eq!(enter.vkey, 13);

    let escape = key_event_fields("Esc").expect("Esc mapping");
    assert_eq!(escape.key, "Escape");
    assert_eq!(escape.code, "Escape");
    assert_eq!(escape.vkey, 27);
}

#[test]
fn key_event_fields_rejects_unknown_keys() {
    assert!(key_event_fields("Space").is_none());
}
