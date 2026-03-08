mod build;

#[cfg(test)]
mod tests;

use std::collections::BTreeMap;
use std::sync::Arc;

use crate::driver::LlmDriver;

#[derive(Clone)]
pub struct LlmRegistry {
    drivers: BTreeMap<String, Arc<dyn LlmDriver>>,
    default_models: BTreeMap<String, String>,
}

impl std::fmt::Debug for LlmRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let keys: Vec<&str> = self.drivers.keys().map(|key| key.as_str()).collect();
        f.debug_struct("LlmRegistry")
            .field("providers", &keys)
            .finish()
    }
}

impl LlmRegistry {
    pub fn driver(&self, name: &str) -> Option<Arc<dyn LlmDriver>> {
        self.drivers.get(name).cloned()
    }

    pub fn default_model(&self, provider: &str) -> Option<&str> {
        self.default_models
            .get(provider)
            .map(|model| model.as_str())
            .filter(|model| !model.trim().is_empty())
    }
}
