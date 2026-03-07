use super::super::super::selection::selected_pages;
use super::super::content::combined_selected_text;
use super::super::selector::parse_selected_page_numbers;

pub(super) fn selected_page_content(
    page_texts: &[String],
    pages_spec: Option<&str>,
    max_pages: usize,
) -> anyhow::Result<(String, usize)> {
    let total_pages = page_texts.len();
    let selected_page_numbers = parse_selected_page_numbers(pages_spec)?;
    let selected_pages = selected_pages(page_texts, selected_page_numbers.as_deref(), total_pages)?;
    let pages_extracted = selected_pages.len().min(max_pages);
    let combined = combined_selected_text(selected_pages, max_pages);
    Ok((combined, pages_extracted))
}
