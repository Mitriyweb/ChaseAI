use app::version;
use app::App;

#[test]
fn test_core_library_access() {
    let response = app::greet("User");
    assert!(response.contains("Hello, User"));
}

#[test]
fn test_version_return() {
    let v = version();
    assert!(!v.is_empty());
}

#[test]
fn test_app_new() {
    let app = App::new();
    assert_eq!(app.name, "ChaseAI");
    assert!(!app.version.is_empty());
}

#[test]
fn test_app_initialization() {
    let app = App::new();
    assert_eq!(app.name, "ChaseAI");
}

#[test]
fn test_process_menu_event_quit() {
    let mut app = App::new();
    assert!(app.process_menu_event("quit"));
}

#[test]
fn test_process_menu_event_unknown() {
    let mut app = App::new();
    assert!(!app.process_menu_event("unknown_event"));
}

#[test]
fn test_process_menu_event_toggle_all() {
    let mut app = App::new();
    app.process_menu_event("cmd:enable_all");
    assert!(app.config.port_bindings.iter().all(|b| b.enabled));

    app.process_menu_event("cmd:disable_all");
    assert!(app.config.port_bindings.iter().all(|b| !b.enabled));
}

#[test]
fn test_download_config_to() {
    let app = App::new();
    let temp_dir = tempfile::tempdir().unwrap();
    assert!(app.download_config_to(temp_dir.path()).is_ok());

    let entries = std::fs::read_dir(temp_dir.path()).unwrap();
    assert_eq!(entries.count(), 1);
}

#[test]
fn test_reload_config() {
    let mut app = App::new();
    app.reload_config();
}

#[test]
fn test_process_menu_event_interface() {
    let mut app = App::new();
    app.process_menu_event("interface:lo0");
    assert_eq!(
        app.config.default_interface,
        app::network::interface_detector::InterfaceType::Loopback
    );
}

#[test]
fn test_process_menu_event_port_toggle() {
    let mut app = App::new();
    if app.config.port_bindings.is_empty() {
        app.config
            .port_bindings
            .push(app::network::port_config::PortBinding {
                port: 8888,
                interface: app::network::interface_detector::NetworkInterface {
                    name: "lo0".to_string(),
                    ip_address: "127.0.0.1".parse().unwrap(),
                    interface_type: app::network::interface_detector::InterfaceType::Loopback,
                },
                role: app::network::port_config::PortRole::Instruction,
                enabled: true,
            });
    }
    let port = app.config.port_bindings[0].port;
    let id = format!("port:{}", port);

    let initial_enabled = app.config.port_bindings[0].enabled;
    app.process_menu_event(&id);
    assert_eq!(app.config.port_bindings[0].enabled, !initial_enabled);
}

#[test]
fn test_process_menu_event_remove_port() {
    let mut app = App::new();
    if app.config.port_bindings.is_empty() {
        app.config
            .port_bindings
            .push(app::network::port_config::PortBinding {
                port: 8888,
                interface: app::network::interface_detector::NetworkInterface {
                    name: "lo0".to_string(),
                    ip_address: "127.0.0.1".parse().unwrap(),
                    interface_type: app::network::interface_detector::InterfaceType::Loopback,
                },
                role: app::network::port_config::PortRole::Instruction,
                enabled: true,
            });
    }
    let port = app.config.port_bindings[0].port;
    let id = format!("remove_port:{}", port);

    app.process_menu_event(&id);
    assert!(!app.config.port_bindings.iter().any(|b| b.port == port));
}

#[test]
fn test_process_menu_event_role_change() {
    let mut app = App::new();
    if app.config.port_bindings.is_empty() {
        app.config
            .port_bindings
            .push(app::network::port_config::PortBinding {
                port: 8888,
                interface: app::network::interface_detector::NetworkInterface {
                    name: "lo0".to_string(),
                    ip_address: "127.0.0.1".parse().unwrap(),
                    interface_type: app::network::interface_detector::InterfaceType::Loopback,
                },
                role: app::network::port_config::PortRole::Instruction,
                enabled: true,
            });
    }
    let port = app.config.port_bindings[0].port;
    let id = format!("role:{}:Verification", port);

    app.process_menu_event(&id);
    assert_eq!(
        app.config.port_bindings[0].role,
        app::network::port_config::PortRole::Verification
    );
}

#[test]
fn test_process_menu_event_download_config() {
    let mut app = App::new();
    app.process_menu_event("cmd:download_config");
}
