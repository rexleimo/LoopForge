use anyhow::Context;

pub(crate) fn select_onboard_model(preferred: &str, available: &[String]) -> Option<String> {
    if available.is_empty() {
        return None;
    }

    let preferred = preferred.trim();
    if !preferred.is_empty() {
        if let Some(hit) = available
            .iter()
            .find(|model| model.trim().eq_ignore_ascii_case(preferred))
        {
            return Some(hit.clone());
        }
    }

    if let Some(chat_like) = available.iter().find(|model| {
        let lower = model.to_ascii_lowercase();
        !lower.contains("embed")
    }) {
        return Some(chat_like.clone());
    }
    Some(available[0].clone())
}

pub(crate) async fn fetch_openai_compat_models(
    base_url: &str,
    timeout_ms: u64,
) -> anyhow::Result<Vec<String>> {
    let endpoint = format!("{}/models", base_url.trim_end_matches('/'));
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_millis(timeout_ms.max(500)))
        .build()
        .context("build model probe http client")?;
    let res = client.get(&endpoint).send().await?;
    if !res.status().is_success() {
        anyhow::bail!("GET {endpoint} -> {}", res.status());
    }
    let value: serde_json::Value = res.json().await?;
    let mut out = Vec::new();
    if let Some(items) = value.get("data").and_then(|entry| entry.as_array()) {
        for item in items {
            if let Some(id) = item.get("id").and_then(|entry| entry.as_str()) {
                let id = id.trim();
                if !id.is_empty() {
                    out.push(id.to_string());
                    continue;
                }
            }
            if let Some(name) = item.get("name").and_then(|entry| entry.as_str()) {
                let name = name.trim();
                if !name.is_empty() {
                    out.push(name.to_string());
                }
            }
        }
    }
    out.sort();
    out.dedup();
    Ok(out)
}
