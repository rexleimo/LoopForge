#[derive(Debug, serde::Deserialize)]
pub(crate) struct ImageAnalyzeArgs {
    pub(crate) path: String,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct MediaDescribeArgs {
    pub(crate) path: String,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct MediaTranscribeArgs {
    pub(crate) path: String,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct SpeechToTextArgs {
    pub(crate) path: String,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct TextToSpeechArgs {
    pub(crate) text: String,
    #[serde(default)]
    pub(crate) path: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct ImageGenerateArgs {
    pub(crate) prompt: String,
    pub(crate) path: String,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct CanvasPresentArgs {
    pub(crate) html: String,
    #[serde(default)]
    pub(crate) title: Option<String>,
}
