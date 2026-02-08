pub mod config;
pub mod instruction;
pub mod network;
pub mod server;
pub mod ui;

use anyhow::Context;
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
    pub fn new() -> anyhow::Result<Self> {
        let config = config::network_config::NetworkConfig::load()?;
        Self::new_with_config(config)
    }

    pub fn new_with_config(config: config::network_config::NetworkConfig) -> anyhow::Result<Self> {
        let runtime =
            Runtime::new().context("Failed to create Tokio runtime for ChaseAI application")?;

        let context_manager = Arc::new(Mutex::new(
            instruction::manager::ContextManager::new_with_config(&config).context(
                "Failed to initialize ContextManager: storage might be inaccessible or corrupted",
            )?,
        ));

        let server_pool = Arc::new(TokioMutex::new(server::pool::ServerPool::new(
            context_manager.clone(),
        )));

        Ok(Self {
            name: "ChaseAI".to_string(),
            version: version().to_string(),
            config,
            tray: ui::tray::TrayManager::new(),
            runtime,
            context_manager,
            server_pool,
        })
    }
}

impl App {
    pub fn run(&mut self) -> anyhow::Result<()> {
        // Set a flag or something if we want to avoid actual side effects in some environments
        // but for now we'll just let it run.
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
        if self.process_menu_event(event.id.as_ref()) {
            std::process::exit(0);
        }
    }

    pub fn process_menu_event(&mut self, id: &str) -> bool {
        println!("Processing menu event: {}", id);
        let mut changed = false;
        let mut should_exit = false;

        if id == "quit" {
            println!("Quit requested, exiting...");
            should_exit = true;
        } else if let Some(port_str) = id.strip_prefix("port:") {
            if let Ok(port) = port_str.parse::<u16>() {
                if let Some(binding) = self
                    .config
                    .port_bindings
                    .iter_mut()
                    .find(|p| p.port == port)
                {
                    binding.enabled = !binding.enabled;
                    changed = true;
                }
            }
        } else if let Some(port_str) = id.strip_prefix("remove_port:") {
            if let Ok(port) = port_str.parse::<u16>() {
                self.config.port_bindings.retain(|b| b.port != port);
                changed = true;
            }
        } else if let Some(role_change) = id.strip_prefix("role:") {
            let parts: Vec<&str> = role_change.split(':').collect();
            if parts.len() == 2 {
                if let Ok(port) = parts[0].parse::<u16>() {
                    let role_str = parts[1];
                    if let Some(binding) = self
                        .config
                        .port_bindings
                        .iter_mut()
                        .find(|b| b.port == port)
                    {
                        binding.role = match role_str {
                            "Instruction" => crate::network::port_config::PortRole::Instruction,
                            "Verification" => crate::network::port_config::PortRole::Verification,
                            _ => binding.role,
                        };
                        changed = true;
                    }
                }
            }
        } else if let Some(name) = id.strip_prefix("interface:") {
            println!("Interface change requested: {}", name);
            // ... (rest of interface code)
            if let Ok(interfaces) =
                crate::network::interface_detector::InterfaceDetector::detect_all()
            {
                if let Some(iface) = interfaces.iter().find(|i| i.name == name) {
                    self.config.default_interface = iface.interface_type.clone();
                    for binding in &mut self.config.port_bindings {
                        binding.interface = iface.clone();
                    }
                    changed = true;
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
            self.add_default_port();
            changed = true;
        } else if id == "cmd:download_config" {
            println!("Download config requested");
            self.download_config();
        } else if id == "cmd:open_repo" {
            println!("Opening GitHub repository...");
            let _ = std::process::Command::new("open")
                .arg("https://github.com/Mitriyweb/ChaseAI")
                .spawn();
        } else if let Some(mode) = id.strip_prefix("mode:") {
            println!("Verification mode change requested: {}", mode);
            self.config.verification_mode = match mode {
                "cli" => crate::config::network_config::VerificationMode::Cli,
                _ => crate::config::network_config::VerificationMode::Port,
            };
            changed = true;
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
        should_exit
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
        // 1. Update UI
        if let Err(e) = self.tray.update_menu(&self.config) {
            eprintln!("Failed to update tray: {}", e);
        }

        // 2. Update Servers
        let pool = self.server_pool.clone();
        let config_clone = self.config.clone();
        self.runtime.block_on(async {
            if let Err(e) = pool.lock().await.update(&config_clone).await {
                eprintln!("Failed to update servers: {}", e);
            }
        });

        // 3. Update Live Manifests (if they exist in root)
        self.update_live_manifests();
    }

    fn update_live_manifests(&self) {
        use crate::config::generator::ConfigurationGenerator;
        use std::fs;
        use std::path::Path;

        let manifests = [
            ("chai_config.md", "md"),
            ("chai_config.json", "json"),
            ("chai_config.yaml", "yaml"),
            ("verification-protocol.md", "agent_rule"),
        ];

        for (filename, ext) in manifests {
            if Path::new(filename).exists() {
                let content = match ext {
                    "md" => ConfigurationGenerator::generate_markdown(&self.config),
                    "json" => ConfigurationGenerator::generate_json(&self.config)
                        .map(|v| serde_json::to_string_pretty(&v).unwrap_or_default()),
                    "yaml" => ConfigurationGenerator::generate_yaml(&self.config),
                    "agent_rule" => ConfigurationGenerator::generate_agent_rule(&self.config),
                    _ => continue,
                };

                if let Ok(data) = content {
                    let _ = fs::write(filename, data);
                }
            }
        }
    }

    fn add_default_port(&mut self) {
        // Find the first port starting from 8888 that is:
        // 1. Not in our config
        // 2. Actually free on the system (at least on 127.0.0.1)
        let existing_ports: std::collections::HashSet<u16> =
            self.config.port_bindings.iter().map(|b| b.port).collect();

        let mut default_port = 8888;
        while default_port < 65535 {
            if !existing_ports.contains(&default_port)
                && std::net::TcpListener::bind(format!("127.0.0.1:{}", default_port)).is_ok()
            {
                break;
            }
            default_port += 1;
        }

        // Show dialog to get port configuration
        if let Some(port_config) = crate::ui::dialogs::show_add_port_dialog(default_port) {
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
            let interface = crate::network::interface_detector::InterfaceDetector::detect_all()
                .ok()
                .and_then(|interfaces| {
                    interfaces
                        .into_iter()
                        .find(|i| i.interface_type == self.config.default_interface)
                })
                .unwrap_or_else(|| {
                    crate::network::interface_detector::NetworkInterface {
                        name: crate::network::interface_detector::InterfaceDetector::default_loopback_name().to_string(),
                        ip_address: std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)),
                        interface_type: crate::network::interface_detector::InterfaceType::Loopback,
                    }
                });

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
        println!("=== Download Config Started ===");

        // Check if there are any ports configured
        if self.config.port_bindings.is_empty() {
            eprintln!("No ports configured. Cannot download config.");
            #[cfg(target_os = "macos")]
            {
                let _ = std::process::Command::new("osascript")
                    .arg("-e")
                    .arg("display dialog \"No ports configured. Please add at least one port first.\" buttons {\"OK\"} default button \"OK\"")
                    .output();
            }
            return;
        }

        // Show preview dialog to get user preferences
        match crate::ui::dialogs::show_download_config_dialog(&self.config) {
            Some(options) => {
                println!("Dialog returned options: {:?}", options);
                if let Err(e) = self.download_config_with_options(&options) {
                    eprintln!("Failed to download config: {}", e);
                    #[cfg(target_os = "macos")]
                    {
                        let error_msg = format!("Failed to download config: {}", e);
                        let script = format!(
                            "display dialog \"{}\" buttons {{\"OK\"}} default button \"OK\"",
                            error_msg.replace("\"", "\\\"")
                        );
                        let _ = std::process::Command::new("osascript")
                            .arg("-e")
                            .arg(&script)
                            .output();
                    }
                } else {
                    println!("✓ Configuration downloaded successfully");
                    #[cfg(target_os = "macos")]
                    {
                        let _ = std::process::Command::new("osascript")
                            .arg("-e")
                            .arg("display dialog \"Configuration downloaded successfully!\" buttons {\"OK\"} default button \"OK\"")
                            .output();
                    }
                }
            }
            None => {
                println!("Download config cancelled by user or dialog failed");
            }
        }
        println!("=== Download Config Completed ===");
    }

    fn download_config_with_options(
        &self,
        options: &crate::ui::dialogs::ConfigDownloadOptions,
    ) -> anyhow::Result<()> {
        use std::fs;

        println!("=== Starting download_config_with_options ===");
        println!("Selected ports: {:?}", options.selected_ports);
        println!("Format: {:?}", options.format);
        println!("Save path: {:?}", options.save_path);

        // Filter config to only include selected ports
        let filtered_config = config::network_config::NetworkConfig {
            port_bindings: self
                .config
                .port_bindings
                .iter()
                .filter(|b| options.selected_ports.contains(&b.port))
                .cloned()
                .collect(),
            ..self.config.clone()
        };

        // Generate configuration in the selected format
        let (content, extension) = match options.format {
            crate::ui::dialogs::ConfigFormat::Json => {
                let json =
                    config::generator::ConfigurationGenerator::generate_json(&filtered_config)?;
                let content = serde_json::to_string_pretty(&json)?;
                (content, "json")
            }
            crate::ui::dialogs::ConfigFormat::Yaml => {
                let content =
                    config::generator::ConfigurationGenerator::generate_yaml(&filtered_config)?;
                (content, "yaml")
            }
            crate::ui::dialogs::ConfigFormat::Markdown => {
                let content =
                    config::generator::ConfigurationGenerator::generate_markdown(&filtered_config)?;
                (content, "md")
            }
            crate::ui::dialogs::ConfigFormat::AgentRule => {
                let content = config::generator::ConfigurationGenerator::generate_agent_rule(
                    &filtered_config,
                )?;
                (content, "agent_rule")
            }
        };

        // Ensure directory exists
        if !options.save_path.exists() {
            fs::create_dir_all(&options.save_path)?;
        }

        // Generate filename
        let has_verification = filtered_config
            .port_bindings
            .iter()
            .any(|b| b.role == crate::network::port_config::PortRole::Verification);

        let filename = if (extension == "agent_rule" || extension == "md") && has_verification {
            "verification-protocol.md".to_string()
        } else {
            format!("chai_config.{}", extension)
        };
        let file_path = options.save_path.join(&filename);

        println!("Writing to file: {:?}", file_path);

        // Write configuration to file
        fs::write(&file_path, content)?;

        // If we generated the verification protocol, we MUST also generate the accompanying JSON config
        if filename == "verification-protocol.md" {
            println!("Generating side-car chai_config.json for verification protocol...");
            let json = config::generator::ConfigurationGenerator::generate_json(&filtered_config)?;
            let json_content = serde_json::to_string_pretty(&json)?;
            let json_path = options.save_path.join("chai_config.json");

            println!("Writing side-car config to: {:?}", json_path);
            fs::write(&json_path, json_content)?;
        }

        println!(
            "✓ Configuration downloaded successfully to: {:?}",
            file_path
        );
        println!("=== download_config_with_options completed ===");
        Ok(())
    }

    pub fn download_config_to(&self, target_dir: &std::path::Path) -> anyhow::Result<()> {
        use std::fs;

        println!("=== Starting download_config_to {:?} ===", target_dir);

        // Generate configuration as JSON
        println!("Generating configuration JSON...");
        let config_json =
            match crate::config::generator::ConfigurationGenerator::generate_json(&self.config) {
                Ok(json) => {
                    println!("Configuration generated successfully");
                    json
                }
                Err(e) => {
                    eprintln!("Failed to generate configuration: {}", e);
                    return Ok(());
                }
            };

        // Ensure directory exists
        if !target_dir.exists() {
            fs::create_dir_all(target_dir)?;
        }

        // Generate filename
        let filename = "chai_config.json";
        let file_path = target_dir.join(filename);

        println!("Writing to file: {:?}", file_path);

        // Write configuration to file
        let json_string = serde_json::to_string_pretty(&config_json)?;

        fs::write(&file_path, json_string)?;

        println!(
            "✓ Configuration downloaded successfully to: {:?}",
            file_path
        );
        println!("=== download_config completed ===");
        Ok(())
    }
}

pub fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to ChaseAI.", name)
}
