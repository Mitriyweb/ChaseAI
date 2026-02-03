use crate::config::generator::ConfigurationGenerator;
use crate::config::network_config::NetworkConfig;
use crate::instruction::context::InstructionContext;
use crate::instruction::manager::ContextManager;
use crate::network::interface_detector::NetworkInterface;
use axum::{
    extract::{Extension, Query, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use serde::Deserialize;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;

#[derive(Debug, Deserialize)]
pub struct ConfigFormat {
    #[serde(default)]
    format: String,
}

pub struct InstructionServer {
    port: u16,
    interface: NetworkInterface,
    context_manager: Arc<Mutex<ContextManager>>,
    network_config: Arc<Mutex<NetworkConfig>>,
    shutdown_tx: broadcast::Sender<()>,
}

impl InstructionServer {
    pub fn new(
        port: u16,
        interface: NetworkInterface,
        context_manager: Arc<Mutex<ContextManager>>,
    ) -> Self {
        let (shutdown_tx, _) = broadcast::channel(1);
        let network_config = Arc::new(Mutex::new(NetworkConfig::new()));
        Self {
            port,
            interface,
            context_manager,
            network_config,
            shutdown_tx,
        }
    }

    pub fn with_config(
        port: u16,
        interface: NetworkInterface,
        context_manager: Arc<Mutex<ContextManager>>,
        network_config: Arc<Mutex<NetworkConfig>>,
    ) -> Self {
        let (shutdown_tx, _) = broadcast::channel(1);
        Self {
            port,
            interface,
            context_manager,
            network_config,
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
            .route("/config", get(get_config))
            .route("/health", get(health_check))
            .route("/verify", axum::routing::post(verify_action))
            .layer(Extension(self.port))
            .layer(Extension(self.network_config.clone()))
            .with_state(self.context_manager.clone())
    }
}

#[derive(Debug, Deserialize)]
pub struct VerificationRequest {
    pub action: String,
    pub reason: String,
    pub context: Option<serde_json::Value>,
}

#[derive(Debug, serde::Serialize)]
pub struct VerificationResponse {
    pub status: String,
    pub verification_id: String,
    pub message: Option<String>,
}

async fn verify_action(Json(payload): Json<VerificationRequest>) -> Json<VerificationResponse> {
    println!("🚨 Verification requested for action: {}", payload.action);

    let context_str = payload
        .context
        .as_ref()
        .map(|c| c.to_string())
        .unwrap_or_else(|| "{}".to_string());

    // 1. Show a tray notification first
    let notify_script = format!(
        "display notification \"Action: {}\" with title \"🚨 ChaseAI: Verification Needed\" subtitle \"Reason: {}\"",
        payload.action.replace("\"", "\\\""),
        payload.reason.replace("\"", "\\\"")
    );
    let _ = std::process::Command::new("osascript")
        .arg("-e")
        .arg(notify_script)
        .output();

    // 2. Show the actual UI dialog
    let (approved, message) = crate::ui::dialogs::show_verification_dialog(
        &payload.action,
        &payload.reason,
        &context_str,
    );

    let status = if approved { "approved" } else { "rejected" };
    let verification_id = format!("v-{}", chrono::Utc::now().timestamp());

    Json(VerificationResponse {
        status: status.to_string(),
        verification_id,
        message,
    })
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

async fn get_config(
    Extension(network_config): Extension<Arc<Mutex<NetworkConfig>>>,
    Query(params): Query<ConfigFormat>,
) -> Result<(StatusCode, String), StatusCode> {
    let config = network_config
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let format = params.format.to_lowercase();
    match format.as_str() {
        "yaml" => {
            let yaml = ConfigurationGenerator::generate_yaml(&config)
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok((StatusCode::OK, yaml))
        }
        "markdown" | "md" => {
            let markdown = ConfigurationGenerator::generate_markdown(&config)
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok((StatusCode::OK, markdown))
        }
        _ => {
            // Default to JSON
            let json = ConfigurationGenerator::generate_json(&config)
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok((StatusCode::OK, json.to_string()))
        }
    }
}
