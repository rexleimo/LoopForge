use serial_test::serial;

#[test]
#[serial]
fn replay_fixtures_are_valid_and_referenced_by_session_replay() {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let fixture_dir = manifest_dir.join("tests/fixtures/replay");
    let source_root_path = manifest_dir.join("tests/session_replay.rs");
    let source_module_dir = manifest_dir.join("tests/session_replay");

    let mut source = std::fs::read_to_string(&source_root_path)
        .unwrap_or_else(|err| panic!("read {}: {err}", source_root_path.display()));

    for entry in std::fs::read_dir(&source_module_dir)
        .unwrap_or_else(|err| panic!("read {}: {err}", source_module_dir.display()))
    {
        let entry = entry.expect("session_replay module entry");
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("rs") {
            continue;
        }
        let module_source = std::fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("read {}: {err}", path.display()));
        source.push('\n');
        source.push_str(&module_source);
    }

    let mut fixture_names: Vec<String> = Vec::new();
    for entry in std::fs::read_dir(&fixture_dir)
        .unwrap_or_else(|err| panic!("read {}: {err}", fixture_dir.display()))
    {
        let entry = entry.expect("fixture dir entry");
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
            continue;
        }

        let file_name = path
            .file_name()
            .expect("fixture file name")
            .to_string_lossy()
            .to_string();
        let raw = std::fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("read {}: {err}", path.display()));

        let _: Vec<serde_json::Value> = serde_json::from_str(&raw)
            .unwrap_or_else(|err| panic!("fixture `{file_name}` must be a JSON array: {err}"));

        fixture_names.push(file_name);
    }

    fixture_names.sort();
    assert!(
        !fixture_names.is_empty(),
        "expected fixtures under {}",
        fixture_dir.display()
    );

    for name in fixture_names {
        let needle = format!("fixtures/replay/{name}");
        assert!(
            source.contains(&needle),
            "fixture `{name}` exists on disk but is not referenced by session_replay.rs via `{needle}`"
        );
    }
}
