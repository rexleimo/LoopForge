use super::{arrows, key_event_fields, named};

#[test]
fn arrow_key_event_fields_map_directional_keys() {
    let right = arrows::key_event_fields("ArrowRight").expect("ArrowRight mapping");
    assert_eq!(right.key, "ArrowRight");
    assert_eq!(right.code, "ArrowRight");
    assert_eq!(right.vkey, 39);
}

#[test]
fn named_key_event_fields_keep_escape_alias() {
    let escape = named::key_event_fields("Esc").expect("Esc mapping");
    assert_eq!(escape.key, "Escape");
    assert_eq!(escape.code, "Escape");
    assert_eq!(escape.vkey, 27);
    assert!(key_event_fields("Space").is_none());
}
