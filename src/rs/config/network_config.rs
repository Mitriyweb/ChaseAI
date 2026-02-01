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
        // Create default port bindings for loopback interface
        let loopback_interface = crate::network::interface_detector::NetworkInterface {
            name: "lo0".to_string(),
            ip_address: "127.0.0.1".parse().unwrap(),
            interface_type: InterfaceType::Loopback,
        };

        let default_bindings = vec![
            PortBinding {
                port: 8888,
                interface: loopback_interface.clone(),
                role: crate::network::port_config::PortRole::Instruction,
                enabled: false, // Disabled by default for safety
            },
            PortBinding {
                port: 9999,
                interface: loopback_interface,
                role: crate::network::port_config::PortRole::Verification,
                enabled: false,
            },
        ];

        Self {
            default_interface: InterfaceType::Loopback,
            port_bindings: default_bindings,
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
        if let Some(test_path) = std::env::var_os("CHASEAI_TEST_CONFIG_DIR") {
            return Some(PathBuf::from(test_path));
        }

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
