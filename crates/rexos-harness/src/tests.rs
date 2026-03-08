use serde_json::json;

use super::features::{first_failing_feature, tail_lines};

#[test]
fn tail_lines_returns_only_requested_suffix() {
    let joined = ["one", "two", "three", "four"].join("\n");
    assert_eq!(tail_lines(&joined, 2), vec!["three", "four"]);
    assert_eq!(tail_lines(&joined, 10), vec!["one", "two", "three", "four"]);
}

#[test]
fn first_failing_feature_returns_first_pending_entry() {
    let value = json!({
        "features": [
            { "id": "feat_1", "description": "done", "passes": true },
            { "id": "feat_2", "description": "next", "passes": false },
            { "id": "feat_3", "description": "later", "passes": false }
        ]
    });

    assert_eq!(
        first_failing_feature(&value).as_deref(),
        Some("feat_2 - next")
    );
}
