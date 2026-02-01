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
