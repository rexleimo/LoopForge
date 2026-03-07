mod canvas;
mod image;
mod speech;

use rexos_llm::openai_compat::ToolDefinition;

pub(super) fn tool_defs() -> Vec<ToolDefinition> {
    vec![
        image::image_generate_def(),
        speech::text_to_speech_def(),
        canvas::canvas_present_def(),
    ]
}
