use serde_json::json;
use tokio_tungstenite::tungstenite::Message as WsMessage;

use super::{incoming_message_text, parsed_response_error, response_id, response_result};

#[test]
fn incoming_message_text_distinguishes_text_close_and_other_frames() {
    assert_eq!(
        incoming_message_text(Ok(WsMessage::Text("hello".into()))),
        Some("hello".to_string())
    );
    assert_eq!(
        incoming_message_text(Ok(WsMessage::Binary(vec![1].into()))),
        Some(String::new())
    );
    assert_eq!(incoming_message_text(Ok(WsMessage::Close(None))), None);
}

#[test]
fn parsed_response_error_uses_message_or_default() {
    assert_eq!(
        parsed_response_error(&json!({"error": {"message": "boom"}}))
            .unwrap()
            .to_string(),
        "boom"
    );
    assert_eq!(
        parsed_response_error(&json!({"error": {}}))
            .unwrap()
            .to_string(),
        "CDP error"
    );
    assert!(parsed_response_error(&json!({"result": {}})).is_none());
}

#[test]
fn response_helpers_extract_id_and_result_defaults() {
    assert_eq!(response_id(&json!({"id": 9})), Some(9));
    assert_eq!(
        response_result(&json!({"result": {"ok": true}})),
        json!({"ok": true})
    );
    assert_eq!(response_result(&json!({})), serde_json::Value::Null);
}
