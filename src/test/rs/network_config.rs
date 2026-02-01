use anyhow::Result;
use app::config::network_config::NetworkConfig;
use app::network::interface_detector::{InterfaceType, NetworkInterface};
use app::network::port_config::{PortBinding, PortRole};

#[test]
fn test_save_load_config() -> Result<()> {
    let mut config = NetworkConfig::new();
    // Clear default ports for this test
    config.port_bindings.clear();
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
    // Default config now includes 2 ports (8888 and 9999)
    assert_eq!(config.port_bindings.len(), 2);
    assert_eq!(config.port_bindings[0].port, 8888);
    assert_eq!(config.port_bindings[1].port, 9999);
}
