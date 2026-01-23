use app::config::network_config::NetworkConfig;
use app::network::interface_detector::InterfaceDetector;
use app::network::port_config::{PortBinding, PortConfig, PortRole};

#[test]
fn test_detect_configure_persist_load_flow() -> anyhow::Result<()> {
    // 1. Detect interfaces
    let interfaces = InterfaceDetector::detect_all()?;
    assert!(!interfaces.is_empty());

    // 2. Configure port binding with the first detected interface
    let mut port_config = PortConfig::new();
    let binding = PortBinding {
        port: 5000,
        interface: interfaces[0].clone(),
        role: PortRole::Instruction,
        enabled: true,
    };
    port_config.add_binding(binding.clone())?;

    // 3. Create network config and persist
    let mut network_config = NetworkConfig::new();
    network_config.default_interface = interfaces[0].interface_type.clone();
    network_config.port_bindings = port_config.list_bindings().into_iter().cloned().collect();

    // Serialize and deserialize to simulate persistence
    let toml_str = toml::to_string_pretty(&network_config)?;
    let loaded_config: NetworkConfig = toml::from_str(&toml_str)?;

    // 4. Verify loaded config
    assert_eq!(
        loaded_config.default_interface,
        interfaces[0].interface_type
    );
    assert_eq!(loaded_config.port_bindings.len(), 1);
    assert_eq!(loaded_config.port_bindings[0].port, 5000);
    assert_eq!(loaded_config.port_bindings[0].role, PortRole::Instruction);

    Ok(())
}
