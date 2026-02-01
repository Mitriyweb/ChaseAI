use app::network::interface_detector::{InterfaceType, NetworkInterface};
use app::network::port_config::{PortBinding, PortConfig, PortRole};

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
