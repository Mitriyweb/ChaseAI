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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_context_creation() {
        let context = InstructionContext::new(
            "WinSF".to_string(),
            "execution-agent".to_string(),
            "Do work".to_string(),
            vec!["analyze".to_string(), "execute-safe".to_string()],
            true,
        );
        assert!(context.is_ok());
    }

    #[test]
    fn test_empty_system_fails() {
        let context = InstructionContext::new(
            "".to_string(),
            "role".to_string(),
            "inst".to_string(),
            vec!["action".to_string()],
            false,
        );
        assert!(context.is_err());
    }

    #[test]
    fn test_empty_role_fails() {
        let context = InstructionContext::new(
            "sys".to_string(),
            "  ".to_string(),
            "inst".to_string(),
            vec!["action".to_string()],
            false,
        );
        assert!(context.is_err());
    }

    #[test]
    fn test_empty_instruction_fails() {
        let context = InstructionContext::new(
            "sys".to_string(),
            "role".to_string(),
            "".to_string(),
            vec!["action".to_string()],
            false,
        );
        assert!(context.is_err());
    }

    #[test]
    fn test_empty_actions_fails() {
        let context = InstructionContext::new(
            "sys".to_string(),
            "role".to_string(),
            "inst".to_string(),
            vec![],
            false,
        );
        assert!(context.is_err());
    }

    #[test]
    fn test_invalid_action_format_fails() {
        let context = InstructionContext::new(
            "sys".to_string(),
            "role".to_string(),
            "inst".to_string(),
            vec!["InvalidAction".to_string()], // Uppercase not allowed
            false,
        );
        assert!(context.is_err());

        let context = InstructionContext::new(
            "sys".to_string(),
            "role".to_string(),
            "inst".to_string(),
            vec!["1action".to_string()], // Cannot start with number
            false,
        );
        assert!(context.is_err());
    }
}
