use app::config::network_config::NetworkConfig;
use app::instruction::context::InstructionContext;
use app::instruction::manager::ContextManager;
use app::instruction::storage::ContextStorage;
use app::network::interface_detector::{InterfaceType, NetworkInterface};
use app::network::port_config::{PortBinding, PortRole};
use app::server::pool::ServerPool;
use std::sync::{Arc, Mutex};

#[tokio::test]
async fn test_full_instruction_flow() -> anyhow::Result<()> {
    // 1. Setup Environment
    let temp_dir = tempfile::tempdir()?;
    let context_path = temp_dir.path().join("contexts.json");

    // 2. Setup Network Config
    let mut net_config = NetworkConfig::new();
    let port = 8090; // Distinct port
    net_config.port_bindings.push(PortBinding {
        port,
        interface: NetworkInterface {
            name: "lo".to_string(),
            ip_address: "127.0.0.1".parse()?,
            interface_type: InterfaceType::Loopback,
        },
        role: PortRole::Instruction,
        enabled: true,
    });

    // 3. Setup Context Manager & Persistence
    let storage = ContextStorage::with_path(context_path);
    let manager = Arc::new(Mutex::new(ContextManager::new_with_storage(storage)?));

    // 4. Create Instruction Context
    let context = InstructionContext::new(
        "IntegrationSystem".to_string(),
        "TestRole".to_string(),
        "Be helpful".to_string(),
        vec!["test-action".to_string()],
        true,
    )?;

    manager
        .lock()
        .unwrap()
        .set_context(port, context.clone(), &net_config)?;

    // 5. Start Server Pool
    let mut pool = ServerPool::new(manager.clone());
    pool.update(&net_config).await?;

    // Wait for server
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // 6. Verify via HTTP
    let client = reqwest::Client::new();
    let resp = client
        .get(format!("http://127.0.0.1:{}/context", port))
        .send()
        .await?;

    assert_eq!(resp.status(), 200);

    let received_context: InstructionContext = resp.json().await?;
    assert_eq!(received_context, context);

    // 7. Cleanup
    pool.shutdown().await;

    Ok(())
}
