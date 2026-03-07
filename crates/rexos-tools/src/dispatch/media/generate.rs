use crate::defs::{CanvasPresentArgs, ImageGenerateArgs, TextToSpeechArgs};
use crate::Toolset;

pub(super) fn dispatch(
    toolset: &Toolset,
    name: &str,
    arguments_json: &str,
) -> anyhow::Result<String> {
    match name {
        "text_to_speech" => {
            let args: TextToSpeechArgs =
                super::super::parse_args(arguments_json, "text_to_speech")?;
            toolset.text_to_speech(&args.text, args.path.as_deref())
        }
        "image_generate" => {
            let args: ImageGenerateArgs =
                super::super::parse_args(arguments_json, "image_generate")?;
            toolset.image_generate(&args.prompt, &args.path)
        }
        "canvas_present" => {
            let args: CanvasPresentArgs =
                super::super::parse_args(arguments_json, "canvas_present")?;
            toolset.canvas_present(&args.html, args.title.as_deref())
        }
        _ => unreachable!("unexpected generate media tool: {name}"),
    }
}
