mod classify;
mod exec;
mod list;
mod managed;
mod route;
#[cfg(test)]
mod tests;

use crate::Toolset;

impl Toolset {
    pub(super) async fn call_process_tool(
        &self,
        name: &str,
        arguments_json: &str,
    ) -> anyhow::Result<String> {
        route::dispatch_process_tool(self, name, arguments_json).await
    }
}
