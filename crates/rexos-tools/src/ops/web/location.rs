pub(super) fn non_empty_env_value(value: Option<&str>) -> Option<String> {
    value
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
}

pub(super) fn location_get() -> anyhow::Result<String> {
    let tz = non_empty_env_value(std::env::var("TZ").ok().as_deref());
    let lang = non_empty_env_value(std::env::var("LANG").ok().as_deref());
    let lc_all = non_empty_env_value(std::env::var("LC_ALL").ok().as_deref());

    Ok(serde_json::json!({
        "os": std::env::consts::OS,
        "arch": std::env::consts::ARCH,
        "tz": tz,
        "lang": lang,
        "lc_all": lc_all,
        "geolocation": null,
        "note": "Exact geolocation is not available; LoopForge only reports environment metadata.",
    })
    .to_string())
}
