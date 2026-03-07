mod parse;
mod range;

use anyhow::bail;

use crate::Toolset;

impl Toolset {
    pub(crate) fn parse_pdf_pages_selector(spec: &str) -> anyhow::Result<Vec<usize>> {
        let spec = spec.trim();
        if spec.is_empty() {
            bail!("pages is empty");
        }

        let mut out = Vec::new();
        for part in spec.split(',') {
            let part = part.trim();
            if part.is_empty() {
                continue;
            }

            if let Some((start, end)) = part.split_once('-') {
                range::append_page_range(&mut out, part, start, end)?;
            } else {
                out.push(parse::parse_page_number(part)?);
            }
        }

        if out.is_empty() {
            bail!("pages selection is empty");
        }
        Ok(out)
    }
}
