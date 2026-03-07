mod read;
mod shared;
#[cfg(test)]
mod tests;
mod write;

#[cfg(test)]
fn encoded_bridge_command(cmd: &serde_json::Value) -> anyhow::Result<String> {
    write::encoded_bridge_command(cmd)
}

#[cfg(test)]
fn trimmed_bridge_response_line(line: &str) -> &str {
    read::trimmed_bridge_response_line(line)
}
