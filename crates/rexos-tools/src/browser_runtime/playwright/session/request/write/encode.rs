use anyhow::Context;

pub(super) fn encoded_bridge_command(cmd: &serde_json::Value) -> anyhow::Result<String> {
    Ok(format!(
        "{}\n",
        serde_json::to_string(cmd).context("encode bridge command")?
    ))
}
