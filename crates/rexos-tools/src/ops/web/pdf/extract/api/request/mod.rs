mod limits;
mod selection;
#[cfg(test)]
mod tests;

use super::super::io::{extract_pdf_pages, validate_pdf_file};
use super::super::truncation::truncate_chars;
use crate::Toolset;

impl Toolset {
    pub(crate) async fn pdf_extract(
        &self,
        user_path: &str,
        pages_spec: Option<&str>,
        max_pages: Option<u64>,
        max_chars: Option<u64>,
    ) -> anyhow::Result<String> {
        let path = self.resolve_workspace_path(user_path)?;
        let bytes = validate_pdf_file(&path, user_path)?;

        let (max_pages, max_chars) = limits::resolved_limits(max_pages, max_chars);

        let page_texts = extract_pdf_pages(path.clone()).await?;
        let total_pages = page_texts.len();
        let (combined, pages_extracted) =
            selection::selected_page_content(&page_texts, pages_spec, max_pages)?;

        let (text, truncated) = truncate_chars(&combined, max_chars);

        Ok(super::super::response::pdf_extract_payload(
            user_path,
            &text,
            truncated,
            bytes,
            total_pages,
            pages_spec,
            pages_extracted,
        )
        .to_string())
    }
}
