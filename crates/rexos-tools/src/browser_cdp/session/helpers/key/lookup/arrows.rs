use super::super::defs::KeyEvent;

pub(super) fn key_event_fields(key: &str) -> Option<KeyEvent> {
    match key {
        "ArrowUp" => Some(KeyEvent {
            key: "ArrowUp",
            code: "ArrowUp",
            vkey: 38,
        }),
        "ArrowDown" => Some(KeyEvent {
            key: "ArrowDown",
            code: "ArrowDown",
            vkey: 40,
        }),
        "ArrowLeft" => Some(KeyEvent {
            key: "ArrowLeft",
            code: "ArrowLeft",
            vkey: 37,
        }),
        "ArrowRight" => Some(KeyEvent {
            key: "ArrowRight",
            code: "ArrowRight",
            vkey: 39,
        }),
        _ => None,
    }
}
