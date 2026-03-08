use std::time::{SystemTime, UNIX_EPOCH};

use base64::Engine;
use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

const TOKEN_TTL_MS: i64 = 3 * 60 * 1000;

pub(super) fn to_bearer_token(api_key: &str) -> Option<String> {
    let key = api_key.trim();
    if key.is_empty() {
        return None;
    }

    let parts: Vec<&str> = key.split('.').collect();
    match parts.len() {
        2 => {
            let key_id = parts[0];
            let key_secret = parts[1];
            Some(sign_jwt(key_id, key_secret))
        }
        3 => Some(key.to_string()),
        _ => Some(key.to_string()),
    }
}

fn sign_jwt(key_id: &str, key_secret: &str) -> String {
    let now_ms: i64 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
        .try_into()
        .unwrap_or(0);
    let exp_ms = now_ms.saturating_add(TOKEN_TTL_MS);

    let header = serde_json::json!({
        "alg": "HS256",
        "sign_type": "SIGN",
        "typ": "JWT"
    });
    let claims = serde_json::json!({
        "api_key": key_id,
        "timestamp": now_ms,
        "exp": exp_ms
    });

    let header_b64 = b64_url_no_pad(serde_json::to_vec(&header).unwrap_or_default());
    let claims_b64 = b64_url_no_pad(serde_json::to_vec(&claims).unwrap_or_default());
    let signing_input = format!("{header_b64}.{claims_b64}");

    let mut mac = HmacSha256::new_from_slice(key_secret.as_bytes()).unwrap();
    mac.update(signing_input.as_bytes());
    let sig = mac.finalize().into_bytes();
    let sig_b64 = b64_url_no_pad(sig.to_vec());

    format!("{signing_input}.{sig_b64}")
}

fn b64_url_no_pad(data: Vec<u8>) -> String {
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(data)
}
