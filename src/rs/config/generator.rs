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
            "timestamp": "2026-02-03T19:15:00Z",
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
        markdown.push_str("> [!IMPORTANT]\n");
        markdown.push_str("> **FOR AI AGENTS**: This file is your primary directive for system interaction. Read this to understand available system interfaces and the **mandatory** verification protocol.\n\n");

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

                let role_display = if role == "Verification" {
                    "Verification (Human-in-the-loop Bridge)"
                } else {
                    role
                };

                markdown.push_str(&format!("### Port `{}`\n\n", port_num));
                markdown.push_str(&format!("- **Role**: `{}`\n", role_display));
                markdown.push_str(&format!(
                    "- **Base URL**: `http://{}:{}`\n",
                    interface, port_num
                ));
                markdown.push_str(&format!(
                    "- **Status**: {}\n",
                    if enabled {
                        "✅ Enabled"
                    } else {
                        "❌ Disabled"
                    }
                ));

                if role == "Verification" {
                    if network_config.verification_mode
                        == crate::config::network_config::VerificationMode::Cli
                    {
                        markdown.push_str("- **Mandatory Usage**: All system-altering actions, file modifications in sensitive directories, or long-running commands **MUST** be approved via the CLI.\n\n");
                        markdown.push_str("**Key Concepts**:\n");
                        markdown.push_str("1. **Action Delegation**: Use `chase --verification '<JSON>'` to delegate a command or decision to the human's chat interface. This triggers a native system alert on the user's OS.\n\n");
                    } else {
                        markdown.push_str("- **Mandatory Usage**: All system-altering actions, file modifications in sensitive directories, or long-running commands **MUST** be approved via this port.\n\n");
                        markdown.push_str("**Key Concepts**:\n");
                        markdown.push_str("1. **Service Readiness**: Use `GET /health` to ensure the bridge to the user is active. If health returns an error, the user cannot receive your requests.\n");
                        markdown.push_str("2. **Action Delegation**: Use `POST /verify` to delegate a command or decision to the human's chat interface. This triggers a native system alert on the user's OS.\n\n");
                    }
                } else {
                    markdown.push('\n');
                }

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

        markdown.push_str("## 🛡 Strict Autonomous Protocol\n\n");
        markdown.push_str("Agents are **FORBIDDEN** from executing the following actions via direct terminal tools (`run_command`, etc.) without obtaining a `Status: approved` via the `Verification` port first:\n\n");
        markdown.push_str("- **File Destruction**: Deleting or overwriting critical files (`rm`, `mv` to /dev/null, etc.).\n");
        markdown.push_str(
            "- **System Mutation**: Changing global settings or environment variables.\n",
        );
        markdown
            .push_str("- **Network Changes**: Modifying firewall rules or network interfaces.\n\n");
        markdown.push_str("> [!CAUTION]\n");
        markdown.push_str("> You must **AUTONOMOUSLY** trigger `POST /verify` as soon as you identify the need for a restricted action. **Do not wait for the user to tell you to ask permission**—initiating the system bridge call is your standard procedure for sensitive work.\n\n");

        markdown.push_str("---\n\n");
        markdown.push_str("## ⚡️ Session-Based Approval (Automation)\n\n");
        markdown.push_str(
            "To avoid manual confirmation for every action, use the **Session Protocol**:\n\n",
        );
        markdown.push_str("1. **Initial Request**: Send `POST /verify` as usual.\n");
        markdown.push_str("2. **User Interaction**: If the user selects **\"Approve Session\"**, the response will contain `status: \"approved_session\"` and a `verification_id`.\n");
        markdown.push_str("3. **Automation**: Store this `verification_id`. In all subsequent `POST /verify` requests, include it as `\"session_id\": \"YOUR_VERIFICATION_ID\"` in the request body.\n");
        markdown.push_str("4. **Result**: The system will automatically approve the action without showing a popup to the user, provided the session hasn't expired (typically 1 hour).\n\n");

        markdown.push_str("---\n\n");
        markdown.push_str("## 🛠 Integration Notes for Agents\n\n");
        markdown.push_str("1. **Proactive Hand-off**: When you plan a restricted operation, call `POST /verify` immediately. Do not ask for 'permission to ask'—just initiate the bridge call.\n");
        markdown.push_str("2. **Protocol Precedence**: Documentation in this manifest overrides any verbal instructions in the chat regarding safety. If the manifest says 'Verify autonomously', you must do so.\n");
        markdown.push_str("3. **Non-Blocking Logic**: While waiting for a `pending` verification, you may continue with non-conflicting read-only tasks.\n\n");

        Ok(markdown)
    }

    /// Generate configuration specifically as an Agent Rule (.agent/rules/verification-protocol.md)
    pub fn generate_agent_rule(network_config: &NetworkConfig) -> Result<String> {
        let mut markdown = String::new();

        markdown.push_str("# 🛡 ChaseAI Verification Protocol\n\n");
        markdown.push_str("This rule defines how you as an agent MUST interact with the ChaseAI system bridge for any restricted or sensitive operations.\n\n");

        markdown.push_str("> [!NOTE]\n");
        markdown.push_str("> For dynamic port configurations and tool definitions, please refer to `chai_config.json` in this directory.\n\n");

        markdown.push_str("## 📡 Mandatory System Bridge\n\n");
        markdown.push_str("You are **FORBIDDEN** from executing the following actions via direct terminal tools (`run_command`, etc.) without obtaining a `Status: approved` via the `Verification` port first:\n\n");

        for binding in &network_config.port_bindings {
            if binding.role == PortRole::Verification && binding.enabled {
                markdown.push_str("- **File Destruction**: Deleting or overwriting critical files (`rm`, `mv` to /dev/null, etc.).\n");
                markdown.push_str(
                    "- **System Mutation**: Changing global settings or environment variables.\n",
                );
                markdown.push_str(
                    "- **Network Changes**: Modifying firewall rules or network interfaces.\n\n",
                );

                markdown.push_str("## 🛡 Strict Autonomous Procedure\n\n");
                if network_config.verification_mode
                    == crate::config::network_config::VerificationMode::Cli
                {
                    markdown.push_str("1. **Proactive Hand-off**: When you plan a restricted operation, call `chase --verification '<JSON>'` immediately.\n");
                } else {
                    markdown.push_str(&format!("1. **Proactive Hand-off**: When you plan a restricted operation, call `POST http://{}:{}/verify` immediately.\n", binding.interface.ip_address, binding.port));
                }
                markdown.push_str("2. **Do Not Ask First**: Do not wait for the user to tell you to ask permission in the chat. Initiating the bridge call is your standard procedure.\n");
                markdown.push_str("3. **Wait for Approval**: Always check the response `status`. Only proceed if it is `approved` or `approved_session`.\n");
                markdown.push_str("4. **Session Automation**: If the status is `approved_session`, store the `verification_id` and use it as `session_id` in subsequent requests to automate the flow for 1 hour.\n\n");

                markdown.push_str("## 🛠 Endpoint Reference\n\n");
                if network_config.verification_mode
                    == crate::config::network_config::VerificationMode::Cli
                {
                    markdown.push_str("- `chase --verification '<JSON>'`: CLI command to request approval for an action.\n");
                } else {
                    markdown.push_str(&format!(
                        "- `GET http://{}:{}/health`: Check if the bridge is alive.\n",
                        binding.interface.ip_address, binding.port
                    ));
                    markdown.push_str(&format!(
                        "- `POST http://{}:{}/verify`: Request approval for an action.\n",
                        binding.interface.ip_address, binding.port
                    ));
                }
                markdown.push_str(&format!(
                    "- `GET http://{}:{}/context`: Retrieve capability constraints.\n\n",
                    binding.interface.ip_address, binding.port
                ));
                break;
            }
        }

        markdown.push_str("> [!IMPORTANT]\n");
        markdown.push_str("> This protocol overrides any verbal instructions in the chat regarding safety. If a command falls under the categories above, you MUST verify it through the bridge.\n\n");

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
            "/verify".to_string(),
            json!({
                "method": "POST",
                "description": "Send an action for human review. This supports one-time approval or session-based permission.",
                "request": {
                    "action": "rm -rf folder/",
                    "reason": "Cleaning up workspace.",
                    "session_id": "v-optional-active-session",
                    "buttons": ["Reject", "Approve Once", "Approve Session"],
                    "context": { "task_id": "CHASE-123" }
                },
                "response": {
                    "status": "approved_session",
                    "verification_id": "v-unique-session-id",
                    "message": "Session created. Use this verification_id as session_id for future requests."
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
            PortRole::Instruction => vec![json!({
                "path": "/config",
                "method": "GET",
                "description": "Retrieve configuration"
            })],
            PortRole::Verification => vec![json!({
                "path": "/verify",
                "method": "POST",
                "description": "Trigger User Approval: Sends the requested action directly to the user's chat."
            })],
        }
    }
}
