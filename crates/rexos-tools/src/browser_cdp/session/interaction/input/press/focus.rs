use crate::browser_cdp::session::CdpBrowserSession;

pub(super) async fn focus_selector_if_present(session: &CdpBrowserSession, selector: Option<&str>) {
    if let Some(selector) = selector {
        let sel_json = serde_json::to_string(selector).unwrap_or_default();
        let js = format!(
            r#"(() => {{
    let sel = {sel_json};
    let el = document.querySelector(sel);
    if (el) el.focus();
    return JSON.stringify({{focused: !!el, selector: sel}});
}})()"#
        );
        let _ = session.cdp.run_js(&js).await;
    }
}
