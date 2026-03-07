use super::*;

#[test]
fn decode_process_output_decodes_utf16le_like_powershell() {
    let mut bytes = Vec::new();
    for b in b"READY\r\n" {
        bytes.push(*b);
        bytes.push(0);
    }

    let out = Toolset::decode_process_output(bytes);
    assert!(out.contains("READY"), "{out:?}");
}

#[tokio::test]
async fn process_poll_truncation_preserves_head_and_tail() {
    use std::time::Duration;

    let tmp = tempfile::tempdir().unwrap();
    let workspace = tmp.path().join("ws");
    std::fs::create_dir_all(&workspace).unwrap();

    let tools = Toolset::new(workspace).unwrap();

    let start_args = if cfg!(windows) {
        serde_json::json!({
            "command": "powershell",
            "args": [
                "-NoProfile",
                "-NonInteractive",
                "-ExecutionPolicy",
                "Bypass",
                "-Command",
                "[Console]::Out.WriteLine('HEAD_START'); [Console]::Out.Write(('A' * 350000)); [Console]::Out.WriteLine(''); [Console]::Out.WriteLine('TAIL_END'); [Console]::Out.Flush(); Start-Sleep -Seconds 5"
            ]
        })
    } else {
        serde_json::json!({
            "command": "bash",
            "args": ["-lc", "echo HEAD_START; head -c 350000 < /dev/zero | tr '\\0' 'A'; echo; echo TAIL_END; sleep 5"]
        })
    };

    let out = tools
        .call("process_start", &start_args.to_string())
        .await
        .unwrap();
    let v: serde_json::Value = serde_json::from_str(&out).expect("process_start is json");
    let process_id = v
        .get("process_id")
        .and_then(|v| v.as_str())
        .expect("process_id")
        .to_string();

    tokio::time::sleep(Duration::from_millis(if cfg!(windows) {
        1200
    } else {
        500
    }))
    .await;

    let deadline = tokio::time::Instant::now()
        + if cfg!(windows) {
            Duration::from_secs(10)
        } else {
            Duration::from_secs(5)
        };

    let mut seen = String::new();
    loop {
        let poll = tools
            .call(
                "process_poll",
                &format!(r#"{{ "process_id": "{}" }}"#, process_id),
            )
            .await
            .unwrap();
        let pv: serde_json::Value = serde_json::from_str(&poll).expect("process_poll is json");
        let stdout = pv.get("stdout").and_then(|v| v.as_str()).unwrap_or("");
        let stderr = pv.get("stderr").and_then(|v| v.as_str()).unwrap_or("");
        seen.push_str(stdout);
        seen.push_str(stderr);

        if seen.contains("TAIL_END") {
            break;
        }
        if pv.get("alive").and_then(|v| v.as_bool()) == Some(false) {
            break;
        }
        if tokio::time::Instant::now() >= deadline {
            break;
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }

    assert!(
        seen.contains("TAIL_END"),
        "did not see TAIL_END\noutput:\n{}",
        seen
    );
    assert!(
        seen.contains("HEAD_START"),
        "expected truncated output to preserve head and tail\noutput:\n{}",
        seen
    );
    assert!(
        seen.contains("[... middle omitted ...]"),
        "expected omission marker in truncated output\noutput:\n{}",
        seen
    );

    let _ = tools
        .call(
            "process_kill",
            &format!(r#"{{ "process_id": "{}" }}"#, process_id),
        )
        .await
        .unwrap();
}
