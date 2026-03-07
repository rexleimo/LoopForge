mod json;
mod message;
mod relay;
mod response;

#[cfg(test)]
mod tests;

fn incoming_message_text(
    msg: Result<tokio_tungstenite::tungstenite::Message, tokio_tungstenite::tungstenite::Error>,
) -> Option<String> {
    message::message_text(msg)
}

fn parsed_response_error(json: &serde_json::Value) -> Option<anyhow::Error> {
    response::response_error(json)
}

fn response_id(json: &serde_json::Value) -> Option<u64> {
    json::response_id(json)
}

fn response_result(json: &serde_json::Value) -> serde_json::Value {
    json::response_result(json)
}
