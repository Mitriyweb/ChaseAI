use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InstructionContext {
    /// System identifier (e.g., "WinSF")
    pub system: String,

    /// Agent role (e.g., "execution-agent")
    pub role: String,

    /// Base instruction text defining agent behavior
    pub base_instruction: String,

    /// List of allowed action types
    pub allowed_actions: Vec<String>,

    /// Whether verification is required for actions
    pub verification_required: bool,
}

impl InstructionContext {
    pub fn new(
        system: String,
        role: String,
        base_instruction: String,
        allowed_actions: Vec<String>,
        verification_required: bool,
    ) -> anyhow::Result<Self> {
        let context = Self {
            system,
            role,
            base_instruction,
            allowed_actions,
            verification_required,
        };
        context.validate()?;
        Ok(context)
    }

    pub fn validate(&self) -> anyhow::Result<()> {
        if self.system.trim().is_empty() {
            return Err(anyhow::anyhow!("System identifier cannot be empty"));
        }
        if self.role.trim().is_empty() {
            return Err(anyhow::anyhow!("Agent role cannot be empty"));
        }
        if self.base_instruction.trim().is_empty() {
            return Err(anyhow::anyhow!("Base instruction cannot be empty"));
        }
        if self.allowed_actions.is_empty() {
            return Err(anyhow::anyhow!("Allowed actions list cannot be empty"));
        }

        let action_regex = Regex::new(r"^[a-z][a-z0-9-]*$").unwrap();
        for action in &self.allowed_actions {
            if !action_regex.is_match(action) {
                return Err(anyhow::anyhow!(
                    "Invalid action name '{}': must start with a lowercase letter and contain only lowercase letters, numbers, and hyphens",
                    action
                ));
            }
        }

        Ok(())
    }
}
