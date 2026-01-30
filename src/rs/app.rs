pub mod config;
pub mod instruction;
pub mod network;
pub mod server;
pub mod ui;

use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;
use tokio::sync::Mutex as TokioMutex;

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

pub struct App {
    pub name: String,
    pub version: String,
    pub config: config::network_config::NetworkConfig,
    pub tray: ui::tray::TrayManager,

    // Runtime components
    pub runtime: Runtime,
    pub context_manager: Arc<Mutex<instruction::manager::ContextManager>>,
    pub server_pool: Arc<TokioMutex<server::pool::ServerPool>>,
}

impl App {
    pub fn new() -> Self {
        let config = config::network_config::NetworkConfig::load().unwrap_or_default();
        let runtime = Runtime::new().expect("Failed to create Tokio runtime");

        let context_manager = Arc::new(Mutex::new(
            instruction::manager::ContextManager::new().unwrap_or_else(|e| {
                eprintln!("Failed to initialize ContextManager: {}", e);
                // Creating a dummy context manager might be safer than panicking if we want to be robust
                // But for now, since it depends on storage being accessible, panic is "safe" relative to undefined behavior
                panic!("Critical startup error: {}", e);
            }),
        ));

        let server_pool = Arc::new(TokioMutex::new(server::pool::ServerPool::new(
            context_manager.clone(),
        )));

        Self {
            name: "ChaseAI".to_string(),
            version: version().to_string(),
            config,
            tray: ui::tray::TrayManager::new(),
            runtime,
            context_manager,
            server_pool,
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn run(&mut self) -> anyhow::Result<()> {
        println!("{} v{} is starting...", self.name, self.version);
        println!("Current network mode: {:?}", self.config.default_interface);
        println!("Active port bindings: {}", self.config.port_bindings.len());

        // Start services
        let pool = self.server_pool.clone();
        let config_clone = self.config.clone();

        println!("Initializing instruction servers...");
        self.runtime.block_on(async {
            if let Err(e) = pool.lock().await.update(&config_clone).await {
                eprintln!("Failed to start servers: {}", e);
            }
        });

        self.tray.setup(&self.config)?;

        println!("System ready for controlled execution.");
        Ok(())
    }
    pub fn handle_menu_event(&mut self, event: tray_icon::menu::MenuEvent) {
        let id = event.id.as_ref();
        println!("Processing menu event: {}", id);
        let mut changed = false;

        if id == "quit" {
            println!("Quit requested, exiting...");
            std::process::exit(0);
        } else if let Some(name) = id.strip_prefix("interface:") {
            println!("Interface change requested: {}", name);
            // Find interface by name
            if let Ok(interfaces) =
                crate::network::interface_detector::InterfaceDetector::detect_all()
            {
                if let Some(iface) = interfaces.iter().find(|i| i.name == name) {
                    self.config.default_interface = iface.interface_type.clone();
                    // Update all bindings to use this new interface type/ip
                    for binding in &mut self.config.port_bindings {
                        binding.interface = iface.clone();
                    }
                    changed = true;
                    println!("Interface changed to: {:?}", iface.interface_type);
                }
            }
        } else if let Some(port_str) = id.strip_prefix("port:") {
            println!("Port toggle requested: {}", port_str);
            if let Ok(port) = port_str.parse::<u16>() {
                if let Some(binding) = self
                    .config
                    .port_bindings
                    .iter_mut()
                    .find(|p| p.port == port)
                {
                    binding.enabled = !binding.enabled;
                    changed = true;
                    println!("Port {} toggled to: {}", port, binding.enabled);
                }
            }
        } else if id == "cmd:enable_all" {
            println!("Enable all services requested");
            for binding in &mut self.config.port_bindings {
                binding.enabled = true;
            }
            changed = true;
        } else if id == "cmd:disable_all" {
            println!("Disable all services requested");
            for binding in &mut self.config.port_bindings {
                binding.enabled = false;
            }
            changed = true;
        } else if id == "cmd:add_port" {
            println!("Add port requested");
            // For now, add a simple port (in future, this could open a dialog)
            self.add_default_port();
            changed = true;
        } else if let Some(port_str) = id.strip_prefix("remove_port:") {
            println!("Remove port requested: {}", port_str);
            if let Ok(port) = port_str.parse::<u16>() {
                self.config.port_bindings.retain(|b| b.port != port);
                changed = true;
                println!("Port {} removed", port);
            }
        } else if let Some(role_change) = id.strip_prefix("role:") {
            // Format: "role:PORT:ROLE"
            let parts: Vec<&str> = role_change.split(':').collect();
            if parts.len() == 2 {
                if let Ok(port) = parts[0].parse::<u16>() {
                    let role_str = parts[1];
                    println!("Change role requested for port {}: {}", port, role_str);

                    if let Some(binding) = self
                        .config
                        .port_bindings
                        .iter_mut()
                        .find(|b| b.port == port)
                    {
                        let new_role = match role_str {
                            "Instruction" => crate::network::port_config::PortRole::Instruction,
                            "Verification" => crate::network::port_config::PortRole::Verification,
                            _ => {
                                println!("Unknown role: {}", role_str);
                                return;
                            }
                        };

                        binding.role = new_role;
                        changed = true;
                        println!("Port {} role changed to: {:?}", port, new_role);
                    }
                }
            }
        } else if id == "cmd:download_config" {
            println!("Download config requested");
            self.download_config();
        } else {
            println!("Unknown menu event: {}", id);
        }

        if changed {
            println!("Configuration changed, saving and refreshing...");
            // 1. Save config
            if let Err(e) = self.config.save() {
                eprintln!("Failed to save config: {}", e);
            }
            self.refresh_ui_and_servers();
        }
    }

    pub fn reload_config(&mut self) {
        println!("Reloading configuration due to external change...");
        if let Ok(new_config) = crate::config::network_config::NetworkConfig::load() {
            self.config = new_config;
            self.refresh_ui_and_servers();
        } else {
            eprintln!("Failed to reload config");
        }
    }

    fn refresh_ui_and_servers(&mut self) {
        // 2. Update UI
        if let Err(e) = self.tray.update_menu(&self.config) {
            eprintln!("Failed to update tray: {}", e);
        }

        // 3. Update Servers
        let pool = self.server_pool.clone();
        let config_clone = self.config.clone();
        self.runtime.block_on(async {
            if let Err(e) = pool.lock().await.update(&config_clone).await {
                eprintln!("Failed to update servers: {}", e);
            }
        });
    }

    fn add_default_port(&mut self) {
        // Show dialog to get port configuration
        if let Some(port_config) = crate::ui::dialogs::show_add_port_dialog() {
            // Check if port already exists
            if self
                .config
                .port_bindings
                .iter()
                .any(|b| b.port == port_config.port)
            {
                eprintln!("Port {} already exists", port_config.port);
                return;
            }

            // Get current interface or use loopback
            let interface = if let Ok(interfaces) =
                crate::network::interface_detector::InterfaceDetector::detect_all()
            {
                interfaces
                    .into_iter()
                    .find(|i| i.interface_type == self.config.default_interface)
                    .unwrap_or_else(|| crate::network::interface_detector::NetworkInterface {
                        name: "lo0".to_string(),
                        ip_address: "127.0.0.1".parse().unwrap(),
                        interface_type: crate::network::interface_detector::InterfaceType::Loopback,
                    })
            } else {
                crate::network::interface_detector::NetworkInterface {
                    name: "lo0".to_string(),
                    ip_address: "127.0.0.1".parse().unwrap(),
                    interface_type: crate::network::interface_detector::InterfaceType::Loopback,
                }
            };

            let new_binding = crate::network::port_config::PortBinding {
                port: port_config.port,
                interface,
                role: port_config.role,
                enabled: port_config.enabled,
            };

            self.config.port_bindings.push(new_binding);
            println!(
                "Added new port: {} with role {:?}, enabled: {}",
                port_config.port, port_config.role, port_config.enabled
            );
        } else {
            println!("Add port cancelled");
        }
    }

    fn download_config(&self) {
        use std::fs;
        use chrono::Local;

        println!("=== Starting download_config ===");

        // Generate configuration as JSON
        println!("Generating configuration JSON...");
        let config_json = match crate::config::generator::ConfigurationGenerator::generate_json(&self.config) {
            Ok(json) => {
                println!("Configuration generated successfully");
                json
            },
            Err(e) => {
                eprintln!("Failed to generate configuration: {}", e);
                return;
            }
        };

        // Create Downloads directory path
        println!("Determining Downloads directory...");
        let downloads_dir = if let Some(home) = std::env::var_os("HOME") {
            let mut path = std::path::PathBuf::from(home);
            path.push("Downloads");
            path
        } else {
            eprintln!("Could not determine Downloads directory");
            return;
        };

        println!("Downloads directory: {:?}", downloads_dir);

        // Ensure Downloads directory exists
        if let Err(e) = fs::create_dir_all(&downloads_dir) {
            eprintln!("Failed to create Downloads directory: {}", e);
            return;
        }

        // Generate timestamped filename
        let timestamp = Local::now().format("%Y%m%d_%H%M%S");
        let filename = format!("chaseai_config_{}.json", timestamp);
        let file_path = downloads_dir.join(&filename);

        println!("Writing to file: {:?}", file_path);

        // Write configuration to file
        let json_string = match serde_json::to_string_pretty(&config_json) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Failed to serialize configuration: {}", e);
                return;
            }
        };

        if let Err(e) = fs::write(&file_path, json_string) {
            eprintln!("Failed to write configuration file: {}", e);
            return;
        }

        println!("✓ Configuration downloaded successfully to: {:?}", file_path);
        println!("=== download_config completed ===");
    }
}

pub fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to ChaseAI.", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        assert!(greet("Agent").contains("ChaseAI"));
    }

    #[test]
    fn test_app_initialization() {
        // This test might fail if environment issues prevent ContextManager from starting
        // But ensures we didn't break basic struct layout
        let app = App::new();
        assert_eq!(app.name, "ChaseAI");
    }
}
