use crate::config::network_config::NetworkConfig;
use crate::instruction::context::InstructionContext;
use crate::instruction::storage::ContextStorage;
use anyhow::{anyhow, Result};
use std::collections::HashMap;

pub struct ContextManager {
    contexts: HashMap<u16, InstructionContext>,
    storage: ContextStorage,
}

impl ContextManager {
    pub fn new() -> Result<Self> {
        let storage = ContextStorage::new()?;
        let contexts = storage.load_all()?;
        Ok(Self { contexts, storage })
    }

    pub fn new_with_storage(storage: ContextStorage) -> Result<Self> {
        let contexts = storage.load_all()?;
        Ok(Self { contexts, storage })
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::network::interface_detector::{InterfaceType, NetworkInterface};
    use crate::network::port_config::{PortBinding, PortRole};

    fn create_test_config() -> NetworkConfig {
        let mut config = NetworkConfig::new();
        config.port_bindings.push(PortBinding {
            port: 3000,
            interface: NetworkInterface {
                name: "lo".to_string(),
                ip_address: "127.0.0.1".parse().unwrap(),
                interface_type: InterfaceType::Loopback,
            },
            role: PortRole::Instruction,
            enabled: true,
        });
        config
    }

    fn create_test_context() -> InstructionContext {
        InstructionContext::new(
            "sys".to_string(),
            "role".to_string(),
            "inst".to_string(),
            vec!["action".to_string()],
            false,
        )
        .unwrap()
    }

    #[test]
    fn test_context_lifecycle() -> Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let storage = ContextStorage::with_path(temp_dir.path().join("contexts.json"));
        let mut manager = ContextManager::new_with_storage(storage)?;
        let config = create_test_config();

        let context = create_test_context();

        // Create
        manager.set_context(3000, context.clone(), &config)?;
        assert_eq!(manager.get_context(3000), Some(&context));

        // List
        let list = manager.list_contexts();
        assert_eq!(list.len(), 1);

        // Update
        let mut new_context = context.clone();
        new_context.role = "updated".to_string();
        manager.set_context(3000, new_context.clone(), &config)?;
        assert_eq!(manager.get_context(3000).unwrap().role, "updated");

        // Delete
        manager.delete_context(3000)?;
        assert!(manager.get_context(3000).is_none());

        Ok(())
    }

    #[test]
    fn test_invalid_port() -> Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let storage = ContextStorage::with_path(temp_dir.path().join("contexts.json"));
        let mut manager = ContextManager::new_with_storage(storage)?;
        let config = create_test_config(); // Only has port 3000

        let context = create_test_context();

        // Try binding to 4000 (not in config)
        let result = manager.set_context(4000, context, &config);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not configured"));

        Ok(())
    }
}
