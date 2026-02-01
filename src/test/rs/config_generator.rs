use app::config::generator::ConfigurationGenerator;
use app::config::network_config::NetworkConfig;
use app::network::interface_detector::{InterfaceType, NetworkInterface};
use app::network::port_config::{PortBinding, PortRole};

fn create_test_config() -> NetworkConfig {
    let interface = NetworkInterface {
        name: "lo0".to_string(),
        ip_address: "127.0.0.1".parse().unwrap(),
        interface_type: InterfaceType::Loopback,
    };

    let mut config = NetworkConfig::new();
    config.port_bindings = vec![
        PortBinding {
            port: 8090,
            interface: interface.clone(),
            role: PortRole::Instruction,
            enabled: true,
        },
        PortBinding {
            port: 9090,
            interface,
            role: PortRole::Verification,
            enabled: true,
        },
    ];

    config
}

#[test]
fn test_generate_json() {
    let config = create_test_config();
    let result = ConfigurationGenerator::generate_json(&config);
    assert!(result.is_ok());

    let json = result.unwrap();
    assert_eq!(json["version"], "1.0.0");
    assert_eq!(json["application"]["name"], "ChaseAI");
    assert!(json["ports"].is_array());
    assert!(json["endpoints"].is_object());
}

#[test]
fn test_generate_yaml() {
    let config = create_test_config();
    let result = ConfigurationGenerator::generate_yaml(&config);
    assert!(result.is_ok());

    let yaml = result.unwrap();
    assert!(yaml.contains("version: 1.0.0"));
    assert!(yaml.contains("ChaseAI"));
}

#[test]
fn test_generate_markdown() {
    let config = create_test_config();
    let result = ConfigurationGenerator::generate_markdown(&config);
    assert!(result.is_ok());

    let markdown = result.unwrap();
    assert!(markdown.contains("# ChaseAI Configuration"));
    assert!(markdown.contains("## Available Ports"));
    assert!(markdown.contains("## API Endpoints"));
}

#[test]
fn test_json_contains_required_fields() {
    let config = create_test_config();
    let json = ConfigurationGenerator::generate_json(&config).unwrap();

    assert!(json["version"].is_string());
    assert!(json["timestamp"].is_string());
    assert!(json["application"]["name"].is_string());
    assert!(json["ports"].is_array());
    assert!(json["endpoints"].is_object());
}

#[test]
fn test_ports_include_endpoints() {
    let config = create_test_config();
    let json = ConfigurationGenerator::generate_json(&config).unwrap();

    let ports = json["ports"].as_array().unwrap();
    assert!(!ports.is_empty());

    for port in ports {
        assert!(port["port"].is_number());
        assert!(port["interface"].is_object());
        assert!(port["role"].is_string());
        assert!(port["enabled"].is_boolean());
        assert!(port["endpoints"].is_array());
    }
}
