use std::path::Path;

use anyhow::{bail, Context};

use crate::{FEATURES_JSON, INIT_PS1, INIT_SH, PROGRESS_MD};

pub(super) fn tail_lines(s: &str, n: usize) -> Vec<&str> {
    let mut lines: Vec<&str> = s.lines().collect();
    if lines.len() > n {
        lines.drain(0..lines.len() - n);
    }
    lines
}

pub(super) fn is_initialized(workspace_dir: &Path) -> bool {
    workspace_dir.join(FEATURES_JSON).exists()
        && workspace_dir.join(PROGRESS_MD).exists()
        && (workspace_dir.join(INIT_SH).exists() || workspace_dir.join(INIT_PS1).exists())
}

pub(super) fn ensure_features_populated(workspace_dir: &Path) -> anyhow::Result<()> {
    let path = workspace_dir.join(FEATURES_JSON);
    let raw = std::fs::read_to_string(&path).with_context(|| format!("read {}", path.display()))?;
    let v: serde_json::Value =
        serde_json::from_str(&raw).with_context(|| format!("parse {}", path.display()))?;
    let n = v
        .get("features")
        .and_then(|v| v.as_array())
        .map(|a| a.len())
        .unwrap_or(0);
    if n == 0 {
        bail!(
            "initializer did not populate features.json (features=[]). Ensure your model supports tool calling and actually uses fs_write/shell to update the workspace."
        );
    }
    Ok(())
}

pub(super) fn normalize_features_json(workspace_dir: &Path) -> anyhow::Result<()> {
    let path = workspace_dir.join(FEATURES_JSON);
    let raw = std::fs::read_to_string(&path).with_context(|| format!("read {}", path.display()))?;
    let mut v: serde_json::Value =
        serde_json::from_str(&raw).with_context(|| format!("parse {}", path.display()))?;

    let mut changed = false;

    if v.get("version").and_then(|x| x.as_i64()).is_none() {
        v["version"] = serde_json::Value::Number(1.into());
        changed = true;
    }

    if v.get("updated_at").and_then(|x| x.as_str()).is_none() {
        v["updated_at"] = serde_json::Value::String(String::new());
        changed = true;
    }

    let default_editing =
        "Only change `passes` (false -> true) and optionally `notes`. Do not delete or reorder items.";
    let default_completion =
        "A feature can only be marked passing after required tests/smoke checks are run.";

    if v.get("rules").and_then(|x| x.as_object()).is_none() {
        v["rules"] = serde_json::json!({
            "editing": default_editing,
            "completion": default_completion
        });
        changed = true;
    } else if let Some(obj) = v.get_mut("rules").and_then(|x| x.as_object_mut()) {
        if obj.get("editing").and_then(|x| x.as_str()).is_none() {
            obj.insert(
                "editing".to_string(),
                serde_json::Value::String(default_editing.to_string()),
            );
            changed = true;
        }
        if obj.get("completion").and_then(|x| x.as_str()).is_none() {
            obj.insert(
                "completion".to_string(),
                serde_json::Value::String(default_completion.to_string()),
            );
            changed = true;
        }
    }

    if v.get("features").and_then(|x| x.as_array()).is_none() {
        v["features"] = serde_json::Value::Array(Vec::new());
        changed = true;
    }

    if let Some(arr) = v.get_mut("features").and_then(|x| x.as_array_mut()) {
        for f in arr {
            let Some(obj) = f.as_object_mut() else {
                continue;
            };
            if obj.get("passes").and_then(|x| x.as_bool()).is_none() {
                obj.insert("passes".to_string(), serde_json::Value::Bool(false));
                changed = true;
            }
        }
    }

    if changed {
        let s = serde_json::to_string_pretty(&v).context("serialize features.json")?;
        std::fs::write(&path, s).with_context(|| format!("write {}", path.display()))?;
    }

    Ok(())
}

pub(super) fn first_failing_feature(v: &serde_json::Value) -> Option<String> {
    let arr = v.get("features")?.as_array()?;
    for f in arr {
        if f.get("passes").and_then(|p| p.as_bool()) == Some(false) {
            let id = f.get("id").and_then(|x| x.as_str()).unwrap_or("<no id>");
            let desc = f.get("description").and_then(|x| x.as_str()).unwrap_or("");
            return Some(format!("{id} - {desc}"));
        }
    }
    None
}
