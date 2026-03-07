use super::*;

#[test]
fn tool_definitions_include_pdf() {
    let tmp = tempfile::tempdir().unwrap();
    let tools = Toolset::new(tmp.path().to_path_buf()).unwrap();
    let defs = tools
        .definitions()
        .into_iter()
        .map(|d| d.function.name)
        .collect::<std::collections::BTreeSet<_>>();

    for name in ["pdf", "pdf_extract"] {
        assert!(defs.contains(name), "missing tool definition: {name}");
    }
}

#[tokio::test]
async fn pdf_extracts_text_from_fixture() {
    let tmp = tempfile::tempdir().unwrap();
    let workspace = tmp.path().join("ws");
    std::fs::create_dir_all(&workspace).unwrap();

    let pdf_b64 = "JVBERi0xLjQKMSAwIG9iago8PCAvVHlwZSAvQ2F0YWxvZyAvUGFnZXMgMiAwIFIgPj4KZW5kb2JqCjIgMCBvYmoKPDwgL1R5cGUgL1BhZ2VzIC9LaWRzIFszIDAgUl0gL0NvdW50IDEgPj4KZW5kb2JqCjMgMCBvYmoKPDwgL1R5cGUgL1BhZ2UgL1BhcmVudCAyIDAgUiAvTWVkaWFCb3ggWzAgMCA2MTIgNzkyXSAvQ29udGVudHMgNCAwIFIgL1Jlc291cmNlcyA8PCAvRm9udCA8PCAvRjEgNSAwIFIgPj4gPj4gPj4KZW5kb2JqCjQgMCBvYmoKPDwgL0xlbmd0aCA0OCA+PgpzdHJlYW0KQlQKL0YxIDI0IFRmCjEwMCA3MDAgVGQKKEhlbGxvIFJleE9TIFBERikgVGoKRVQKZW5kc3RyZWFtCmVuZG9iago1IDAgb2JqCjw8IC9UeXBlIC9Gb250IC9TdWJ0eXBlIC9UeXBlMSAvQmFzZUZvbnQgL0hlbHZldGljYSA+PgplbmRvYmoKeHJlZgowIDYKMDAwMDAwMDAwMCA2NTUzNSBmIAowMDAwMDAwMDA5IDAwMDAwIG4gCjAwMDAwMDAwNTggMDAwMDAgbiAKMDAwMDAwMDExNSAwMDAwMCBuIAowMDAwMDAwMjQxIDAwMDAwIG4gCjAwMDAwMDAzMzggMDAwMDAgbiAKdHJhaWxlcgo8PCAvU2l6ZSA2IC9Sb290IDEgMCBSID4+CnN0YXJ0eHJlZgo0MDgKJSVFT0YK";
    let pdf_bytes = base64::engine::general_purpose::STANDARD
        .decode(pdf_b64)
        .unwrap();
    std::fs::write(workspace.join("fixture.pdf"), pdf_bytes).unwrap();

    let tools = Toolset::new(workspace).unwrap();
    let out = tools
        .call("pdf", r#"{ "path": "fixture.pdf" }"#)
        .await
        .unwrap();
    let v: serde_json::Value = serde_json::from_str(&out).unwrap();
    let text = v.get("text").and_then(|v| v.as_str()).unwrap_or("");
    assert!(text.contains("Hello") && text.contains("PDF"), "{text}");
    assert_eq!(v.get("path").and_then(|v| v.as_str()), Some("fixture.pdf"));
}

#[tokio::test]
async fn pdf_pages_range_selects_requested_pages() {
    let tmp = tempfile::tempdir().unwrap();
    let workspace = tmp.path().join("ws");
    std::fs::create_dir_all(&workspace).unwrap();

    let pdf_b64 = "JVBERi0xLjQKMSAwIG9iago8PCAvVHlwZSAvQ2F0YWxvZyAvUGFnZXMgMiAwIFIgPj4KZW5kb2JqCjIgMCBvYmoKPDwgL1R5cGUgL1BhZ2VzIC9LaWRzIFszIDAgUiA0IDAgUiA1IDAgUl0gL0NvdW50IDMgPj4KZW5kb2JqCjMgMCBvYmoKPDwgL1R5cGUgL1BhZ2UgL1BhcmVudCAyIDAgUiAvTWVkaWFCb3ggWzAgMCA2MTIgNzkyXSAvQ29udGVudHMgNiAwIFIgL1Jlc291cmNlcyA8PCAvRm9udCA8PCAvRjEgOSAwIFIgPj4gPj4gPj4KZW5kb2JqCjQgMCBvYmoKPDwgL1R5cGUgL1BhZ2UgL1BhcmVudCAyIDAgUiAvTWVkaWFCb3ggWzAgMCA2MTIgNzkyXSAvQ29udGVudHMgNyAwIFIgL1Jlc291cmNlcyA8PCAvRm9udCA8PCAvRjEgOSAwIFIgPj4gPj4gPj4KZW5kb2JqCjUgMCBvYmoKPDwgL1R5cGUgL1BhZ2UgL1BhcmVudCAyIDAgUiAvTWVkaWFCb3ggWzAgMCA2MTIgNzkyXSAvQ29udGVudHMgOCAwIFIgL1Jlc291cmNlcyA8PCAvRm9udCA8PCAvRjEgOSAwIFIgPj4gPj4gPj4KZW5kb2JqCjYgMCBvYmoKPDwgL0xlbmd0aCA0MSA+PgpzdHJlYW0KQlQKL0YxIDI0IFRmCjEwMCA3MDAgVGQKKFBBR0VfT05FKSBUagpFVAplbmRzdHJlYW0KZW5kb2JqCjcgMCBvYmoKPDwgL0xlbmd0aCA0MSA+PgpzdHJlYW0KQlQKL0YxIDI0IFRmCjEwMCA3MDAgVGQKKFBBR0VfVFdPKSBUagpFVAplbmRzdHJlYW0KZW5kb2JqCjggMCBvYmoKPDwgL0xlbmd0aCA0MyA+PgpzdHJlYW0KQlQKL0YxIDI0IFRmCjEwMCA3MDAgVGQKKFBBR0VfVEhSRUUpIFRqCkVUCmVuZHN0cmVhbQplbmRvYmoKOSAwIG9iago8PCAvVHlwZSAvRm9udCAvU3VidHlwZSAvVHlwZTEgL0Jhc2VGb250IC9IZWx2ZXRpY2EgPj4KZW5kb2JqCnhyZWYKMCAxMAowMDAwMDAwMDAwIDY1NTM1IGYgCjAwMDAwMDAwMDkgMDAwMDAgbiAKMDAwMDAwMDA1OCAwMDAwMCBuIAowMDAwMDAwMTI3IDAwMDAwIG4gCjAwMDAwMDAyNTMgMDAwMDAgbiAKMDAwMDAwMDM3OSAwMDAwMCBuIAowMDAwMDAwNTA1IDAwMDAwIG4gCjAwMDAwMDA1OTUgMDAwMDAgbiAKMDAwMDAwMDY4NSAwMDAwMCBuIAowMDAwMDAwNzc3IDAwMDAwIG4gCnRyYWlsZXIKPDwgL1NpemUgMTAgL1Jvb3QgMSAwIFIgPj4Kc3RhcnR4cmVmCjg0NwolJUVPRgo=";
    let pdf_bytes = base64::engine::general_purpose::STANDARD
        .decode(pdf_b64)
        .unwrap();
    std::fs::write(workspace.join("pages.pdf"), pdf_bytes).unwrap();

    let tools = Toolset::new(workspace).unwrap();
    let out = tools
        .call("pdf", r#"{ "path": "pages.pdf", "pages": "2" }"#)
        .await
        .unwrap();
    let v: serde_json::Value = serde_json::from_str(&out).unwrap();
    let text = v.get("text").and_then(|v| v.as_str()).unwrap_or("");

    assert!(text.contains("PAGE_TWO"), "{text}");
    assert!(!text.contains("PAGE_ONE"), "{text}");
    assert!(!text.contains("PAGE_THREE"), "{text}");
}
