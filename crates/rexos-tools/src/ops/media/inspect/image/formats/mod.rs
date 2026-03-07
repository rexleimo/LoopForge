mod gif;
mod jpeg;
mod png;

pub(super) fn detect_image_format_and_dimensions(bytes: &[u8]) -> Option<(&'static str, u32, u32)> {
    png::parse_png_dimensions(bytes)
        .map(|(width, height)| ("png", width, height))
        .or_else(|| {
            jpeg::parse_jpeg_dimensions(bytes).map(|(width, height)| ("jpeg", width, height))
        })
        .or_else(|| gif::parse_gif_dimensions(bytes).map(|(width, height)| ("gif", width, height)))
}
