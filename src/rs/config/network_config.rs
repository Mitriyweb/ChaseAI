use crate::network::interface_detector::InterfaceType;
use crate::network::port_config::PortBinding;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NetworkConfig {
    pub default_interface: InterfaceType,
    pub port_bindings: Vec<PortBinding>,
}

impl NetworkConfig {
    pub fn new() -> Self {
        Self {
            default_interface: InterfaceType::Loopback,
            port_bindings: Vec::new(),
        }
    }

    pub fn load() -> Result<Self> {
        let path = Self::config_path()?;
        if !path.exists() {
            return Ok(Self::new());
        }

        let content = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read config file at {:?}", path))?;

        let config: NetworkConfig = toml::from_str(&content)
            .with_context(|| "Failed to parse network configuration TOML")?;

        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::config_path()?;

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create config directory at {:?}", parent))?;
        }

        let content = toml::to_string_pretty(self)
            .with_context(|| "Failed to serialize network configuration to TOML")?;

        fs::write(&path, content)
            .with_context(|| format!("Failed to write config file to {:?}", path))?;

        // On Unix, set permissions to 600
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&path, fs::Permissions::from_mode(0o600))?;
        }

        Ok(())
    }

    pub fn config_path() -> Result<PathBuf> {
        let mut path = dirs::config_dir().context("Could not find system config directory")?;
        path.push("chaseai");
        path.push("network.toml");
        Ok(path)
    }
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self::new()
    }
}

// Helper module for directory handling
mod dirs {
    use std::path::PathBuf;

    pub fn config_dir() -> Option<PathBuf> {
        #[cfg(target_os = "macos")]
        {
            std::env::var_os("HOME").map(|h| {
                let mut p = PathBuf::from(h);
                p.push(".config");
                p
            })
        }
        #[cfg(not(target_os = "macos"))]
        {
            // Simplified for non-macOS platforms in MVP
            std::env::var_os("HOME").map(|h| {
                let mut p = PathBuf::from(h);
                p.push(".config");
                p
            })
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::network::interface_detector::{InterfaceType, NetworkInterface};
    use crate::network::port_config::{PortBinding, PortRole};
    #[test]
    fn test_save_load_config() -> Result<()> {
        let mut config = NetworkConfig::new();
        config.default_interface = InterfaceType::Lan;
        config.port_bindings.push(PortBinding {
            port: 4000,
            interface: NetworkInterface {
                name: "test".to_string(),
                ip_address: "192.168.1.1".parse()?,
                interface_type: InterfaceType::Lan,
            },
            role: PortRole::Instruction,
            enabled: true,
        });

        // Test serialization/deserialization directly
        let toml_str = toml::to_string_pretty(&config)?;
        let loaded_config: NetworkConfig = toml::from_str(&toml_str)?;

        assert_eq!(config, loaded_config);
        assert_eq!(loaded_config.port_bindings[0].port, 4000);

        Ok(())
    }

    #[test]
    fn test_default_config() {
        let config = NetworkConfig::default();
        assert_eq!(config.default_interface, InterfaceType::Loopback);
        assert!(config.port_bindings.is_empty());
    }
}
