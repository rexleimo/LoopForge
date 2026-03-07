use anyhow::Context;
use rexos::memory::MemoryStore;

pub(crate) fn load_acp_events(
    memory: &MemoryStore,
    session: Option<&str>,
    limit: usize,
) -> anyhow::Result<Vec<serde_json::Value>> {
    let raw = memory
        .kv_get("rexos.acp.events")
        .context("kv_get rexos.acp.events")?
        .unwrap_or_else(|| "[]".to_string());
    let mut events: Vec<serde_json::Value> = serde_json::from_str(&raw).unwrap_or_default();

    if let Some(session) = session {
        let session = session.trim();
        if !session.is_empty() {
            events.retain(|ev| ev.get("session_id").and_then(|v| v.as_str()) == Some(session));
        }
    }

    let wanted = limit.max(1);
    if events.len() > wanted {
        events = events.split_off(events.len() - wanted);
    }
    Ok(events)
}

pub(crate) fn load_acp_checkpoints(
    memory: &MemoryStore,
    session: &str,
) -> anyhow::Result<Vec<serde_json::Value>> {
    let session = session.trim();
    if session.is_empty() {
        anyhow::bail!("session is empty");
    }
    let key = format!("rexos.acp.checkpoints.{session}");
    let raw = memory
        .kv_get(&key)
        .with_context(|| format!("kv_get {key}"))?
        .unwrap_or_else(|| "[]".to_string());
    let checkpoints: Vec<serde_json::Value> = serde_json::from_str(&raw).unwrap_or_default();
    Ok(checkpoints)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rexos::paths::RexosPaths;
    use tempfile::tempdir;

    #[test]
    fn load_acp_events_filters_by_session_and_limit() {
        let tmp = tempdir().unwrap();
        let paths = RexosPaths {
            base_dir: tmp.path().join(".loopforge"),
        };
        paths.ensure_dirs().unwrap();
        let memory = MemoryStore::open_or_create(&paths).unwrap();
        memory
            .kv_set(
                "rexos.acp.events",
                r#"[
  {"session_id":"s-1","step":1},
  {"session_id":"s-2","step":2},
  {"session_id":"s-1","step":3}
]"#,
            )
            .unwrap();

        let events = load_acp_events(&memory, Some("s-1"), 1).unwrap();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].get("step").and_then(|v| v.as_i64()), Some(3));
    }

    #[test]
    fn load_acp_checkpoints_rejects_empty_session() {
        let tmp = tempdir().unwrap();
        let paths = RexosPaths {
            base_dir: tmp.path().join(".loopforge"),
        };
        paths.ensure_dirs().unwrap();
        let memory = MemoryStore::open_or_create(&paths).unwrap();

        let err = load_acp_checkpoints(&memory, "  ").expect_err("empty session should fail");
        assert!(err.to_string().contains("session is empty"));
    }
}
