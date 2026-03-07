use super::limits::resolved_limits;
use super::selection::selected_page_content;

#[test]
fn resolved_limits_clamp_pages_and_chars() {
    assert_eq!(resolved_limits(None, None), (10, 12_000));
    assert_eq!(resolved_limits(Some(99), Some(99_999)), (50, 50_000));
}

#[test]
fn selected_page_content_applies_page_filter_and_limit() {
    let pages = vec![
        "page 1".to_string(),
        "page 2".to_string(),
        "page 3".to_string(),
    ];

    let (text, extracted) = selected_page_content(&pages, Some("2-3"), 1).unwrap();
    assert_eq!(text, "page 2");
    assert_eq!(extracted, 1);
}
