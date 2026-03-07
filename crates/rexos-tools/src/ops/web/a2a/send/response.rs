use anyhow::bail;

pub(super) fn parse_send_response(value: serde_json::Value) -> anyhow::Result<String> {
    if let Some(result) = value.get("result") {
        return Ok(serde_json::to_string_pretty(result).unwrap_or_else(|_| result.to_string()));
    }
    if let Some(error) = value.get("error") {
        bail!("a2a_send error: {error}");
    }
    bail!("invalid a2a_send response")
}
