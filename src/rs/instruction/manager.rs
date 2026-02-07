use crate::config::network_config::NetworkConfig;
use crate::instruction::context::InstructionContext;
use crate::instruction::storage::ContextStorage;
use anyhow::{anyhow, Result};
use std::collections::HashMap;

pub type SessionMap = HashMap<String, (chrono::DateTime<chrono::Utc>, Vec<String>)>;

pub struct ContextManager {
    contexts: HashMap<u16, InstructionContext>,
    storage: ContextStorage,
    /// Maps verification_id to (expires_at, allowed_actions)
    pub sessions: SessionMap,
}

impl ContextManager {
    pub fn new() -> Result<Self> {
        let storage = ContextStorage::new()?;
        let contexts = storage.load_all()?;
        Ok(Self {
            contexts,
            storage,
            sessions: HashMap::new(),
        })
    }

    pub fn new_with_storage(storage: ContextStorage) -> Result<Self> {
        let contexts = storage.load_all()?;
        Ok(Self {
            contexts,
            storage,
            sessions: HashMap::new(),
        })
    }

    pub fn set_context(
        &mut self,
        port: u16,
        context: InstructionContext,
        config: &NetworkConfig,
    ) -> Result<()> {
        self.validate_port(port, config)?;
        context.validate()?;

        self.contexts.insert(port, context);
        self.storage.save_all(&self.contexts)?;
        Ok(())
    }

    pub fn get_context(&self, port: u16) -> Option<&InstructionContext> {
        self.contexts.get(&port)
    }

    pub fn delete_context(&mut self, port: u16) -> Result<()> {
        if self.contexts.remove(&port).is_some() {
            self.storage.save_all(&self.contexts)?;
        }
        Ok(())
    }

    pub fn list_contexts(&self) -> Vec<(u16, &InstructionContext)> {
        self.contexts.iter().map(|(k, v)| (*k, v)).collect()
    }

    fn validate_port(&self, port: u16, config: &NetworkConfig) -> Result<()> {
        let binding = config
            .port_bindings
            .iter()
            .find(|b| b.port == port)
            .ok_or_else(|| anyhow!("Port {} is not configured in network settings", port))?;

        if !binding.enabled {
            return Err(anyhow!("Port {} is disabled", port));
        }

        Ok(())
    }
}
