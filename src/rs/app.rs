pub mod config;
pub mod network;
pub mod ui;

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

pub struct App {
    pub name: String,
    pub version: String,
    pub config: config::network_config::NetworkConfig,
    pub tray: ui::tray::TrayManager,
}

impl App {
    pub fn new() -> Self {
        let config = config::network_config::NetworkConfig::load().unwrap_or_default();
        Self {
            name: "ChaseAI".to_string(),
            version: version().to_string(),
            config,
            tray: ui::tray::TrayManager::new(),
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

        self.tray.setup()?;

        println!("System ready for controlled execution.");
        Ok(())
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
}
