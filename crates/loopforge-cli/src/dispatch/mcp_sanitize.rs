use serde_json::Value;

pub(crate) fn sanitize_mcp_config(value: &Value) -> Value {
    match value {
        Value::Object(map) => {
            let mut out = serde_json::Map::new();
            for (k, v) in map {
                if k == "env" {
                    match v.as_object() {
                        Some(env) => {
                            let mut redacted = serde_json::Map::new();
                            for (ek, _ev) in env {
                                redacted
                                    .insert(ek.clone(), Value::String("[redacted]".to_string()));
                            }
                            out.insert(k.clone(), Value::Object(redacted));
                        }
                        None => {
                            out.insert(k.clone(), Value::String("[redacted]".to_string()));
                        }
                    }
                    continue;
                }
                out.insert(k.clone(), sanitize_mcp_config(v));
            }
            Value::Object(out)
        }
        Value::Array(values) => Value::Array(values.iter().map(sanitize_mcp_config).collect()),
        other => other.clone(),
    }
}
