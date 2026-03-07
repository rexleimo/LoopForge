pub(super) fn clamped_max_pages(max_pages: Option<u64>) -> usize {
    max_pages.unwrap_or(10).clamp(1, 50) as usize
}

pub(super) fn clamped_max_chars(max_chars: Option<u64>) -> usize {
    max_chars.unwrap_or(12_000).clamp(1, 50_000) as usize
}
