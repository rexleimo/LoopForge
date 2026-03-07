#[test]
fn stdin_payload_appends_newline_when_missing() {
    assert_eq!(super::payload::stdin_payload("hi"), "hi\n");
    assert_eq!(super::payload::stdin_payload("hi\n"), "hi\n");
}

#[test]
fn process_write_ok_output_stays_stable() {
    assert_eq!(
        super::response::process_write_ok_output(),
        r#"{"status":"written"}"#
    );
}
