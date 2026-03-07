use serde_json::Value;

pub(super) fn add_headless_flag(mut value: Value, headless: bool) -> Value {
    if let Some(obj) = value.as_object_mut() {
        obj.insert("headless".to_string(), Value::Bool(headless));
    }
    value
}

pub(super) fn insert_optional_string(cmd: &mut Value, key: &str, value: Option<&str>) {
    if let Some(value) = value {
        cmd[key] = Value::String(value.to_string());
    }
}

pub(super) fn insert_optional_u64(cmd: &mut Value, key: &str, value: Option<u64>) {
    if let Some(value) = value {
        cmd[key] = Value::Number(value.into());
    }
}
