use super::content::combined_selected_text;
use super::selector::parse_selected_page_numbers;

#[test]
fn combined_selected_text_joins_pages_and_respects_limit() {
    let text = combined_selected_text(
        vec![
            "page 1".to_string(),
            "page 2".to_string(),
            "page 3".to_string(),
        ],
        2,
    );

    assert_eq!(text, "page 1\n\npage 2");
}

#[test]
fn parse_selected_page_numbers_preserves_absent_selector() {
    assert_eq!(parse_selected_page_numbers(None).unwrap(), None);
}
