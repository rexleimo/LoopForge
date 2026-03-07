use anyhow::bail;

pub(super) fn selected_pages(
    page_texts: &[String],
    requested: Option<&[usize]>,
    total_pages: usize,
) -> anyhow::Result<Vec<String>> {
    let Some(requested) = requested else {
        return Ok(page_texts.to_vec());
    };

    let mut out = Vec::with_capacity(requested.len());
    for &page_no in requested {
        if page_no == 0 || page_no > total_pages {
            bail!("page out of range: {page_no} (valid range: 1..={total_pages})");
        }
        out.push(page_texts[page_no - 1].clone());
    }
    Ok(out)
}
