use std::path::Path;

use super::svg::{is_svg_output_path, placeholder_svg};

#[test]
fn svg_output_path_requires_lowercase_svg_extension() {
    assert!(is_svg_output_path(Path::new("art/output.svg")));
    assert!(!is_svg_output_path(Path::new("art/output.png")));
    assert!(!is_svg_output_path(Path::new("art/output.SVG")));
}

#[test]
fn placeholder_svg_escapes_prompt_text() {
    let svg = placeholder_svg("cats < dogs & \"birds\"");
    assert!(
        svg.contains("cats &lt; dogs &amp; &quot;birds&quot;"),
        "{svg}"
    );
}
