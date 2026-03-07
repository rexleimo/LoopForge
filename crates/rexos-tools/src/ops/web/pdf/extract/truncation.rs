pub(super) fn truncate_chars(input: &str, max_chars: usize) -> (String, bool) {
    let mut iter = input.chars();
    let text: String = iter.by_ref().take(max_chars).collect();
    let truncated = iter.next().is_some();
    (text, truncated)
}
