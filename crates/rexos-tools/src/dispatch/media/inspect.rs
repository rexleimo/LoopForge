use crate::defs::{ImageAnalyzeArgs, MediaDescribeArgs, MediaTranscribeArgs, SpeechToTextArgs};
use crate::Toolset;

pub(super) fn dispatch(
    toolset: &Toolset,
    name: &str,
    arguments_json: &str,
) -> anyhow::Result<String> {
    match name {
        "image_analyze" => {
            let args: ImageAnalyzeArgs = super::super::parse_args(arguments_json, "image_analyze")?;
            toolset.image_analyze(&args.path)
        }
        "media_describe" => {
            let args: MediaDescribeArgs =
                super::super::parse_args(arguments_json, "media_describe")?;
            toolset.media_describe(&args.path)
        }
        "media_transcribe" => {
            let args: MediaTranscribeArgs =
                super::super::parse_args(arguments_json, "media_transcribe")?;
            toolset.media_transcribe(&args.path)
        }
        "speech_to_text" => {
            let args: SpeechToTextArgs =
                super::super::parse_args(arguments_json, "speech_to_text")?;
            toolset.speech_to_text(&args.path)
        }
        _ => unreachable!("unexpected inspect media tool: {name}"),
    }
}
