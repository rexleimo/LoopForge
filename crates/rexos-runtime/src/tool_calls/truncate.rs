pub(crate) fn truncate_tool_result_with_flag(output: String, max_chars: usize) -> (String, bool) {
    if max_chars == 0 {
        return (String::new(), !output.is_empty());
    }

    let total_chars = output.chars().count();
    if total_chars <= max_chars {
        return (output, false);
    }

    let head_chars = max_chars / 2;
    let tail_chars = max_chars - head_chars;
    let omitted = total_chars.saturating_sub(max_chars);

    let head: String = output.chars().take(head_chars).collect();
    let tail: String = output
        .chars()
        .skip(total_chars.saturating_sub(tail_chars))
        .collect();

    (
        format!("{head}\n\n[... omitted {omitted} chars ...]\n\n{tail}"),
        true,
    )
}
