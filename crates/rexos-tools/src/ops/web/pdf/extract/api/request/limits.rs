pub(super) fn resolved_limits(max_pages: Option<u64>, max_chars: Option<u64>) -> (usize, usize) {
    (
        super::super::super::limits::clamped_max_pages(max_pages),
        super::super::super::limits::clamped_max_chars(max_chars),
    )
}
