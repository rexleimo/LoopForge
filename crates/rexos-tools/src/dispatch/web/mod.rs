mod a2a;
mod fetch;
mod location;
mod pdf;
mod search;

use crate::Toolset;

impl Toolset {
    pub(super) async fn call_web_tool(
        &self,
        name: &str,
        arguments_json: &str,
    ) -> anyhow::Result<String> {
        match name {
            "web_fetch" => fetch::dispatch(self, arguments_json).await,
            "pdf" | "pdf_extract" => pdf::dispatch(self, arguments_json).await,
            "web_search" => search::dispatch(self, arguments_json).await,
            "a2a_discover" | "a2a_send" => a2a::dispatch(self, name, arguments_json).await,
            "location_get" => location::dispatch(self, arguments_json).await,
            _ => unreachable!("unexpected web tool: {name}"),
        }
    }
}
