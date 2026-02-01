use anyhow::Result;
use app::config::network_config::NetworkConfig;
use app::instruction::context::InstructionContext;
use app::instruction::manager::ContextManager;
use app::instruction::storage::ContextStorage;
use app::network::interface_detector::{InterfaceType, NetworkInterface};
use app::network::port_config::{PortBinding, PortRole};
use std::collections::HashMap;

fn create_test_config() -> NetworkConfig {
    let mut config = NetworkConfig::new();
    config.port_bindings.push(PortBinding {
        port: 3000,
        interface: NetworkInterface {
            name: "lo".to_string(),
            ip_address: "127.0.0.1".parse().unwrap(),
            interface_type: InterfaceType::Loopback,
        },
        role: PortRole::Instruction,
        enabled: true,
    });
    config
}

fn create_test_context() -> InstructionContext {
    InstructionContext::new(
        "sys".to_string(),
        "role".to_string(),
        "inst".to_string(),
        vec!["action".to_string()],
        false,
    )
    .unwrap()
}

#[test]
fn test_context_lifecycle() -> Result<()> {
    let temp_dir = tempfile::tempdir()?;
    let storage = ContextStorage::with_path(temp_dir.path().join("contexts.json"));
    let mut manager = ContextManager::new_with_storage(storage)?;
    let config = create_test_config();

    let context = create_test_context();

    manager.set_context(3000, context.clone(), &config)?;
    assert_eq!(manager.get_context(3000), Some(&context));

    let list = manager.list_contexts();
    assert_eq!(list.len(), 1);

    let mut new_context = context.clone();
    new_context.role = "updated".to_string();
    manager.set_context(3000, new_context.clone(), &config)?;
    assert_eq!(manager.get_context(3000).unwrap().role, "updated");

    manager.delete_context(3000)?;
    assert!(manager.get_context(3000).is_none());

    Ok(())
}

#[test]
fn test_invalid_port() -> Result<()> {
    let temp_dir = tempfile::tempdir()?;
    let storage = ContextStorage::with_path(temp_dir.path().join("contexts.json"));
    let mut manager = ContextManager::new_with_storage(storage)?;
    let config = create_test_config();

    let context = create_test_context();

    let result = manager.set_context(4000, context, &config);
    assert!(result.is_err());
    Ok(())
}

#[test]
fn test_storage_new() -> Result<()> {
    let temp_dir = tempfile::tempdir()?;
    std::env::set_var("CHASEAI_TEST_CONFIG_DIR", temp_dir.path());
    let storage = ContextStorage::new();
    assert!(storage.is_ok());
    std::env::remove_var("CHASEAI_TEST_CONFIG_DIR");
    Ok(())
}

#[test]
fn test_save_load_storage() -> Result<()> {
    let temp_dir = tempfile::tempdir()?;
    let config_path = temp_dir.path().join("contexts.json");
    let storage = ContextStorage::with_path(config_path.clone());

    let mut contexts = HashMap::new();
    let context = create_test_context();
    contexts.insert(3000, context);

    storage.save_all(&contexts)?;
    assert!(config_path.exists());

    let loaded = storage.load_all()?;
    assert_eq!(loaded.len(), 1);
    Ok(())
}

#[test]
fn test_valid_context_creation() {
    let ctx = InstructionContext::new(
        "sys".to_string(),
        "role".to_string(),
        "inst".to_string(),
        vec!["action".to_string()],
        false,
    );
    assert!(ctx.is_ok());
}

#[test]
fn test_empty_system_fails() {
    let ctx = InstructionContext::new(
        "".to_string(),
        "role".to_string(),
        "inst".to_string(),
        vec!["action".to_string()],
        false,
    );
    assert!(ctx.is_err());
}
