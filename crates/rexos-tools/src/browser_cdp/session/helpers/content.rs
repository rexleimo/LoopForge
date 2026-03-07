pub(crate) const EXTRACT_CONTENT_JS: &str = r#"(() => {
  function clean(s) {
    return (s || '').replace(/\s+/g, ' ').trim();
  }
  let title = document.title || '';
  let url = location.href || '';
  let body = '';
  try {
    body = document.body ? document.body.innerText : '';
  } catch (e) {
    body = '';
  }
  body = body || '';
  const max = 50000;
  let truncated = false;
  if (body.length > max) {
    body = body.substring(0, max);
    truncated = true;
  }
  return JSON.stringify({
    title: clean(title),
    url: url,
    content: body + (truncated ? `\n\n[Truncated — ${max} chars]` : ''),
  });
})()"#;
