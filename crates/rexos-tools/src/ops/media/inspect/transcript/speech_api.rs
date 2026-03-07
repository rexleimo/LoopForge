use anyhow::Context;

use crate::Toolset;

impl Toolset {
    pub(crate) fn speech_to_text(&self, user_path: &str) -> anyhow::Result<String> {
        let out = self.media_transcribe(user_path)?;
        let value: serde_json::Value =
            serde_json::from_str(&out).context("parse media_transcribe output")?;

        Ok(super::speech::speech_to_text_payload(
            user_path,
            &value
                .get("text")
                .cloned()
                .unwrap_or(serde_json::Value::Null),
        )
        .to_string())
    }
}
