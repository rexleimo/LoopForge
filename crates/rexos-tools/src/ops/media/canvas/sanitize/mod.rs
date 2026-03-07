mod attrs;
mod limits;
mod schemes;
mod tags;

pub(super) fn sanitize_canvas_html(html: &str, max_bytes: usize) -> anyhow::Result<String> {
    limits::validate_html_size(html, max_bytes)?;
    let lower = html.to_ascii_lowercase();

    tags::validate_forbidden_tags(&lower)?;
    attrs::validate_event_handler_attrs(&lower)?;
    schemes::validate_forbidden_schemes(&lower)?;

    Ok(html.to_string())
}
