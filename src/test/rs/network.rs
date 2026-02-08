use app::network::interface_detector::{InterfaceDetector, InterfaceType, NetworkInterface};
use app::network::port_config::{PortBinding, PortConfig, PortRole};

fn mock_interface() -> NetworkInterface {
    NetworkInterface {
        name: "lo0".to_string(),
        ip_address: "127.0.0.1".parse().unwrap(),
        interface_type: InterfaceType::Loopback,
    }
}

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
    let private_v4 = ["192.168.1.1", "10.0.0.1", "172.16.0.1"];
    for ip in private_v4 {
        assert!(
            InterfaceDetector::is_private_ip(ip.parse::<IpAddr>().unwrap()),
            "IP {} should be private",
            ip
        );
    }

    // V4 Public
    let public_v4 = ["8.8.8.8", "1.1.1.1", "142.250.190.46"];
    for ip in public_v4 {
        assert!(
            !InterfaceDetector::is_private_ip(ip.parse::<IpAddr>().unwrap()),
            "IP {} should be public",
            ip
        );
    }

    // V6 (currently always returns true in implementation, but let's test common ones)
    let v6_ips = ["::1", "fe80::1", "2001:db8::1"];
    for ip in v6_ips {
        assert!(
            InterfaceDetector::is_private_ip(ip.parse::<IpAddr>().unwrap()),
            "IP {} should be private (V6 current behavior)",
            ip
        );
    }
}

#[test]
fn test_detect_lan() {
    let _ = InterfaceDetector::detect_lan().unwrap();
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

#[test]
fn test_remove_binding() {
    let mut config = PortConfig::new();
    let binding = PortBinding {
        port: 3000,
        interface: mock_interface(),
        role: PortRole::Instruction,
        enabled: true,
    };
    config.add_binding(binding).unwrap();
    assert!(config.remove_binding(3000).is_ok());
    assert!(config.get_binding(3000).is_none());
    assert!(config.remove_binding(3000).is_err());
}

#[test]
fn test_list_bindings() {
    let mut config = PortConfig::new();
    let binding = PortBinding {
        port: 3000,
        interface: mock_interface(),
        role: PortRole::Instruction,
        enabled: true,
    };
    config.add_binding(binding).unwrap();
    let bindings = config.list_bindings();
    assert_eq!(bindings.len(), 1);
    assert_eq!(bindings[0].port, 3000);
}

#[test]
fn test_duplicate_binding() {
    let mut config = PortConfig::new();
    let binding = PortBinding {
        port: 3000,
        interface: mock_interface(),
        role: PortRole::Instruction,
        enabled: true,
    };
    config.add_binding(binding.clone()).unwrap();
    assert!(config.add_binding(binding).is_err());
}
