mod generate;
mod inspect;

use crate::Toolset;

impl Toolset {
    pub(super) fn call_media_tool(
        &self,
        name: &str,
        arguments_json: &str,
    ) -> anyhow::Result<String> {
        match name {
            "image_analyze" | "media_describe" | "media_transcribe" | "speech_to_text" => {
                inspect::dispatch(self, name, arguments_json)
            }
            "text_to_speech" | "image_generate" | "canvas_present" => {
                generate::dispatch(self, name, arguments_json)
            }
            _ => unreachable!("unexpected media tool: {name}"),
        }
    }
}
