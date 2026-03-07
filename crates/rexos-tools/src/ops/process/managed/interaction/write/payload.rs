pub(super) fn stdin_payload(data: &str) -> String {
    if data.ends_with('\n') {
        data.to_string()
    } else {
        format!("{data}\n")
    }
}
