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
        use crate::ui::dialogs::show_add_port_dialog;
        let result = show_add_port_dialog(8888);
        assert!(result.is_none());
    }
}
