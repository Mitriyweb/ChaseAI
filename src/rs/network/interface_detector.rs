use anyhow::Result;
use if_addrs::get_if_addrs;
use serde::{Deserialize, Serialize};
use std::net::IpAddr;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum InterfaceType {
    Loopback,
    Lan,
    Public,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NetworkInterface {
    pub name: String,
    pub ip_address: IpAddr,
    pub interface_type: InterfaceType,
}

pub struct InterfaceDetector;

impl InterfaceDetector {
    /// Returns the platform-standard loopback interface name
    pub fn default_loopback_name() -> &'static str {
        if cfg!(target_os = "macos") {
            "lo0"
        } else {
            "lo"
        }
    }

    pub fn detect_all() -> Result<Vec<NetworkInterface>> {
        let mut interfaces = Vec::new();
        let if_addrs = get_if_addrs()?;

        for if_addr in if_addrs {
            let name = if_addr.name.clone();
            let ip_address = if_addr.ip();
            let interface_type = if if_addr.is_loopback() {
                InterfaceType::Loopback
            } else if Self::is_private_ip(ip_address) {
                InterfaceType::Lan
            } else {
                InterfaceType::Public
            };

            interfaces.push(NetworkInterface {
                name,
                ip_address,
                interface_type,
            });
        }

        Ok(interfaces)
    }

    pub fn detect_loopback() -> Result<Vec<NetworkInterface>> {
        Ok(Self::detect_all()?
            .into_iter()
            .filter(|i| i.interface_type == InterfaceType::Loopback)
            .collect())
    }

    pub fn detect_lan() -> Result<Vec<NetworkInterface>> {
        Ok(Self::detect_all()?
            .into_iter()
            .filter(|i| i.interface_type == InterfaceType::Lan)
            .collect())
    }

    pub fn is_private_ip(ip: IpAddr) -> bool {
        match ip {
            IpAddr::V4(v4) => v4.is_private(),
            IpAddr::V6(_v6) => {
                // Simplified for MVP, mostly assuming LAN for common V6 addr
                // In a real system we'd check for unique local addr etc.
                true
            }
        }
    }
}
