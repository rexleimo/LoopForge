use crate::browser_cdp::session::CdpBrowserSession;

pub(super) fn fallback_key_script(key: &str) -> String {
    let key_json = serde_json::to_string(key).unwrap_or_default();
    format!(
        r#"(() => {{
    let k = {key_json};
    let el = document.activeElement || document.body;
    try {{
      el.dispatchEvent(new KeyboardEvent('keydown', {{key: k, bubbles: true}}));
      el.dispatchEvent(new KeyboardEvent('keyup', {{key: k, bubbles: true}}));
    }} catch (e) {{}}
    if (k === 'Enter' && el) {{
      try {{
        const form = el.form || el.closest?.('form');
        if (form?.requestSubmit) form.requestSubmit();
      }} catch (e) {{}}
    }}
    return JSON.stringify({{ok: true, key: k}});
}})()"#
    )
}

pub(super) async fn dispatch_fallback_key(session: &CdpBrowserSession, script: &str) {
    let _ = session.cdp.run_js(script).await;
}
