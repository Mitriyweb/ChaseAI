use crate::instruction::context::InstructionContext;
use anyhow::Result;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

pub struct ContextStorage {
    config_path: PathBuf,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct ContextsContainer {
    contexts: Vec<ContextEntry>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct ContextEntry {
    port: u16,
    context: InstructionContext,
}

impl ContextStorage {
    pub fn new() -> Result<Self> {
        let config_dir = get_config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not determine configuration directory"))?
            .join("chaseai");

        if !config_dir.exists() {
            fs::create_dir_all(&config_dir)?;
        }

        Ok(Self {
            config_path: config_dir.join("contexts.json"),
        })
    }

    pub fn with_path(path: PathBuf) -> Self {
        Self { config_path: path }
    }

    pub fn load_all(&self) -> Result<HashMap<u16, InstructionContext>> {
        if !self.config_path.exists() {
            return Ok(HashMap::new());
        }

        let content = fs::read_to_string(&self.config_path)?;
        let container: ContextsContainer = serde_json::from_str(&content)?;

        let mut map = HashMap::new();
        for entry in container.contexts {
            map.insert(entry.port, entry.context);
        }

        Ok(map)
    }

    pub fn save_all(&self, contexts: &HashMap<u16, InstructionContext>) -> Result<()> {
        let entries: Vec<ContextEntry> = contexts
            .iter()
            .map(|(port, context)| ContextEntry {
                port: *port,
                context: context.clone(),
            })
            .collect();

        let container = ContextsContainer { contexts: entries };
        let content = serde_json::to_string_pretty(&container)?;

        if let Some(parent) = self.config_path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }

        fs::write(&self.config_path, content)?;

        // Set permissions 600
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&self.config_path, fs::Permissions::from_mode(0o600))?;
        }

        Ok(())
    }
}

fn get_config_dir() -> Option<PathBuf> {
    if let Some(test_path) = std::env::var_os("CHASEAI_TEST_CONFIG_DIR") {
        return Some(PathBuf::from(test_path));
    }
    std::env::var_os("HOME").map(|h| {
        let mut p = PathBuf::from(h);
        p.push(".config");
        p
    })
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
