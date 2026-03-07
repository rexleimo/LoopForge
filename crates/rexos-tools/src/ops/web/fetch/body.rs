use crate::process_runtime::TOOL_OUTPUT_MIDDLE_OMISSION_MARKER;

pub(super) fn format_fetch_body(bytes: &[u8], max_bytes: usize) -> (String, bool, usize) {
    let truncated = bytes.len() > max_bytes;
    if !truncated {
        return (
            String::from_utf8_lossy(bytes).to_string(),
            false,
            bytes.len(),
        );
    }

    let marker = TOOL_OUTPUT_MIDDLE_OMISSION_MARKER.as_bytes();
    if max_bytes <= marker.len() + 2 {
        let slice = &bytes[..max_bytes];
        return (
            String::from_utf8_lossy(slice).to_string(),
            true,
            slice.len(),
        );
    }

    let budget = max_bytes.saturating_sub(marker.len());
    let tail_budget = (budget / 4).max(1);
    let head_budget = budget.saturating_sub(tail_budget).max(1);

    let head_slice = &bytes[..head_budget.min(bytes.len())];
    let tail_slice = &bytes[bytes.len().saturating_sub(tail_budget)..];

    let mut out = Vec::with_capacity(max_bytes);
    out.extend_from_slice(head_slice);
    out.extend_from_slice(marker);
    out.extend_from_slice(tail_slice);
    (String::from_utf8_lossy(&out).to_string(), true, out.len())
}
