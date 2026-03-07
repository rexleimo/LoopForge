use super::parse::devtools_url_from_line;

#[test]
fn devtools_url_from_line_extracts_url() {
    let url = devtools_url_from_line(
        "[1234:5678] DevTools listening on ws://127.0.0.1:9222/devtools/browser/abc",
    )
    .unwrap();

    assert_eq!(
        url,
        Some("ws://127.0.0.1:9222/devtools/browser/abc".to_string())
    );
}

#[test]
fn devtools_url_from_line_ignores_unrelated_lines() {
    assert_eq!(
        devtools_url_from_line("[INFO] browser warmup").unwrap(),
        None
    );
}

#[test]
fn devtools_url_from_line_rejects_malformed_marker() {
    let err = devtools_url_from_line("DevTools listening on").unwrap_err();
    assert!(
        err.to_string().contains("malformed DevTools URL line"),
        "{err}"
    );
}
