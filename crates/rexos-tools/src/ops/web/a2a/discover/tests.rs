#[tokio::test]
async fn agent_card_url_resets_path_query_and_fragment() {
    let url = super::url::agent_card_url("https://example.com/a2a/path?debug=1#frag", false)
        .await
        .unwrap();

    assert_eq!(url.as_str(), "https://example.com/.well-known/agent.json");
}
