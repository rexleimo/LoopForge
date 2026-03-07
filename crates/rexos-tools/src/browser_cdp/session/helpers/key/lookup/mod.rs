mod arrows;
mod named;
#[cfg(test)]
mod tests;

use super::defs::KeyEvent;

pub(crate) fn key_event_fields(key: &str) -> Option<KeyEvent> {
    named::key_event_fields(key).or_else(|| arrows::key_event_fields(key))
}
