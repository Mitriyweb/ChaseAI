use crate::instruction::context::InstructionContext;
use crate::instruction::manager::ContextManager;
use crate::network::interface_detector::NetworkInterface;
use axum::{
    extract::{Extension, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;

pub struct InstructionServer {
    port: u16,
    interface: NetworkInterface,
    context_manager: Arc<Mutex<ContextManager>>,
    shutdown_tx: broadcast::Sender<()>,
}

impl InstructionServer {
    pub fn new(
        port: u16,
        interface: NetworkInterface,
        context_manager: Arc<Mutex<ContextManager>>,
    ) -> Self {
        let (shutdown_tx, _) = broadcast::channel(1);
        Self {
            port,
            interface,
            context_manager,
            shutdown_tx,
        }
    }

    pub async fn start(&self) -> anyhow::Result<()> {
        let app = self.router();
        let addr = SocketAddr::new(self.interface.ip_address, self.port);

        // Bind first to ensure port is available and fail fast if not
        let listener = tokio::net::TcpListener::bind(addr).await?;
        let mut shutdown_rx = self.shutdown_tx.subscribe();

        println!("Starting InstructionServer on {}", addr);

        tokio::spawn(async move {
            if let Err(e) = axum::serve(listener, app)
                .with_graceful_shutdown(async move {
                    let _ = shutdown_rx.recv().await;
                })
                .await
            {
                eprintln!("Server error: {}", e);
            }
        });

        Ok(())
    }

    pub async fn stop(&self) -> anyhow::Result<()> {
        let _ = self.shutdown_tx.send(());
        Ok(())
    }

    fn router(&self) -> Router {
        Router::new()
            .route("/context", get(get_context))
            .route("/health", get(health_check))
            .layer(Extension(self.port))
            .with_state(self.context_manager.clone())
    }
}

async fn get_context(
    State(manager): State<Arc<Mutex<ContextManager>>>,
    Extension(port): Extension<u16>,
) -> Result<Json<InstructionContext>, StatusCode> {
    // In a real high-throughput scenario, we'd want RwLock, but Mutex is fine for MVP
    let manager = manager
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some(context) = manager.get_context(port) {
        Ok(Json(context.clone()))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instruction::storage::ContextStorage;
    use crate::network::interface_detector::InterfaceType;

    #[tokio::test]
    async fn test_server_startup_and_request() {
        let port = 8085; // Use a random test port
        let interface = NetworkInterface {
            name: "lo".to_string(),
            ip_address: "127.0.0.1".parse().unwrap(),
            interface_type: InterfaceType::Loopback,
        };

        let temp_dir = tempfile::tempdir().unwrap();
        let storage = ContextStorage::with_path(temp_dir.path().join("contexts.json"));
        let manager = Arc::new(Mutex::new(
            ContextManager::new_with_storage(storage).unwrap(),
        ));

        // Setup context
        let context = InstructionContext::new(
            "test_sys".to_string(),
            "test_role".to_string(),
            "inst".to_string(),
            vec!["action".to_string()],
            false,
        )
        .unwrap();

        // Need config to save context using manager (manager validation), or just hack it:
        // We'll insert directly via manager if we can or mock the config.
        // Manager requires Config to set_context.
        // Let's create a fake config.
        let mut config = crate::config::network_config::NetworkConfig::new();
        config
            .port_bindings
            .push(crate::network::port_config::PortBinding {
                port,
                interface: interface.clone(),
                role: crate::network::port_config::PortRole::Instruction,
                enabled: true,
            });

        manager
            .lock()
            .unwrap()
            .set_context(port, context, &config)
            .unwrap();

        let server = InstructionServer::new(port, interface, manager.clone());
        server.start().await.unwrap();

        // Give it a moment to start
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        // Test Health
        let client = reqwest::Client::new();
        let resp = client
            .get(format!("http://127.0.0.1:{}/health", port))
            .send()
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        // Test Context
        let resp = client
            .get(format!("http://127.0.0.1:{}/context", port))
            .send()
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let ctx: InstructionContext = resp.json().await.unwrap();
        assert_eq!(ctx.system, "test_sys");

        // cleanup
        server.stop().await.unwrap();
    }
}
