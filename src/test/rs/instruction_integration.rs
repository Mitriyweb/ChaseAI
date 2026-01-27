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

#[tokio::test]
async fn test_multiple_instruction_servers() -> anyhow::Result<()> {
    // 1. Setup Environment
    let temp_dir = tempfile::tempdir()?;
    let context_path = temp_dir.path().join("contexts.json");

    // 2. Setup Network Config with two ports
    let mut net_config = NetworkConfig::new();
    let port1 = 8091;
    let port2 = 8092;

    net_config.port_bindings.push(PortBinding {
        port: port1,
        interface: NetworkInterface {
            name: "lo".to_string(),
            ip_address: "127.0.0.1".parse()?,
            interface_type: InterfaceType::Loopback,
        },
        role: PortRole::Instruction,
        enabled: true,
    });

    net_config.port_bindings.push(PortBinding {
        port: port2,
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

    // 4. Create and set contexts for both ports
    let context1 = InstructionContext::new(
        "Sys1".to_string(),
        "Role1".to_string(),
        "Inst1".to_string(),
        vec!["action1".to_string()],
        true,
    )?;
    let context2 = InstructionContext::new(
        "Sys2".to_string(),
        "Role2".to_string(),
        "Inst2".to_string(),
        vec!["action2".to_string()],
        true,
    )?;

    manager
        .lock()
        .unwrap()
        .set_context(port1, context1.clone(), &net_config)?;
    manager
        .lock()
        .unwrap()
        .set_context(port2, context2.clone(), &net_config)?;

    // 5. Start Server Pool
    let mut pool = ServerPool::new(manager.clone());
    pool.update(&net_config).await?;

    // Wait for servers
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // 6. Verify both via HTTP
    let client = reqwest::Client::new();

    let resp1 = client
        .get(format!("http://127.0.0.1:{}/context", port1))
        .send()
        .await?;
    assert_eq!(resp1.status(), 200);
    let received_context1: InstructionContext = resp1.json().await?;
    assert_eq!(received_context1, context1);

    let resp2 = client
        .get(format!("http://127.0.0.1:{}/context", port2))
        .send()
        .await?;
    assert_eq!(resp2.status(), 200);
    let received_context2: InstructionContext = resp2.json().await?;
    assert_eq!(received_context2, context2);

    // 7. Cleanup
    pool.shutdown().await;

    Ok(())
}

#[tokio::test]
async fn test_context_update_reflection() -> anyhow::Result<()> {
    // 1. Setup Environment
    let temp_dir = tempfile::tempdir()?;
    let context_path = temp_dir.path().join("contexts.json");

    // 2. Setup Network Config
    let mut net_config = NetworkConfig::new();
    let port = 8093;
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

    // 4. Initial Context
    let context1 = InstructionContext::new(
        "Sys1".to_string(),
        "Role1".to_string(),
        "Inst1".to_string(),
        vec!["action1".to_string()],
        true,
    )?;

    manager
        .lock()
        .unwrap()
        .set_context(port, context1.clone(), &net_config)?;

    // 5. Start Server Pool
    let mut pool = ServerPool::new(manager.clone());
    pool.update(&net_config).await?;

    // Wait for server
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // 6. Verify first context
    let client = reqwest::Client::new();
    let resp1 = client
        .get(format!("http://127.0.0.1:{}/context", port))
        .send()
        .await?;
    let received_context1: InstructionContext = resp1.json().await?;
    assert_eq!(received_context1, context1);

    // 7. Update Context
    let context2 = InstructionContext::new(
        "Sys2".to_string(),
        "Role2".to_string(),
        "Inst2".to_string(),
        vec!["action2".to_string()],
        true,
    )?;

    manager
        .lock()
        .unwrap()
        .set_context(port, context2.clone(), &net_config)?;

    // 8. Verify update is reflected
    let resp2 = client
        .get(format!("http://127.0.0.1:{}/context", port))
        .send()
        .await?;
    let received_context2: InstructionContext = resp2.json().await?;
    assert_eq!(received_context2, context2);

    // 9. Cleanup
    pool.shutdown().await;

    Ok(())
}
