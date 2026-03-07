use super::super::defs::KeyEvent;

pub(super) fn key_event_fields(key: &str) -> Option<KeyEvent> {
    match key {
        "Enter" => Some(KeyEvent {
            key: "Enter",
            code: "Enter",
            vkey: 13,
        }),
        "Tab" => Some(KeyEvent {
            key: "Tab",
            code: "Tab",
            vkey: 9,
        }),
        "Escape" | "Esc" => Some(KeyEvent {
            key: "Escape",
            code: "Escape",
            vkey: 27,
        }),
        _ => None,
    }
}
