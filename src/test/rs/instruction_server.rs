use app::config::network_config::NetworkConfig;
use app::instruction::context::InstructionContext;
use app::instruction::manager::ContextManager;
use app::instruction::storage::ContextStorage;
use app::network::interface_detector::{InterfaceType, NetworkInterface};
use app::network::port_config::{PortBinding, PortRole};
use app::server::instruction_server::InstructionServer;
use axum::http::StatusCode;
use std::sync::{Arc, Mutex};

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

    let mut config = NetworkConfig::new();
    config.port_bindings.push(PortBinding {
        port,
        interface: interface.clone(),
        role: PortRole::Instruction,
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

#[tokio::test]
async fn test_config_endpoint_json() {
    let port = 8086;
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

    let mut config = NetworkConfig::new();
    config.port_bindings.push(PortBinding {
        port,
        interface: interface.clone(),
        role: PortRole::Instruction,
        enabled: true,
    });

    let network_config = Arc::new(Mutex::new(config));
    let server = InstructionServer::with_config(port, interface, manager, network_config);
    server.start().await.unwrap();

    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    let client = reqwest::Client::new();

    // Test JSON config (default)
    let resp = client
        .get(format!("http://127.0.0.1:{}/config", port))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = resp.text().await.unwrap();
    assert!(body.contains("\"version\""));
    assert!(body.contains("ChaseAI"));

    // Test YAML config
    let resp = client
        .get(format!("http://127.0.0.1:{}/config?format=yaml", port))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = resp.text().await.unwrap();
    assert!(body.contains("version:"));

    // Test Markdown config
    let resp = client
        .get(format!("http://127.0.0.1:{}/config?format=markdown", port))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = resp.text().await.unwrap();
    assert!(body.contains("# ChaseAI Configuration"));

    server.stop().await.unwrap();
}
