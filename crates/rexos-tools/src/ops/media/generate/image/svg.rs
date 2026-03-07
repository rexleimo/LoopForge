use std::path::Path;

pub(super) fn is_svg_output_path(path: &Path) -> bool {
    path.extension().and_then(|ext| ext.to_str()).unwrap_or("") == "svg"
}

pub(super) fn placeholder_svg(prompt: &str) -> String {
    let escaped = super::super::super::escape_xml_text(prompt);
    format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="800" height="450" viewBox="0 0 800 450"><rect width="100%" height="100%" fill="#0b1020"/><text x="40" y="120" fill="#e2e8f0" font-size="48" font-family="Inter, system-ui, -apple-system, Segoe UI, Roboto, Arial">{escaped}</text></svg>"##
    )
}
