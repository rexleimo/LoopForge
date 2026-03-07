use tokio_tungstenite::tungstenite::Message as WsMessage;

pub(super) fn message_text(
    msg: Result<WsMessage, tokio_tungstenite::tungstenite::Error>,
) -> Option<String> {
    match msg {
        Ok(WsMessage::Text(text)) => Some(text.to_string()),
        Ok(WsMessage::Close(_)) => None,
        Err(_) => None,
        _ => Some(String::new()),
    }
}
