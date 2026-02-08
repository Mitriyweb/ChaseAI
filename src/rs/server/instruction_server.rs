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
    pub buttons: Option<Vec<String>>,
    pub session_id: Option<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct VerificationResponse {
    pub status: String,
    pub verification_id: String,
    pub message: Option<String>,
}

async fn verify_action(
    State(manager): State<Arc<Mutex<ContextManager>>>,
    Json(payload): Json<VerificationRequest>,
) -> Json<VerificationResponse> {
    println!("🚨 Verification requested for action: {}", payload.action);

    // 1. Check if we already have an active authorized session for this agent - not ready yet
    // if let Some(sid) = &payload.session_id {
    //     let mgr = manager.lock().unwrap();
    //     if let Some((expires, _allowed)) = mgr.sessions.get(sid) {
    //         if *expires > chrono::Utc::now() {
    //             // Check if the current action is within the scope of allowed actions in this session
    //             // For simplicity, if the session exists, we assume it covers the agent's work
    //             println!("✅ Action automatically approved via session: {}", sid);
    //             return Json(VerificationResponse {
    //                 status: "approved".to_string(),
    //                 verification_id: sid.clone(),
    //                 message: Some("Automatically approved via active session".to_string()),
    //             });
    //         }
    //     }
    // }

    let context_str = payload
        .context
        .as_ref()
        .map(|c| c.to_string())
        .unwrap_or_else(|| "{}".to_string());

    let buttons = payload.buttons.unwrap_or_else(|| {
        vec![
            "Reject".to_string(),
            "Approve Once".to_string(),
            "Approve Session".to_string(),
        ]
    });

    let task_id = payload
        .context
        .as_ref()
        .and_then(|c| c.get("task_id"))
        .and_then(|v| v.as_str())
        .unwrap_or("CHASE-TASK");

    // Show the UI dialog
    let (approved_idx, message) = crate::ui::dialogs::show_verification_dialog(
        &payload.action,
        &payload.reason,
        &context_str,
        &buttons,
        task_id,
    );

    let mut status = if approved_idx < buttons.len() {
        buttons[approved_idx].to_lowercase()
    } else {
        "cancelled".to_string()
    };

    let verification_id = format!("v-{}", chrono::Utc::now().timestamp());

    // 2. If user chose "Approve Session", register it in the manager
    if status.contains("session") {
        let mut mgr = manager.lock().unwrap();
        // Session valid for 1 hour
        let expires = chrono::Utc::now() + chrono::Duration::hours(1);
        mgr.sessions
            .insert(verification_id.clone(), (expires, vec![]));
        println!("🎟 Session created: {}", verification_id);
        status = "approved_session".to_string();
    } else if status.contains("approve") {
        status = "approved".to_string();
    }

    Json(VerificationResponse {
        status,
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
        "agent_rule" | "rule" => {
            let rule = ConfigurationGenerator::generate_agent_rule(&config)
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok((StatusCode::OK, rule))
        }
        _ => {
            // Default to JSON
            let json = ConfigurationGenerator::generate_json(&config)
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok((StatusCode::OK, json.to_string()))
        }
    }
}
