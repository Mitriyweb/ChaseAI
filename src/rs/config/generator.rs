use crate::config::network_config::NetworkConfig;
use crate::network::port_config::PortRole;
use anyhow::Result;
use serde_json::{json, Value};
use std::collections::BTreeMap;

/// Configuration generator for creating machine-readable configuration files
/// that AI agents can use to discover and integrate with ChaseAI
pub struct ConfigurationGenerator;

#[derive(Debug, Clone)]
pub struct EndpointInfo {
    pub path: String,
    pub method: String,
    pub description: String,
    pub request: Option<Value>,
    pub response: Option<Value>,
}

impl ConfigurationGenerator {
    /// Generate complete configuration as JSON
    pub fn generate_json(network_config: &NetworkConfig) -> Result<Value> {
        let endpoints = Self::build_endpoints();
        let ports = Self::build_ports(network_config);

        let config = json!({
            "version": "1.0.0",
            "timestamp": "2026-02-02T23:30:00Z",
            "application": {
                "name": "ChaseAI",
                "version": env!("CARGO_PKG_VERSION"),
                "description": "Local control and orchestration system for AI agents"
            },
            "ports": ports,
            "endpoints": endpoints,
            "documentation": {
                "getting_started": "https://github.com/chaseai/chaseai/docs/ai-integration.md",
                "api_reference": "https://github.com/chaseai/chaseai/docs/api-reference.md",
                "verification_workflow": "https://github.com/chaseai/chaseai/docs/verification-workflow.md"
            }
        });

        Ok(config)
    }

    /// Generate configuration as YAML string
    pub fn generate_yaml(network_config: &NetworkConfig) -> Result<String> {
        let json_config = Self::generate_json(network_config)?;
        let yaml_string = serde_yaml::to_string(&json_config)?;
        Ok(yaml_string)
    }

    /// Generate configuration as Markdown documentation
    pub fn generate_markdown(network_config: &NetworkConfig) -> Result<String> {
        let json_config = Self::generate_json(network_config)?;
        let mut markdown = String::new();

        markdown.push_str("# 🛠 ChaseAI Agent Integration Manifest\n\n");
        markdown.push_str("> **FOR AI AGENTS**: This file is your entry point. Read this to understand available system interfaces and required protocols.\n\n");

        markdown.push_str("## 📡 Live System Status\n\n");
        markdown.push_str(&format!(
            "- **Status**: ACTIVE\n- **Last Updated**: `{}`\n- **App Version**: `{}`\n\n",
            json_config["timestamp"].as_str().unwrap_or("unknown"),
            json_config["application"]["version"]
                .as_str()
                .unwrap_or("unknown")
        ));

        markdown.push_str("## 🔌 Available Service Ports\n\n");
        if let Some(ports) = json_config["ports"].as_array() {
            for port in ports {
                let port_num = port["port"].as_u64().unwrap_or(0);
                let interface = port["interface"]["ip_address"]
                    .as_str()
                    .unwrap_or("unknown");
                let role = port["role"].as_str().unwrap_or("unknown");
                let enabled = port["enabled"].as_bool().unwrap_or(false);

                markdown.push_str(&format!("### Port `{}`\n\n", port_num));
                markdown.push_str(&format!("- **Role**: `{}`\n", role));
                markdown.push_str(&format!(
                    "- **Base URL**: `http://{}:{}`\n",
                    interface, port_num
                ));
                markdown.push_str(&format!(
                    "- **Status**: {}\n\n",
                    if enabled {
                        "✅ Enabled"
                    } else {
                        "❌ Disabled"
                    }
                ));

                if let Some(endpoints) = port["endpoints"].as_array() {
                    markdown.push_str("**Available Endpoints**:\n\n");
                    for ep in endpoints {
                        let path = ep["path"].as_str().unwrap_or("unknown");
                        let method = ep["method"].as_str().unwrap_or("unknown");
                        let desc = ep["description"].as_str().unwrap_or("N/A");
                        markdown.push_str(&format!("- `{}` `{}` — {}\n", method, path, desc));
                    }
                    markdown.push('\n');
                }
            }
        }

        markdown.push_str("## 📖 Protocol Reference\n\n");
        if let Some(endpoints) = json_config["endpoints"].as_object() {
            for (path, endpoint) in endpoints {
                let method = endpoint["method"].as_str().unwrap_or("unknown");
                let desc = endpoint["description"].as_str().unwrap_or("N/A");

                markdown.push_str(&format!("### `{}` `{}`\n\n", method, path));
                markdown.push_str(&format!("*{}*\n\n", desc));

                if let Some(req) = endpoint["request"].as_object() {
                    markdown.push_str("**Request Format**:\n\n");
                    markdown.push_str("```json\n");
                    markdown.push_str(&serde_json::to_string_pretty(req).unwrap_or_default());
                    markdown.push_str("\n```\n\n");
                }

                if let Some(resp) = endpoint["response"].as_object() {
                    markdown.push_str("**Response Example**:\n\n");
                    markdown.push_str("```json\n");
                    markdown.push_str(&serde_json::to_string_pretty(resp).unwrap_or_default());
                    markdown.push_str("\n```\n\n");
                }
            }
        }

        markdown.push_str("---\n\n");
        markdown.push_str("## 🛠 Integration Notes\n\n");
        markdown.push_str("1. **Discovery**: Always check `/config` to refresh your understanding of active roles.\n");
        markdown.push_str("2. **Instructions**: Poll `/context` on your designated port to get up-to-date system instructions.\n");
        markdown.push_str("3. **Verification**: If your role is `Instruction`, any high-risk system changes MUST be verified via a `Verification` role port.\n\n");

        Ok(markdown)
    }

    /// Build port information from network configuration
    // Should now include ALL ports (including disabled ones)
    fn build_ports(network_config: &NetworkConfig) -> Vec<Value> {
        network_config
            .port_bindings
            .iter()
            .map(|binding| {
                json!({
                    "port": binding.port,
                    "interface": {
                        "name": binding.interface.name,
                        "ip_address": binding.interface.ip_address.to_string(),
                        "type": format!("{:?}", binding.interface.interface_type)
                    },
                    "role": format!("{:?}", binding.role),
                    "enabled": binding.enabled,
                    "endpoints": Self::get_endpoints_for_role(binding.role)
                })
            })
            .collect()
    }

    /// Build endpoint information
    fn build_endpoints() -> BTreeMap<String, Value> {
        let mut endpoints = BTreeMap::new();

        endpoints.insert(
            "/context".to_string(),
            json!({
                "method": "GET",
                "description": "Retrieve instruction context for this port",
                "response": {
                    "system": "ChaseAI-OS",
                    "role": "execution-agent",
                    "base_instruction": "Execute tasks within the provided workspace scope...",
                    "allowed_actions": ["read_file", "write_file", "run_command"],
                    "verification_required": true
                }
            }),
        );

        endpoints.insert(
            "/verify".to_string(),
            json!({
                "method": "POST",
                "description": "Request verification for an action",
                "request": {
                    "action": "rm -rf folder/",
                    "reason": "Cleaning up workspace after implementation",
                    "context": { "task_id": "CHASE-123" }
                },
                "response": {
                    "status": "pending",
                    "verification_id": "v-789-xyz",
                    "message": "Waiting for user approval"
                }
            }),
        );

        endpoints.insert(
            "/health".to_string(),
            json!({
                "method": "GET",
                "description": "Health check endpoint",
                "response": {
                    "status": "healthy",
                    "timestamp": "2026-02-02T23:30:00Z"
                }
            }),
        );

        endpoints.insert(
            "/config".to_string(),
            json!({
                "method": "GET",
                "description": "Retrieve configuration (supports ?format=json|yaml|markdown)",
                "response": {
                    "version": "0.1.0",
                    "timestamp": "2026-02-02T23:30:00Z",
                    "application": {
                        "name": "ChaseAI",
                        "version": "0.1.0"
                    },
                    "ports": [],
                    "endpoints": {}
                }
            }),
        );

        endpoints
    }

    /// Get endpoints available for a specific port role
    fn get_endpoints_for_role(role: PortRole) -> Vec<Value> {
        match role {
            PortRole::Instruction => vec![
                json!({
                    "path": "/context",
                    "method": "GET",
                    "description": "Retrieve instruction context"
                }),
                json!({
                    "path": "/config",
                    "method": "GET",
                    "description": "Retrieve configuration"
                }),
                json!({
                    "path": "/health",
                    "method": "GET",
                    "description": "Health check"
                }),
            ],
            PortRole::Verification => vec![
                json!({
                    "path": "/verify",
                    "method": "POST",
                    "description": "Request verification"
                }),
                json!({
                    "path": "/health",
                    "method": "GET",
                    "description": "Health check"
                }),
            ],
        }
    }
}
