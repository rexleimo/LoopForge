mod request;
mod response;
mod status;
#[cfg(test)]
mod tests;
mod validate;

use crate::Toolset;

impl Toolset {
    pub(crate) async fn a2a_send(
        &self,
        agent_url: &str,
        message: &str,
        session_id: Option<&str>,
        allow_private: bool,
    ) -> anyhow::Result<String> {
        validate::ensure_non_empty_message(message)?;

        let url = validate::parse_agent_url(agent_url)?;
        super::super::ensure_remote_url_allowed(
            &url,
            allow_private,
            "a2a_send",
            "POST",
            &self.security,
        )
        .await?;

        let value =
            status::send_request(&self.http, url, request::send_request(message, session_id))
                .await?;
        response::parse_send_response(value)
    }
}
