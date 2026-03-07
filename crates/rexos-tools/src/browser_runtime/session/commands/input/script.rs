use serde_json::Value;

use super::super::super::BrowserSession;

impl BrowserSession {
    pub(crate) async fn run_js(&mut self, expression: &str) -> anyhow::Result<Value> {
        match self {
            Self::Cdp(session) => {
                let result: Value = session.run_js(expression).await?;
                let url = session.current_url().await.ok();
                Ok(serde_json::json!({
                    "result": result,
                    "url": url,
                }))
            }
            Self::Playwright(session) => {
                session
                    .send(serde_json::json!({
                        "action": "RunJs",
                        "expression": expression,
                    }))
                    .await
            }
        }
    }
}
