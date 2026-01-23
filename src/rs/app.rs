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

    #[test]
    fn test_app_initialization() {
        // This test might fail if environment issues prevent ContextManager from starting
        // But ensures we didn't break basic struct layout
        let app = App::new();
        assert_eq!(app.name, "ChaseAI");
    }
}
