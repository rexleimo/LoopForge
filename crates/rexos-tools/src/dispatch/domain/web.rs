pub(super) fn is_web_tool(name: &str) -> bool {
    matches!(
        name,
        "web_fetch"
            | "pdf"
            | "pdf_extract"
            | "web_search"
            | "a2a_discover"
            | "a2a_send"
            | "location_get"
    )
}
