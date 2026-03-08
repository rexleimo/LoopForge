use super::*;
use rexos_llm::openai_compat::{ChatMessage, Role, ToolCall, ToolFunction};
use tempfile::tempdir;

#[test]
fn kv_round_trip() {
    let tmp = tempdir().unwrap();
    let db_path = tmp.path().join("test.db");
    let store = MemoryStore::open_or_create_at_path(&db_path).unwrap();

    assert_eq!(store.kv_get("missing").unwrap(), None);
    store.kv_set("a", "1").unwrap();
    assert_eq!(store.kv_get("a").unwrap(), Some("1".to_string()));
    store.kv_set("a", "2").unwrap();
    assert_eq!(store.kv_get("a").unwrap(), Some("2".to_string()));
}

#[test]
fn messages_persist_across_reopen() {
    let tmp = tempdir().unwrap();
    let db_path = tmp.path().join("test.db");

    {
        let store = MemoryStore::open_or_create_at_path(&db_path).unwrap();
        store.append_message("s1", "user", "hello").unwrap();
        store.append_message("s1", "assistant", "world").unwrap();
    }

    let store = MemoryStore::open_or_create_at_path(&db_path).unwrap();
    let msgs = store.list_messages("s1").unwrap();
    assert_eq!(msgs.len(), 2);
    assert_eq!(msgs[0].role, "user");
    assert_eq!(msgs[0].content, "hello");
    assert_eq!(msgs[1].role, "assistant");
    assert_eq!(msgs[1].content, "world");
}

#[test]
fn tool_calls_round_trip() {
    let tmp = tempdir().unwrap();
    let db_path = tmp.path().join("test.db");
    let store = MemoryStore::open_or_create_at_path(&db_path).unwrap();

    let assistant = ChatMessage {
        role: Role::Assistant,
        content: None,
        name: None,
        tool_call_id: None,
        tool_calls: Some(vec![ToolCall {
            id: "call_1".to_string(),
            kind: "function".to_string(),
            function: ToolFunction {
                name: "fs_read".to_string(),
                arguments: "{\"path\":\"README.md\"}".to_string(),
            },
        }]),
    };
    store.append_chat_message("s1", &assistant).unwrap();

    let tool = ChatMessage {
        role: Role::Tool,
        content: Some("file contents".to_string()),
        name: None,
        tool_call_id: Some("call_1".to_string()),
        tool_calls: None,
    };
    store.append_chat_message("s1", &tool).unwrap();

    let msgs = store.list_chat_messages("s1").unwrap();
    assert_eq!(msgs.len(), 2);
    assert_eq!(msgs[0].role, Role::Assistant);
    assert_eq!(msgs[0].content, None);
    assert_eq!(msgs[0].tool_calls.as_ref().unwrap()[0].id, "call_1");
    assert_eq!(msgs[1].role, Role::Tool);
    assert_eq!(msgs[1].tool_call_id.as_deref(), Some("call_1"));
    assert_eq!(msgs[1].content.as_deref(), Some("file contents"));
}
