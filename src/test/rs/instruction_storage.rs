use anyhow::Result;
use app::instruction::context::InstructionContext;
use app::instruction::storage::ContextStorage;
use std::collections::HashMap;

#[test]
fn test_save_load_storage() -> Result<()> {
    let temp_dir = tempfile::tempdir()?;
    let config_path = temp_dir.path().join("contexts.json");
    let storage = ContextStorage::with_path(config_path.clone());

    let mut contexts = HashMap::new();
    let context = InstructionContext::new(
        "test".to_string(),
        "role".to_string(),
        "inst".to_string(),
        vec!["action".to_string()],
        false,
    )?;
    contexts.insert(3000, context);

    storage.save_all(&contexts)?;

    assert!(config_path.exists());

    let loaded = storage.load_all()?;
    assert_eq!(loaded.len(), 1);
    assert_eq!(loaded.get(&3000).unwrap().system, "test");

    Ok(())
}
