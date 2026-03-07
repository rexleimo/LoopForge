use std::collections::HashMap;
use std::sync::Arc;

mod buffer;
mod entry;

pub(crate) use buffer::ProcessOutputBuffer;
pub(crate) use entry::ProcessEntry;

pub(crate) const PROCESS_MAX_PROCESSES: usize = 5;
pub(crate) const PROCESS_OUTPUT_MAX_BYTES: usize = 200_000;
pub(crate) const PROCESS_OUTPUT_HEAD_MAX_BYTES: usize = 20_000;
pub(crate) const PROCESS_OUTPUT_TAIL_MAX_BYTES: usize =
    PROCESS_OUTPUT_MAX_BYTES - PROCESS_OUTPUT_HEAD_MAX_BYTES;
pub(crate) const TOOL_OUTPUT_MIDDLE_OMISSION_MARKER: &str = "\n\n[... middle omitted ...]\n\n";

pub(crate) struct ProcessManager {
    pub(crate) processes: HashMap<String, Arc<tokio::sync::Mutex<ProcessEntry>>>,
}

impl std::fmt::Debug for ProcessManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProcessManager")
            .field("processes", &self.processes.len())
            .finish()
    }
}

impl ProcessManager {
    pub(crate) fn new() -> Self {
        Self {
            processes: HashMap::new(),
        }
    }
}
