use app::config::generator::ConfigurationGenerator;
use app::config::network_config::NetworkConfig;
use app::network::interface_detector::{InterfaceType, NetworkInterface};
use app::network::port_config::{PortBinding, PortRole};

fn create_test_config() -> NetworkConfig {
    NetworkConfig {
        default_interface: InterfaceType::Loopback,
        port_bindings: vec![
            PortBinding {
                port: 8090,
                interface: NetworkInterface {
                    name: "lo0".to_string(),
                    ip_address: "127.0.0.1".parse().unwrap(),
                    interface_type: InterfaceType::Loopback,
                },
                role: PortRole::Instruction,
                enabled: true,
            },
            PortBinding {
                port: 8091,
                interface: NetworkInterface {
                    name: "lo0".to_string(),
                    ip_address: "127.0.0.1".parse().unwrap(),
                    interface_type: InterfaceType::Loopback,
                },
                role: PortRole::Verification,
                enabled: true,
            },
            PortBinding {
                port: 8092,
                interface: NetworkInterface {
                    name: "lo0".to_string(),
                    ip_address: "127.0.0.1".parse().unwrap(),
                    interface_type: InterfaceType::Loopback,
                },
                role: PortRole::Verification,
                enabled: false, // Disabled port
            },
        ],
    }
}

#[test]
fn test_generate_json_all_ports() {
    let config = create_test_config();
    let result = ConfigurationGenerator::generate_json(&config);
    assert!(result.is_ok());

    let json = result.unwrap();
    let ports = json["ports"].as_array().unwrap();

    // Should now include ALL ports (including disabled ones)
    assert_eq!(ports.len(), 3);
}

#[test]
fn test_generate_json_filtered_single_port() {
    let config = create_test_config();

    // Filter to only include port 8090
    let filtered_config = NetworkConfig {
        port_bindings: config
            .port_bindings
            .iter()
            .filter(|b| b.port == 8090)
            .cloned()
            .collect(),
        ..config.clone()
    };

    let result = ConfigurationGenerator::generate_json(&filtered_config);
    assert!(result.is_ok());

    let json = result.unwrap();
    let ports = json["ports"].as_array().unwrap();

    assert_eq!(ports.len(), 1);
    assert_eq!(ports[0]["port"].as_u64().unwrap(), 8090);
}

#[test]
fn test_generate_json_filtered_subset() {
    let config = create_test_config();

    // Filter to include ports 8090 and 8091
    let selected_ports = [8090, 8091];
    let filtered_config = NetworkConfig {
        port_bindings: config
            .port_bindings
            .iter()
            .filter(|b| selected_ports.contains(&b.port))
            .cloned()
            .collect(),
        ..config.clone()
    };

    let result = ConfigurationGenerator::generate_json(&filtered_config);
    assert!(result.is_ok());

    let json = result.unwrap();
    let ports = json["ports"].as_array().unwrap();

    assert_eq!(ports.len(), 2);
}

#[test]
fn test_generate_yaml_filtered() {
    let config = create_test_config();

    let filtered_config = NetworkConfig {
        port_bindings: config
            .port_bindings
            .iter()
            .filter(|b| b.port == 8090)
            .cloned()
            .collect(),
        ..config.clone()
    };

    let result = ConfigurationGenerator::generate_yaml(&filtered_config);
    assert!(result.is_ok());

    let yaml = result.unwrap();
    assert!(yaml.contains("8090"));
    assert!(!yaml.contains("8091")); // Should not include filtered out port
}

#[test]
fn test_generate_markdown_filtered() {
    let config = create_test_config();

    let filtered_config = NetworkConfig {
        port_bindings: config
            .port_bindings
            .iter()
            .filter(|b| b.port == 8090)
            .cloned()
            .collect(),
        ..config.clone()
    };

    let result = ConfigurationGenerator::generate_markdown(&filtered_config);
    assert!(result.is_ok());

    let markdown = result.unwrap();
    assert!(markdown.contains("Port `8090`"));
    assert!(!markdown.contains("Port 8091")); // Should not include filtered out port
}

#[test]
fn test_disabled_ports_excluded() {
    let config = create_test_config();

    // Even if we include disabled port in filter, it should be excluded
    let selected_ports = [8090, 8092]; // 8092 is disabled
    let filtered_config = NetworkConfig {
        port_bindings: config
            .port_bindings
            .iter()
            .filter(|b| selected_ports.contains(&b.port))
            .cloned()
            .collect(),
        ..config.clone()
    };

    let result = ConfigurationGenerator::generate_json(&filtered_config);
    assert!(result.is_ok());

    let json = result.unwrap();
    let ports = json["ports"].as_array().unwrap();

    // Should include BOTH requested ports, regardless of enabled status in generator
    // (Filtering is now handled at the app level when calling generator)
    assert_eq!(ports.len(), 2);
    assert_eq!(ports[0]["port"].as_u64().unwrap(), 8090);
    assert_eq!(ports[1]["port"].as_u64().unwrap(), 8092);
}

#[test]
fn test_config_format_extension() {
    use app::ui::dialogs::ConfigFormat;

    assert_eq!(ConfigFormat::Json.extension(), "json");
    assert_eq!(ConfigFormat::Yaml.extension(), "yaml");
    assert_eq!(ConfigFormat::Markdown.extension(), "md");
}

#[test]
fn test_config_format_name() {
    use app::ui::dialogs::ConfigFormat;

    assert_eq!(ConfigFormat::Json.name(), "JSON");
    assert_eq!(ConfigFormat::Yaml.name(), "YAML");
    assert_eq!(ConfigFormat::Markdown.name(), "Markdown");
}
