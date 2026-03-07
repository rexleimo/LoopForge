use super::percent::decode_url_component;
use super::redirect::duckduckgo_redirect_target;

#[test]
fn duckduckgo_redirect_target_extracts_uddg_param() {
    assert_eq!(
        duckduckgo_redirect_target(
            "https://duckduckgo.com/l/?uddg=https%3A%2F%2Fexample.com%2Fa+b&rut=x"
        ),
        Some("https%3A%2F%2Fexample.com%2Fa+b")
    );
    assert_eq!(duckduckgo_redirect_target("https://example.com"), None);
}

#[test]
fn decode_url_component_decodes_hex_and_plus() {
    assert_eq!(
        decode_url_component("https%3A%2F%2Fexample.com%2Fa+b"),
        "https://example.com/a b"
    );
}
