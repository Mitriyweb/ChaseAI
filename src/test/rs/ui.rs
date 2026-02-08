use app::config::network_config::NetworkConfig;
use app::ui::tray_menu::build_menu;

#[test]
fn test_build_menu_empty() {
    let mut config = NetworkConfig::new();
    config.port_bindings.clear();
    let menu = build_menu(&config);
    assert!(menu.is_ok());
}

#[test]
fn test_build_menu_with_ports() {
    let config = NetworkConfig::new();
    let menu = build_menu(&config);
    assert!(menu.is_ok());
}

#[test]
fn test_show_add_port_dialog_fallback() {
    #[cfg(not(target_os = "macos"))]
    {
        use app::ui::dialogs::show_add_port_dialog;
        let result = show_add_port_dialog(8888);
        assert!(result.is_none());
    }
}

#[test]
fn test_download_config_with_selected_ports() {
    use app::config::generator::ConfigurationGenerator;
    use app::config::network_config::NetworkConfig;

    let config = NetworkConfig::new();

    // Test JSON format
    let json_result = ConfigurationGenerator::generate_json(&config);
    assert!(json_result.is_ok());
    let json = json_result.unwrap();
    assert!(json["version"].is_string());
    assert!(json["ports"].is_array());

    // Test YAML format
    let yaml_result = ConfigurationGenerator::generate_yaml(&config);
    assert!(yaml_result.is_ok());
    let yaml = yaml_result.unwrap();
    assert!(!yaml.is_empty());

    // Test Markdown format
    let md_result = ConfigurationGenerator::generate_markdown(&config);
    assert!(md_result.is_ok());
    let md = md_result.unwrap();
    assert!(md.contains("ChaseAI Agent Integration Manifest"));

    // Test Agent Rule format
    let rule_result = ConfigurationGenerator::generate_agent_rule(&config);
    assert!(rule_result.is_ok());
    let rule = rule_result.unwrap();
    assert!(rule.contains("ChaseAI Verification Protocol"));
}

#[test]
fn test_config_format_consistency() {
    use app::config::generator::ConfigurationGenerator;
    use app::config::network_config::NetworkConfig;

    let config = NetworkConfig::new();

    // Generate in all formats
    let json = ConfigurationGenerator::generate_json(&config).unwrap();
    let yaml = ConfigurationGenerator::generate_yaml(&config).unwrap();
    let markdown = ConfigurationGenerator::generate_markdown(&config).unwrap();
    let agent_rule = ConfigurationGenerator::generate_agent_rule(&config).unwrap();

    // All should be non-empty
    assert!(!yaml.is_empty());
    assert!(!markdown.is_empty());
    assert!(!agent_rule.is_empty());

    // JSON should have required fields
    assert!(json["version"].is_string());
    assert!(json["application"]["name"].is_string());
    assert!(json["ports"].is_array());
}

#[test]
fn test_download_config_with_filtered_ports() {
    use app::config::generator::ConfigurationGenerator;
    use app::config::network_config::NetworkConfig;

    let mut config = NetworkConfig::new();

    // Add a second port
    if !config.port_bindings.is_empty() {
        let first_binding = config.port_bindings[0].clone();
        let mut second_binding = first_binding.clone();
        second_binding.port = 9999;
        config.port_bindings.push(second_binding);
    }

    // Generate config with all ports
    let json = ConfigurationGenerator::generate_json(&config).unwrap();
    let all_ports = json["ports"].as_array().unwrap();

    // Verify we have multiple ports
    assert!(!all_ports.is_empty());

    // Each port should have endpoints
    for port in all_ports {
        assert!(port["port"].is_number());
        assert!(port["enabled"].is_boolean());
        assert!(port["role"].is_string());
        assert!(port["endpoints"].is_array());
    }
}
