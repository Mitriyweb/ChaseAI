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
            let status_icon = if binding.enabled { "●" } else { "○" };
            let role_name = format!("{:?}", binding.role);
            let port_label = format!("{} {} • {}", status_icon, binding.port, role_name);

            let port_submenu = Submenu::new(&port_label, true);

            // 1. Toggle
            let toggle_id = format!("port:{}", binding.port);
            let toggle_label = if binding.enabled { "● Disable Port" } else { "○ Enable Port" };
            port_submenu.append(&MenuItem::with_id(toggle_id, toggle_label, true, None))?;

            port_submenu.append(&PredefinedMenuItem::separator())?;

            // 2. Roles (manual symbols for alignment)
            let inst_id = format!("role:{}:Instruction", binding.port);
            let is_inst = matches!(binding.role, crate::network::port_config::PortRole::Instruction);
            let inst_label = if is_inst { "● Role: Instruction" } else { "○ Role: Instruction" };
            port_submenu.append(&MenuItem::with_id(inst_id, inst_label, true, None))?;

            let ver_id = format!("role:{}:Verification", binding.port);
            let is_ver = matches!(binding.role, crate::network::port_config::PortRole::Verification);
            let ver_label = if is_ver { "● Role: Verification" } else { "○ Role: Verification" };
            port_submenu.append(&MenuItem::with_id(ver_id, ver_label, true, None))?;

            port_submenu.append(&PredefinedMenuItem::separator())?;

            // 3. Delete
            let remove_id = format!("remove_port:{}", binding.port);
            port_submenu.append(&MenuItem::with_id(remove_id, "✕ Remove Port", true, None))?;

            menu.append(&port_submenu)?;
        }
    }

    // Add new port button directly under the list
    let add_port = MenuItem::with_id("cmd:add_port", "Add New Port...", true, None);
    menu.append(&add_port)?;

    menu.append(&PredefinedMenuItem::separator())?;

    // 4. Download Config Button
    println!("Adding Download Config button to menu");
    let download_config = MenuItem::with_id("cmd:download_config", "Download Config", true, None);
    menu.append(&download_config)?;
    println!("Download Config button added successfully");


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
