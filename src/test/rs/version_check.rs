use app::config::generator::ConfigurationGenerator;
use app::config::network_config::NetworkConfig;

#[test]
fn test_environment_config() {
    let config = NetworkConfig::new();
    let json = ConfigurationGenerator::generate_json(&config).unwrap();
    let env = json["application"]["environment"].as_str().unwrap();

    if cfg!(feature = "dev") {
        assert_eq!(env, "dev");
        // Dev implies Beta, so should have 2 ports
        assert_eq!(
            config.port_bindings.len(),
            2,
            "Dev should have 2 default ports"
        );
    } else if cfg!(feature = "beta") {
        assert_eq!(env, "beta");
        assert_eq!(
            config.port_bindings.len(),
            2,
            "Beta should have 2 default ports"
        );
    } else {
        assert_eq!(env, "prod");
        assert_eq!(
            config.port_bindings.len(),
            2,
            "Prod should have instruction and verification default ports"
        );
        let binding = &config.port_bindings[1];
        assert_eq!(
            binding.role,
            app::network::port_config::PortRole::Verification
        );
        assert!(
            binding.enabled,
            "Prod Verification port must be enabled strictly"
        );
        assert_eq!(
            binding.interface.interface_type,
            app::network::interface_detector::InterfaceType::Loopback,
            "Prod must be on Loopback"
        );
        assert_eq!(
            binding.interface.ip_address.to_string(),
            "127.0.0.1",
            "Prod IP must be 127.0.0.1"
        );

        let instruction = &config.port_bindings[0];
        assert_eq!(
            instruction.role,
            app::network::port_config::PortRole::Instruction
        );
        assert!(
            !instruction.enabled,
            "Prod Instruction port defaults to disabled"
        );
        assert_eq!(
            instruction.port, 8888,
            "Prod instruction port must default to 8888"
        );
    }
}
