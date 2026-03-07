mod entry;
mod payload;
mod response;
mod send;
#[cfg(test)]
mod tests;

use crate::Toolset;

impl Toolset {
    pub(crate) async fn process_write(
        &self,
        process_id: &str,
        data: &str,
    ) -> anyhow::Result<String> {
        let entry = entry::find_process_entry(&self.processes, process_id).await?;
        let payload = payload::stdin_payload(data);
        send::write_to_process_stdin(&entry, &payload).await?;
        Ok(response::process_write_ok_output().to_string())
    }
}
