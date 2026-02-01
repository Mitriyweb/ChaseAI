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

    fn is_private_ip(ip: IpAddr) -> bool {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detection_not_empty() {
        let interfaces = InterfaceDetector::detect_all().unwrap();
        assert!(!interfaces.is_empty());
    }

    #[test]
    fn test_loopback_present() {
        let interfaces = InterfaceDetector::detect_loopback().unwrap();
        assert!(!interfaces.is_empty());
        assert!(interfaces
            .iter()
            .all(|i| i.interface_type == InterfaceType::Loopback));
    }

    #[test]
    fn test_is_private_ip() {
        use std::net::IpAddr;

        // V4 Private
        assert!(InterfaceDetector::is_private_ip(
            "192.168.1.1".parse::<IpAddr>().unwrap()
        ));
        assert!(InterfaceDetector::is_private_ip(
            "10.0.0.1".parse::<IpAddr>().unwrap()
        ));
        assert!(InterfaceDetector::is_private_ip(
            "172.16.0.1".parse::<IpAddr>().unwrap()
        ));

        // V4 Public
        assert!(!InterfaceDetector::is_private_ip(
            "8.8.8.8".parse::<IpAddr>().unwrap()
        ));

        // V6 (currently always returns true in implementation)
        assert!(InterfaceDetector::is_private_ip(
            "::1".parse::<IpAddr>().unwrap()
        ));
    }

    #[test]
    fn test_detect_lan() {
        let _ = InterfaceDetector::detect_lan().unwrap();
        // We can't guarantee there is a LAN interface in all test environments,
        // but we can ensure the call doesn't fail.
    }
}
