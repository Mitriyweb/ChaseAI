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
