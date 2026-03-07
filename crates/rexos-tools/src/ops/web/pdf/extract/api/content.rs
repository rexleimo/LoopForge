pub(super) fn combined_selected_text(selected_pages: Vec<String>, max_pages: usize) -> String {
    selected_pages
        .into_iter()
        .take(max_pages)
        .collect::<Vec<_>>()
        .join("\n\n")
}
