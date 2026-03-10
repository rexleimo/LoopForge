use std::collections::HashMap;

use aws_smithy_types::Document;

pub(super) fn json_to_document(value: &serde_json::Value) -> Document {
    match value {
        serde_json::Value::Null => Document::Null,
        serde_json::Value::Bool(value) => Document::Bool(*value),
        serde_json::Value::Number(value) => {
            if let Some(value) = value.as_u64() {
                Document::Number(aws_smithy_types::Number::PosInt(value))
            } else if let Some(value) = value.as_i64() {
                Document::Number(aws_smithy_types::Number::NegInt(value))
            } else if let Some(value) = value.as_f64() {
                Document::Number(aws_smithy_types::Number::Float(value))
            } else {
                Document::Null
            }
        }
        serde_json::Value::String(value) => Document::String(value.clone()),
        serde_json::Value::Array(values) => {
            Document::Array(values.iter().map(json_to_document).collect())
        }
        serde_json::Value::Object(values) => Document::Object(
            values
                .iter()
                .map(|(key, value)| (key.clone(), json_to_document(value)))
                .collect::<HashMap<_, _>>(),
        ),
    }
}

pub(super) fn document_to_json(value: &Document) -> serde_json::Value {
    match value {
        Document::Null => serde_json::Value::Null,
        Document::Bool(value) => serde_json::Value::Bool(*value),
        Document::Number(value) => match value {
            aws_smithy_types::Number::PosInt(value) => {
                serde_json::Value::Number(serde_json::Number::from(*value))
            }
            aws_smithy_types::Number::NegInt(value) => {
                serde_json::Value::Number(serde_json::Number::from(*value))
            }
            aws_smithy_types::Number::Float(value) => serde_json::Number::from_f64(*value)
                .map(serde_json::Value::Number)
                .unwrap_or(serde_json::Value::Null),
        },
        Document::String(value) => serde_json::Value::String(value.clone()),
        Document::Array(values) => {
            serde_json::Value::Array(values.iter().map(document_to_json).collect())
        }
        Document::Object(values) => serde_json::Value::Object(
            values
                .iter()
                .map(|(key, value)| (key.clone(), document_to_json(value)))
                .collect(),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_document_round_trip_preserves_structure() {
        let json = serde_json::json!({
            "name": "test",
            "count": 42,
            "negative": -7,
            "ratio": 3.125,
            "active": true,
            "nothing": null,
            "tags": ["a", "b"],
            "nested": {"x": 1}
        });

        let doc = json_to_document(&json);
        let back = document_to_json(&doc);
        assert_eq!(json, back);
    }
}
