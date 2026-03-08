use super::{
    acp_delivery_checkpoints_get, acp_delivery_checkpoints_set, acp_events_get, append_acp_event,
};
use crate::records::{AcpDeliveryCheckpointRecord, AcpEventRecord};
use crate::ACP_EVENTS_MAX_RECORDS;
use rexos_kernel::paths::RexosPaths;
use rexos_memory::MemoryStore;

fn test_paths() -> RexosPaths {
    let base = std::env::temp_dir().join(format!("rexos-runtime-test-{}", uuid::Uuid::new_v4()));
    RexosPaths { base_dir: base }
}

#[test]
fn append_event_keeps_only_recent_records_when_over_cap() {
    let paths = test_paths();
    paths.ensure_dirs().unwrap();
    let memory = MemoryStore::open_or_create(&paths).unwrap();

    let seeded_events: Vec<AcpEventRecord> = (0..ACP_EVENTS_MAX_RECORDS)
        .map(|idx| AcpEventRecord {
            id: format!("e-{idx}"),
            session_id: Some("s-1".to_string()),
            event_type: "demo".to_string(),
            payload: serde_json::json!({"idx": idx}),
            created_at: idx as i64,
        })
        .collect();
    super::events::acp_events_set(&memory, &seeded_events).unwrap();

    for idx in ACP_EVENTS_MAX_RECORDS..(ACP_EVENTS_MAX_RECORDS + 5) {
        append_acp_event(
            &memory,
            AcpEventRecord {
                id: format!("e-{idx}"),
                session_id: Some("s-1".to_string()),
                event_type: "demo".to_string(),
                payload: serde_json::json!({"idx": idx}),
                created_at: idx as i64,
            },
        )
        .unwrap();
    }

    let events = acp_events_get(&memory).unwrap();
    assert_eq!(events.len(), ACP_EVENTS_MAX_RECORDS);
    assert_eq!(events.first().map(|event| event.id.as_str()), Some("e-5"));
    assert_eq!(
        events.last().map(|event| event.id.as_str()),
        Some(format!("e-{}", ACP_EVENTS_MAX_RECORDS + 4).as_str())
    );
}

#[test]
fn checkpoints_round_trip_for_session() {
    let paths = test_paths();
    paths.ensure_dirs().unwrap();
    let memory = MemoryStore::open_or_create(&paths).unwrap();
    let checkpoints = vec![AcpDeliveryCheckpointRecord {
        channel: "console".to_string(),
        cursor: "c-1".to_string(),
        updated_at: 42,
    }];

    acp_delivery_checkpoints_set(&memory, "session-1", &checkpoints).unwrap();
    let loaded = acp_delivery_checkpoints_get(&memory, "session-1").unwrap();
    assert_eq!(loaded.len(), 1);
    assert_eq!(loaded[0].cursor, "c-1");
}
