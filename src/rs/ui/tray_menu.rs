use crate::config::network_config::NetworkConfig;
use crate::network::interface_detector::InterfaceDetector;
use tray_icon::menu::{CheckMenuItem, Menu, MenuItem, PredefinedMenuItem, Submenu};

pub struct MenuIds {
    pub quit: String,
    pub enable_all: String,
    pub disable_all: String,
}

pub fn build_menu(config: &NetworkConfig) -> anyhow::Result<Menu> {
    let menu = Menu::new();

    // 1. Status Section
    let status_text = if config.port_bindings.iter().any(|p| p.enabled) {
        "ChaseAI: Running"
    } else {
        "ChaseAI: Stopped"
    };
    menu.append(&MenuItem::new(status_text, true, None))?;

    // Show current IP
    let ip = config
        .port_bindings
        .first()
        .map(|b| b.interface.ip_address.to_string())
        .unwrap_or_else(|| "127.0.0.1".to_string());

    let interface_type = &config.default_interface;
    let start_text = format!("IP: {} ({:?})", ip, interface_type);
    menu.append(&MenuItem::new(&start_text, true, None))?;

    menu.append(&PredefinedMenuItem::separator())?;

    // 2. Interface Selection Submenu
    let interface_menu = Submenu::new("Select Interface", true);
    let interfaces = InterfaceDetector::detect_all()?;

    for interface in interfaces {
        let is_selected = interface.interface_type == config.default_interface;
        // ID format: "interface:{index}" or just use auto-generated and map later?
        // For simplicity in MVP, we might need a way to map IDs back to actions.
        // tray-icon 0.14 uses MenuEvent which gives us an ID.
        // We can construct IDs like "interface:<name>".

        let id = format!("interface:{}", interface.name);
        let label = format!("{} ({})", interface.name, interface.ip_address);

        // CheckMenuItem is better for selection
        let item = CheckMenuItem::with_id(id, &label, true, is_selected, None);
        interface_menu.append(&item)?;
    }
    menu.append(&interface_menu)?;

    // 3. Ports Configuration
    menu.append(&PredefinedMenuItem::separator())?;

    if config.port_bindings.is_empty() {
        menu.append(&MenuItem::new("No ports configured", true, None))?;
    } else {
        menu.append(&MenuItem::new("Ports:", true, None))?;

        for binding in &config.port_bindings {
            // Show status indicator in the port label
            let status_icon = if binding.enabled { "●" } else { "○" };
            let role_name = match binding.role {
                crate::network::port_config::PortRole::Instruction => "Instruction",
                crate::network::port_config::PortRole::Verification => "Verification",
                crate::network::port_config::PortRole::Workflow => "Workflow",
            };
            let port_label = format!("{} {} • {}", status_icon, binding.port, role_name);

            let port_submenu = Submenu::new(&port_label, true);

            // Toggle enabled/disabled
            let toggle_id = format!("port:{}", binding.port);
            let toggle_label = if binding.enabled { "Disable" } else { "Enable" };
            let toggle_item = MenuItem::with_id(toggle_id, toggle_label, true, None);
            port_submenu.append(&toggle_item)?;

            port_submenu.append(&PredefinedMenuItem::separator())?;

            // Change role submenu
            let role_menu = Submenu::new("Change Role", true);

            let instruction_id = format!("role:{}:Instruction", binding.port);
            let instruction_item = CheckMenuItem::with_id(
                instruction_id,
                "Instruction",
                true,
                matches!(
                    binding.role,
                    crate::network::port_config::PortRole::Instruction
                ),
                None,
            );
            role_menu.append(&instruction_item)?;

            let verification_id = format!("role:{}:Verification", binding.port);
            let verification_item = CheckMenuItem::with_id(
                verification_id,
                "Verification",
                true,
                matches!(
                    binding.role,
                    crate::network::port_config::PortRole::Verification
                ),
                None,
            );
            role_menu.append(&verification_item)?;

            port_submenu.append(&role_menu)?;

            menu.append(&port_submenu)?;
        }
    }

    menu.append(&PredefinedMenuItem::separator())?;

    // Add port management submenu
    let port_mgmt = Submenu::new("Manage Ports", true);

    let add_port = MenuItem::with_id("cmd:add_port", "Add New Port...", true, None);
    port_mgmt.append(&add_port)?;

    if !config.port_bindings.is_empty() {
        port_mgmt.append(&PredefinedMenuItem::separator())?;

        for binding in &config.port_bindings {
            let id = format!("remove_port:{}", binding.port);
            let label = format!("Remove Port {}", binding.port);
            let item = MenuItem::with_id(id, &label, true, None);
            port_mgmt.append(&item)?;
        }
    }

    menu.append(&port_mgmt)?;

    menu.append(&PredefinedMenuItem::separator())?;

    // 4. Global Commands
    let enable_all = MenuItem::with_id("cmd:enable_all", "Enable All Services", true, None);
    let disable_all = MenuItem::with_id("cmd:disable_all", "Disable All Services", true, None);

    menu.append(&enable_all)?;
    menu.append(&disable_all)?;

    menu.append(&PredefinedMenuItem::separator())?;

    // Custom quit menu item with lowercase
    let quit_item = MenuItem::with_id("quit", "Quit ChaseAI", true, None);
    menu.append(&quit_item)?;

    Ok(menu)
}
