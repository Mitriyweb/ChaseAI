use crate::network::interface_detector::NetworkInterface;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PortRole {
    Instruction,
    Verification,
    Workflow,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PortBinding {
    pub port: u16,
    pub interface: NetworkInterface,
    pub role: PortRole,
    pub enabled: bool,
}

pub struct PortConfig {
    bindings: HashMap<u16, PortBinding>,
}

impl PortConfig {
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
        }
    }

    pub fn add_binding(&mut self, binding: PortBinding) -> Result<()> {
        self.validate_port(binding.port)?;
        if self.bindings.contains_key(&binding.port) {
            return Err(anyhow!("Port {} is already bound", binding.port));
        }
        self.bindings.insert(binding.port, binding);
        Ok(())
    }

    pub fn remove_binding(&mut self, port: u16) -> Result<()> {
        if self.bindings.remove(&port).is_none() {
            return Err(anyhow!("No binding found for port {}", port));
        }
        Ok(())
    }

    pub fn get_binding(&self, port: u16) -> Option<&PortBinding> {
        self.bindings.get(&port)
    }

    pub fn list_bindings(&self) -> Vec<&PortBinding> {
        self.bindings.values().collect()
    }

    pub fn validate_port(&self, port: u16) -> Result<()> {
        if port < 1024 {
            return Err(anyhow!("Ports below 1024 are reserved for system services"));
        }
        Ok(())
    }
}

impl Default for PortConfig {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::network::interface_detector::{InterfaceType, NetworkInterface};

    fn mock_interface() -> NetworkInterface {
        NetworkInterface {
            name: "lo0".to_string(),
            ip_address: "127.0.0.1".parse().unwrap(),
            interface_type: InterfaceType::Loopback,
        }
    }

    #[test]
    fn test_add_binding() {
        let mut config = PortConfig::new();
        let binding = PortBinding {
            port: 3000,
            interface: mock_interface(),
            role: PortRole::Instruction,
            enabled: true,
        };
        config.add_binding(binding).unwrap();
        assert!(config.get_binding(3000).is_some());
    }

    #[test]
    fn test_validate_privileged_port() {
        let config = PortConfig::new();
        assert!(config.validate_port(80).is_err());
        assert!(config.validate_port(3000).is_ok());
    }
}
