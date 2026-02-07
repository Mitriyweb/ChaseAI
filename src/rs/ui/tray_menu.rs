use crate::config::network_config::NetworkConfig;
#[cfg(feature = "beta")]
use crate::network::interface_detector::InterfaceDetector;
#[cfg(any(feature = "beta", feature = "dev"))]
use tray_icon::menu::{CheckMenuItem, Submenu};
use tray_icon::menu::{Menu, MenuItem, PredefinedMenuItem};

pub struct MenuIds {
    pub quit: String,
    pub enable_all: String,
    pub disable_all: String,
}

pub fn build_menu(config: &NetworkConfig) -> anyhow::Result<Menu> {
    let menu = Menu::new();

    // 1. Status Section
    let version = env!("CARGO_PKG_VERSION");
    let env_label = if cfg!(feature = "dev") {
        " (dev)"
    } else if cfg!(feature = "beta") {
        " (beta)"
    } else {
        ""
    };

    // Main Status Item with Version (Clickable)
    let status_text = if config.port_bindings.iter().any(|p| p.enabled) {
        format!("ChaseAI v{}{}: Running", version, env_label)
    } else {
        format!("ChaseAI v{}{}: Stopped", version, env_label)
    };
    // Make it clickable and enabled
    menu.append(&MenuItem::with_id(
        "cmd:open_repo",
        &status_text,
        true,
        None,
    ))?;

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

    // 2. Interface Selection Submenu (BETA Only)
    #[cfg(feature = "beta")]
    {
        let interface_menu = Submenu::new("Select Interface", true);
        let interfaces = InterfaceDetector::detect_all()?;

        for interface in interfaces {
            let is_selected = interface.interface_type == config.default_interface;
            let id = format!("interface:{}", interface.name);
            let label = format!("{} ({})", interface.name, interface.ip_address);
            let item = CheckMenuItem::with_id(id, &label, true, is_selected, None);
            interface_menu.append(&item)?;
        }
        menu.append(&interface_menu)?;
    }

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

            // In Prod, we just show the label, no submenu for editing unless beta
            #[cfg(not(feature = "beta"))]
            {
                menu.append(&MenuItem::new(&port_label, false, None))?;
            }

            #[cfg(feature = "beta")]
            {
                let port_submenu = Submenu::new(&port_label, true);

                // 1. Toggle
                let toggle_id = format!("port:{}", binding.port);
                let toggle_label = if binding.enabled {
                    "● Disable Port"
                } else {
                    "○ Enable Port"
                };
                port_submenu.append(&MenuItem::with_id(toggle_id, toggle_label, true, None))?;

                port_submenu.append(&PredefinedMenuItem::separator())?;

                // 2. Roles
                let inst_id = format!("role:{}:Instruction", binding.port);
                let is_inst = matches!(
                    binding.role,
                    crate::network::port_config::PortRole::Instruction
                );
                let inst_label = if is_inst {
                    "● Role: Instruction"
                } else {
                    "○ Role: Instruction"
                };
                port_submenu.append(&MenuItem::with_id(inst_id, inst_label, true, None))?;

                let ver_id = format!("role:{}:Verification", binding.port);
                let is_ver = matches!(
                    binding.role,
                    crate::network::port_config::PortRole::Verification
                );
                let ver_label = if is_ver {
                    "● Role: Verification"
                } else {
                    "○ Role: Verification"
                };
                port_submenu.append(&MenuItem::with_id(ver_id, ver_label, true, None))?;

                port_submenu.append(&PredefinedMenuItem::separator())?;

                // 3. Delete
                let remove_id = format!("remove_port:{}", binding.port);
                port_submenu.append(&MenuItem::with_id(remove_id, "✕ Remove Port", true, None))?;

                menu.append(&port_submenu)?;
            }
        }
    }

    // Add new port button (BETA Only)
    #[cfg(feature = "beta")]
    {
        let add_port = MenuItem::with_id("cmd:add_port", "Add New Port...", true, None);
        menu.append(&add_port)?;
    }

    menu.append(&PredefinedMenuItem::separator())?;

    // Verification Mode Submenu (DEV Only)
    #[cfg(feature = "dev")]
    {
        menu.append(&PredefinedMenuItem::separator())?;
        let mode_menu = Submenu::new("Verification Mode", true);
        let is_port = matches!(
            config.verification_mode,
            crate::config::network_config::VerificationMode::Port
        );
        let is_cli = matches!(
            config.verification_mode,
            crate::config::network_config::VerificationMode::Cli
        );

        mode_menu.append(&CheckMenuItem::with_id(
            "mode:port",
            "Port (HTTP)",
            true,
            is_port,
            None,
        ))?;
        mode_menu.append(&CheckMenuItem::with_id(
            "mode:cli",
            "CLI (chase --verification)",
            true,
            is_cli,
            None,
        ))?;
        menu.append(&mode_menu)?;
    }

    menu.append(&PredefinedMenuItem::separator())?;

    // 4. Download Config Button (BETA Only)
    #[cfg(feature = "beta")]
    {
        menu.append(&PredefinedMenuItem::separator())?;
        println!("Adding Download Config button to menu");
        let download_config =
            MenuItem::with_id("cmd:download_config", "Download Config", true, None);
        menu.append(&download_config)?;
        println!("Download Config button added successfully");

        // 4. Global Commands
        let enable_all = MenuItem::with_id("cmd:enable_all", "Enable All Services", true, None);
        let disable_all = MenuItem::with_id("cmd:disable_all", "Disable All Services", true, None);

        menu.append(&enable_all)?;
        menu.append(&disable_all)?;
    }

    menu.append(&PredefinedMenuItem::separator())?;

    // Custom quit menu item with lowercase
    let quit_item = MenuItem::with_id("quit", "Quit ChaseAI", true, None);
    menu.append(&quit_item)?;

    Ok(menu)
}
