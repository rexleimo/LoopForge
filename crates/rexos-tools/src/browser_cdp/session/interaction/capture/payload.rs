use serde_json::Value;

pub(super) fn screenshot_result(url: String, image_base64: &str) -> Value {
    serde_json::json!({
        "format": "png",
        "url": url,
        "image_base64": image_base64,
    })
}
