use anyhow::Context;
use base64::Engine as _;

pub(super) const BROWSER_SESSION_REQUIRED: &str =
    "browser session not started; call browser_navigate first";
pub(super) const DEFAULT_SCREENSHOT_PATH: &str = ".loopforge/browser/screenshot.png";

pub(super) fn decode_screenshot_bytes(data: &serde_json::Value) -> anyhow::Result<Vec<u8>> {
    let image_base64 = data
        .get("image_base64")
        .and_then(|value| value.as_str())
        .context("screenshot response missing image_base64")?;

    base64::engine::general_purpose::STANDARD
        .decode(image_base64)
        .context("decode screenshot base64")
}

pub(super) fn write_screenshot_file(path: &std::path::Path, bytes: &[u8]) -> anyhow::Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("create dirs {}", parent.display()))?;
    }
    std::fs::write(path, bytes).with_context(|| format!("write {}", path.display()))
}

pub(super) fn screenshot_payload(path: &str, url: Option<serde_json::Value>) -> String {
    serde_json::json!({
        "status": "ok",
        "path": path,
        "url": url.unwrap_or(serde_json::Value::Null),
    })
    .to_string()
}
