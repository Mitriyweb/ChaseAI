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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::network_config::NetworkConfig;
    use crate::network::interface_detector::{InterfaceType, NetworkInterface};
    use crate::network::port_config::{PortBinding, PortRole};

    async fn create_test_pool() -> (ServerPool, Arc<Mutex<ContextManager>>) {
        let temp_dir = tempfile::tempdir().unwrap();
        let storage = crate::instruction::storage::ContextStorage::with_path(
            temp_dir.path().join("contexts.json"),
        );
        let context_manager = Arc::new(Mutex::new(
            ContextManager::new_with_storage(storage).unwrap(),
        ));
        (ServerPool::new(context_manager.clone()), context_manager)
    }

    fn create_test_config(port: u16, enabled: bool) -> NetworkConfig {
        let mut config = NetworkConfig::new();
        config.port_bindings.clear();
        config.port_bindings.push(PortBinding {
            port,
            interface: NetworkInterface {
                name: "lo".to_string(),
                ip_address: "127.0.0.1".parse().unwrap(),
                interface_type: InterfaceType::Loopback,
            },
            role: PortRole::Instruction,
            enabled,
        });
        config
    }

    #[tokio::test]
    async fn test_pool_update_start_stop() {
        let (mut pool, _) = create_test_pool().await;

        // Start server
        let config_on = create_test_config(3001, true);
        pool.update(&config_on).await.unwrap();
        assert_eq!(pool.servers.len(), 1);
        assert!(pool.servers.contains_key(&3001));

        // Stop server
        let config_off = create_test_config(3001, false);
        pool.update(&config_off).await.unwrap();
        assert_eq!(pool.servers.len(), 0);
    }

    #[tokio::test]
    async fn test_pool_shutdown() {
        let (mut pool, _) = create_test_pool().await;
        let config = create_test_config(3002, true);
        pool.update(&config).await.unwrap();
        assert_eq!(pool.servers.len(), 1);

        pool.shutdown().await;
        assert_eq!(pool.servers.len(), 0);
    }
}
