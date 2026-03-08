use anyhow::Context;

use super::mapping::decode_chat_completion_response;
use super::retry::{
    is_retryable_reqwest_error, is_retryable_status, llm_retry_max, openai_compat_timeout,
    sleep_retry_backoff, truncate_one_line,
};
use super::types::{ChatCompletionRequest, ChatMessage};

#[derive(Debug, Clone)]
pub struct OpenAiCompatibleClient {
    base_url: String,
    api_key: Option<String>,
    http: reqwest::Client,
}

impl OpenAiCompatibleClient {
    pub fn new(base_url: String, api_key: Option<String>) -> anyhow::Result<Self> {
        let base_url = base_url.trim_end_matches('/').to_string();
        let timeout = openai_compat_timeout();
        let http = reqwest::Client::builder()
            .timeout(timeout)
            .build()
            .context("build http client")?;

        Ok(Self {
            base_url,
            api_key,
            http,
        })
    }

    pub async fn chat_completions(
        &self,
        req: ChatCompletionRequest,
    ) -> anyhow::Result<ChatMessage> {
        let url = format!("{}/chat/completions", self.base_url);
        let max_retries = llm_retry_max();

        for attempt in 0..=max_retries {
            let mut http_req = self.http.post(&url).json(&req);
            if let Some(key) = &self.api_key {
                if !key.is_empty() {
                    http_req = http_req.bearer_auth(key);
                }
            }

            let response = http_req.send().await;
            match response {
                Ok(response) => {
                    if response.status().is_success() {
                        return decode_chat_completion_response(response).await;
                    }

                    let status = response.status();
                    let body = response.text().await.unwrap_or_default();
                    let retryable = is_retryable_status(status);
                    if attempt < max_retries && retryable {
                        sleep_retry_backoff(attempt + 1).await;
                        continue;
                    }

                    anyhow::bail!(
                        "chat completion HTTP error (status {}): {}",
                        status,
                        truncate_one_line(&body, 400)
                    );
                }
                Err(err) => {
                    let retryable = is_retryable_reqwest_error(&err);
                    if attempt < max_retries && retryable {
                        sleep_retry_backoff(attempt + 1).await;
                        continue;
                    }
                    return Err(err).context("send chat completion request");
                }
            }
        }

        unreachable!("retry loop should return or bail")
    }
}
