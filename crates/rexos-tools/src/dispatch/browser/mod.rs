mod capture;
mod input;
mod navigation;

use crate::Toolset;

impl Toolset {
    pub(super) async fn call_browser_tool(
        &self,
        name: &str,
        arguments_json: &str,
    ) -> anyhow::Result<String> {
        match name {
            "browser_navigate" | "browser_back" | "browser_close" => {
                navigation::dispatch(self, name, arguments_json).await
            }
            "browser_click" | "browser_type" | "browser_press_key" | "browser_scroll"
            | "browser_wait" | "browser_wait_for" | "browser_run_js" => {
                input::dispatch(self, name, arguments_json).await
            }
            "browser_read_page" | "browser_screenshot" => {
                capture::dispatch(self, name, arguments_json).await
            }
            _ => unreachable!("unexpected browser tool: {name}"),
        }
    }
}
