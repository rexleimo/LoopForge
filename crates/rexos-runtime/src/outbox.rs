mod delivery;
mod dispatcher;
mod events;
mod queue;
mod store;

use rexos_memory::MemoryStore;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct OutboxDrainSummary {
    pub sent: u32,
    pub failed: u32,
}

#[derive(Debug)]
pub struct OutboxDispatcher {
    pub(super) memory: MemoryStore,
    pub(super) http: reqwest::Client,
}
