//! ChaseAI Core Library
//!
//! This library provides the core execution and orchestration logic for controlled AI agents.

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

pub struct App {
    pub name: String,
    pub version: String,
}

impl App {
    pub fn new() -> Self {
        Self {
            name: "ChaseAI".to_string(),
            version: version().to_string(),
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn run(&self) -> anyhow::Result<()> {
        println!("{} v{} is starting...", self.name, self.version);
        println!("System ready for controlled execution.");
        Ok(())
    }
}

pub fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to ChaseAI.", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        assert!(greet("Agent").contains("ChaseAI"));
    }
}
