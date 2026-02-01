use crate::config::network_config::NetworkConfig;
use crate::instruction::manager::ContextManager;
use crate::server::instruction_server::InstructionServer;
use anyhow::Result;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct ServerPool {
    servers: HashMap<u16, InstructionServer>,
    context_manager: Arc<Mutex<ContextManager>>,
}

impl ServerPool {
    pub fn new(context_manager: Arc<Mutex<ContextManager>>) -> Self {
        Self {
            servers: HashMap::new(),
            context_manager,
        }
    }

    pub async fn update(&mut self, config: &NetworkConfig) -> Result<()> {
        let mut active_ports = Vec::new();

        for binding in &config.port_bindings {
            if binding.enabled {
                active_ports.push(binding.port);

                if !self.servers.contains_key(&binding.port) {
                    println!("Starting instruction server on port {}", binding.port);
                    let server = InstructionServer::new(
                        binding.port,
                        binding.interface.clone(),
                        self.context_manager.clone(),
                    );
                    if let Err(e) = server.start().await {
                        eprintln!("Failed to start server on port {}: {}", binding.port, e);
                    } else {
                        self.servers.insert(binding.port, server);
                    }
                }
            }
        }

        // Stop disabled servers
        let running_ports: Vec<u16> = self.servers.keys().cloned().collect();
        for port in running_ports {
            if !active_ports.contains(&port) {
                println!("Stopping instruction server on port {}", port);
                if let Some(server) = self.servers.remove(&port) {
                    let _ = server.stop().await;
                }
            }
        }

        Ok(())
    }

    pub async fn shutdown(&mut self) {
        for (_, server) in self.servers.drain() {
            let _ = server.stop().await;
        }
    }

    pub fn server_count(&self) -> usize {
        self.servers.len()
    }

    pub fn has_server(&self, port: u16) -> bool {
        self.servers.contains_key(&port)
    }
}
