use anyhow::Result;
use app::config::generator::ConfigurationGenerator;
use app::config::network_config::NetworkConfig;

#[test]
fn test_generate_json() {
    let config = NetworkConfig::new();
    let result = ConfigurationGenerator::generate_json(&config);
    assert!(result.is_ok());
    let json = result.unwrap();
    assert!(json["version"].is_string());
}

#[test]
fn test_generate_yaml() {
    let config = NetworkConfig::new();
    let result = ConfigurationGenerator::generate_yaml(&config);
    assert!(result.is_ok());
}

#[test]
fn test_generate_markdown() {
    let config = NetworkConfig::new();
    let result = ConfigurationGenerator::generate_markdown(&config);
    assert!(result.is_ok());
}

#[test]
fn test_json_contains_required_fields() {
    let config = NetworkConfig::new();
    let json = ConfigurationGenerator::generate_json(&config).unwrap();
    assert!(json["version"].is_string());
    assert!(json["application"]["name"].is_string());
}

#[test]
fn test_ports_include_endpoints() {
    let config = NetworkConfig::new();
    let json = ConfigurationGenerator::generate_json(&config).unwrap();
    let ports = json["ports"].as_array().unwrap();
    for port in ports {
        assert!(port["endpoints"].is_array());
    }
}

#[test]
fn test_default_config() {
    let config = NetworkConfig::default();
    assert_eq!(
        config.default_interface,
        app::network::interface_detector::InterfaceType::Loopback
    );

    #[cfg(not(feature = "beta"))]
    assert_eq!(config.port_bindings.len(), 1);

    #[cfg(feature = "beta")]
    assert_eq!(config.port_bindings.len(), 2);
}

#[test]
fn test_real_save_load() -> Result<()> {
    let temp_dir = tempfile::tempdir()?;
    std::env::set_var("CHASEAI_TEST_CONFIG_DIR", temp_dir.path());

    let mut config = NetworkConfig::new();
    config.port_bindings[0].port = 7777;

    config.save()?;

    let loaded = NetworkConfig::load()?;
    assert_eq!(loaded.port_bindings[0].port, 7777);

    std::env::set_var(
        "CHASEAI_TEST_CONFIG_DIR",
        temp_dir.path().join("non_existent"),
    );
    let default_config = NetworkConfig::load()?;

    #[cfg(feature = "beta")]
    assert_eq!(default_config.port_bindings[0].port, 8888);

    #[cfg(not(feature = "beta"))]
    assert_eq!(default_config.port_bindings[0].port, 9999);

    std::env::remove_var("CHASEAI_TEST_CONFIG_DIR");
    Ok(())
}

#[test]
fn test_verification_mode_documentation() {
    let mut config = NetworkConfig::new();

    // In Prod, Verification is at index 0. In Beta, it's at index 1.
    #[cfg(not(feature = "beta"))]
    let binding_idx = 0;
    #[cfg(feature = "beta")]
    let binding_idx = 1;

    config.port_bindings[binding_idx].enabled = true;

    // Test Port mode
    config.verification_mode = app::config::network_config::VerificationMode::Port;
    let md = ConfigurationGenerator::generate_markdown(&config).unwrap();
    assert!(md.contains("POST /verify"));
    assert!(md.contains("http://127.0.0.1:9999"));
    let rule = ConfigurationGenerator::generate_agent_rule(&config).unwrap();
    assert!(rule.contains("POST http://127.0.0.1:9999/verify"));

    // Test CLI mode
    config.verification_mode = app::config::network_config::VerificationMode::Cli;
    let md = ConfigurationGenerator::generate_markdown(&config).unwrap();
    assert!(md.contains("chase --verification"));
    let rule = ConfigurationGenerator::generate_agent_rule(&config).unwrap();
    assert!(rule.contains("chase --verification"));
}
