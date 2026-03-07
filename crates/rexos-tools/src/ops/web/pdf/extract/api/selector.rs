use crate::Toolset;

pub(super) fn parse_selected_page_numbers(
    pages_spec: Option<&str>,
) -> anyhow::Result<Option<Vec<usize>>> {
    match pages_spec {
        Some(spec) => Ok(Some(Toolset::parse_pdf_pages_selector(spec)?)),
        None => Ok(None),
    }
}
