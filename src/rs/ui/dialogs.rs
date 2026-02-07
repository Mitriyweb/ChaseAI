use crate::network::port_config::PortRole;
#[cfg(target_os = "macos")]
use std::process::Command;

pub struct PortConfig {
    pub port: u16,
    pub role: PortRole,
    pub enabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConfigFormat {
    Json,
    Yaml,
    Markdown,
    AgentRule,
}

impl ConfigFormat {
    pub fn extension(&self) -> &str {
        match self {
            ConfigFormat::Json => "json",
            ConfigFormat::Yaml => "yaml",
            ConfigFormat::Markdown => "md",
            ConfigFormat::AgentRule => "md",
        }
    }

    pub fn name(&self) -> &str {
        match self {
            ConfigFormat::Json => "JSON",
            ConfigFormat::Yaml => "YAML",
            ConfigFormat::Markdown => "Markdown",
            ConfigFormat::AgentRule => "Agent Rule",
        }
    }
}

pub struct ConfigDownloadOptions {
    pub selected_ports: Vec<u16>,
    pub format: ConfigFormat,
    pub save_path: std::path::PathBuf,
}

#[cfg(target_os = "macos")]
pub fn show_add_port_dialog(default_port: u16) -> Option<PortConfig> {
    // Single-screen fast dialog: Field for Port, Buttons for Role
    let script = format!(
        r#"
        activate
        set response to display dialog "Enter Port Number:" default answer "{}" buttons {{"Cancel", "Instruction", "Verification"}} default button "Instruction" with title "Add New Port"
        set pNum to text returned of response
        set roleChoice to button returned of response
        return pNum & "|" & roleChoice
        "#,
        default_port
    );

    let output = Command::new("osascript").arg("-e").arg(script).output();

    match output {
        Ok(output) if output.status.success() => {
            let result = String::from_utf8_lossy(&output.stdout).trim().to_string();
            let parts: Vec<&str> = result.split('|').collect();
            if parts.len() == 2 {
                if let Ok(port) = parts[0].parse::<u16>() {
                    let role = if parts[1] == "Verification" {
                        PortRole::Verification
                    } else {
                        PortRole::Instruction
                    };
                    // Port is enabled by default when added via this dialog
                    return Some(PortConfig {
                        port,
                        role,
                        enabled: true,
                    });
                }
            }
            None
        }
        _ => None,
    }
}

#[cfg(not(target_os = "macos"))]
pub fn show_add_port_dialog(_default_port: u16) -> Option<PortConfig> {
    // Fallback for non-macOS platforms
    None
}

#[cfg(not(target_os = "macos"))]
pub fn show_download_config_dialog(
    _config: &crate::config::network_config::NetworkConfig,
) -> Option<ConfigDownloadOptions> {
    None
}

#[cfg(not(target_os = "macos"))]
pub fn show_manage_port_dialog(
    _binding: &crate::network::port_config::PortBinding,
) -> Option<PortAction> {
    None
}

#[cfg(target_os = "macos")]
pub fn show_download_config_dialog(
    config: &crate::config::network_config::NetworkConfig,
) -> Option<ConfigDownloadOptions> {
    use std::path::PathBuf;

    let all_port_bindings: Vec<_> = config.port_bindings.clone();
    if all_port_bindings.is_empty() {
        let _ = Command::new("osascript")
            .arg("-e")
            .arg("display dialog \"No ports configured.\" buttons {\"OK\"} default button \"OK\"")
            .output();
        return None;
    }

    // Step 1: Select Ports
    let mut port_menu_items: Vec<String> = Vec::new();
    let mut port_default_selections: Vec<String> = Vec::new();

    let mut roles_seen = std::collections::HashSet::new();
    for b in &all_port_bindings {
        let is_default = b.enabled && !roles_seen.contains(&b.role);
        if is_default {
            roles_seen.insert(b.role);
        }

        let label = format!(
            "{} Port {} ({:?}, {})",
            if is_default { "[✓]" } else { "[  ]" },
            b.port,
            b.role,
            if b.enabled { "Active" } else { "Inactive" }
        );
        port_menu_items.push(label.clone());
        if is_default {
            port_default_selections.push(label);
        }
    }

    let port_menu_str = port_menu_items.join("\", \"");
    let port_default_str = port_default_selections.join("\", \"");

    let port_script = format!(
        r#"
        activate
        set menuList to {{"{}"}}
        set defaultItems to {{"{}"}}
        set choices to choose from list menuList with prompt "Select PORTS to include (Hold Cmd for multiple):" default items defaultItems OK button name "Next" cancel button name "Cancel" with multiple selections allowed
        if choices is false then return "CANCELLED"
        set resultStr to ""
        repeat with choice in choices
            set resultStr to resultStr & choice & "\n"
        end repeat
        return resultStr
        "#,
        port_menu_str, port_default_str
    );

    let port_output = Command::new("osascript")
        .arg("-e")
        .arg(&port_script)
        .output();
    let mut selected_ports: Vec<u16> = Vec::new();

    match port_output {
        Ok(output) if output.status.success() => {
            let result = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if result == "CANCELLED" {
                return None;
            }
            for line in result.lines() {
                for b in &all_port_bindings {
                    if line.contains(&format!("Port {}", b.port)) {
                        selected_ports.push(b.port);
                    }
                }
            }
        }
        _ => return None,
    }

    if selected_ports.is_empty() {
        let _ = Command::new("osascript")
            .arg("-e")
            .arg("activate\ndisplay dialog \"No ports selected. Please select at least one port.\" buttons {\"OK\"} default button \"OK\"")
            .output();
        return None;
    }

    // Step 2: Select Format
    let format_script = r#"
        activate
        set formatChoices to {"JSON", "YAML", "Markdown", "Agent Rule"}
        set formatChoice to choose from list formatChoices with prompt "Select Export Format:" default items {"Agent Rule"} OK button name "Next" cancel button name "Cancel"
        if formatChoice is false then return "CANCELLED"
        return item 1 of formatChoice
    "#;

    let format_output = Command::new("osascript")
        .arg("-e")
        .arg(format_script)
        .output();
    let selected_format = match format_output {
        Ok(output) if output.status.success() => {
            let result = String::from_utf8_lossy(&output.stdout).trim().to_string();
            match result.as_str() {
                "JSON" => ConfigFormat::Json,
                "YAML" => ConfigFormat::Yaml,
                "Markdown" => ConfigFormat::Markdown,
                "Agent Rule" => ConfigFormat::AgentRule,
                "CANCELLED" => return None,
                _ => ConfigFormat::Markdown,
            }
        }
        _ => return None,
    };

    let format = selected_format;

    // Step 2: Location Selection
    let location_script = r#"
        activate
        set defaultLocation to (path to downloads folder)
        set chosenFolder to choose folder with prompt "Select save location:" default location defaultLocation
        return POSIX path of chosenFolder
    "#;

    let location_output = Command::new("osascript")
        .arg("-e")
        .arg(location_script)
        .output();

    let save_path = match location_output {
        Ok(output) if output.status.success() => {
            let result = String::from_utf8_lossy(&output.stdout).trim().to_string();
            PathBuf::from(result)
        }
        _ => return None,
    };

    // Step 3: Preview & Confirm
    // Generate preview
    let preview_config = crate::config::network_config::NetworkConfig {
        port_bindings: config
            .port_bindings
            .iter()
            .filter(|b| selected_ports.contains(&b.port))
            .cloned()
            .collect(),
        ..config.clone()
    };

    let preview_text = match format {
        ConfigFormat::Json => {
            crate::config::generator::ConfigurationGenerator::generate_json(&preview_config)
                .ok()
                .and_then(|v| serde_json::to_string_pretty(&v).ok())
                .unwrap_or_else(|| "Error generating preview".to_string())
        }
        ConfigFormat::Yaml => {
            crate::config::generator::ConfigurationGenerator::generate_yaml(&preview_config)
                .unwrap_or_else(|_| "Error generating preview".to_string())
        }
        ConfigFormat::Markdown => {
            crate::config::generator::ConfigurationGenerator::generate_markdown(&preview_config)
                .unwrap_or_else(|_| "Error generating preview".to_string())
        }
        ConfigFormat::AgentRule => {
            crate::config::generator::ConfigurationGenerator::generate_agent_rule(&preview_config)
                .unwrap_or_else(|_| "Error generating preview".to_string())
        }
    };

    // Truncate preview if too long (AppleScript has limits)
    let preview_display = if preview_text.len() > 1000 {
        format!(
            "{}...\n\n(Preview truncated, full content will be saved)",
            &preview_text[..1000]
        )
    } else {
        preview_text.clone()
    };

    let preview_script = format!(
        r#"
        activate
        set previewText to "{}"
        set confirmChoice to button returned of (display dialog "Preview:\n\n" & previewText buttons {{"Cancel", "Download"}} default button "Download" giving up after 30)
        return confirmChoice
        "#,
        preview_display.replace("\"", "\\\"").replace("\n", "\\n")
    );

    let confirm_output = Command::new("osascript")
        .arg("-e")
        .arg(&preview_script)
        .output();

    match confirm_output {
        Ok(output) if output.status.success() => {
            let result = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if result == "Download" || result.is_empty() {
                // Empty means timeout (giving up after 30s) - treat as confirm
                Some(ConfigDownloadOptions {
                    selected_ports,
                    format,
                    save_path,
                })
            } else {
                None
            }
        }
        _ => None,
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PortAction {
    Toggle,
    SetRole(PortRole),
    Delete,
}

#[cfg(target_os = "macos")]
pub fn show_manage_port_dialog(
    binding: &crate::network::port_config::PortBinding,
) -> Option<PortAction> {
    use std::process::Command;

    let status_action = if binding.enabled {
        "🔴 Disable Port"
    } else {
        "🟢 Enable Port"
    };
    let current_role = match binding.role {
        PortRole::Instruction => "Instruction",
        PortRole::Verification => "Verification",
    };

    let script = format!(
        r#"
        activate
        set choice to choose from list {{"{}", "🛠 Change Role to Instruction", "🛠 Change Role to Verification", "🗑 Remove Port"}} with prompt "Managing Port {} (Role: {})\nSelect Action:" default items {{"{}"}} OK button name "Apply"

        if choice is false then
            return "CANCEL"
        end if

        set action to item 1 of choice
        if action is "{}" then
            return "TOGGLE"
        else if action is "🛠 Change Role to Instruction" then
            return "ROLE:Instruction"
        else if action is "🛠 Change Role to Verification" then
            return "ROLE:Verification"
        else if action is "🗑 Remove Port" then
            return "DELETE"
        end if
        "#,
        status_action, binding.port, current_role, status_action, status_action
    );

    let output = Command::new("osascript").arg("-e").arg(&script).output();

    match output {
        Ok(output) if output.status.success() => {
            let result = String::from_utf8_lossy(&output.stdout).trim().to_string();
            match result.as_str() {
                "TOGGLE" => Some(PortAction::Toggle),
                "ROLE:Instruction" => Some(PortAction::SetRole(PortRole::Instruction)),
                "ROLE:Verification" => Some(PortAction::SetRole(PortRole::Verification)),
                "DELETE" => Some(PortAction::Delete),
                _ => None,
            }
        }
        _ => None,
    }
}

#[cfg(target_os = "macos")]
pub fn show_verification_dialog(
    action: &str,
    reason: &str,
    context_str: &str,
    buttons: &[String],
    task_id: &str,
) -> (usize, Option<String>) {
    let buttons_list = buttons
        .iter()
        .map(|b| format!("\"{}\"", b))
        .collect::<Vec<_>>()
        .join(", ");

    let default_button = buttons
        .last()
        .cloned()
        .unwrap_or_else(|| "Approve".to_string());

    // Simplified script: directly use activate without System Events check for maximum compatibility
    let script = format!(
        r#"
        activate
        set userResponse to display alert "🚨 {} | ChaseAI" message "Action: " & "{}" & "\n\nReason: " & "{}" & "\n\nContext: " & "{}" as critical buttons {{{}}} default button "{}"
        return button returned of userResponse
        "#,
        task_id,
        action.replace("\"", "\\\""),
        reason.replace("\"", "\\\""),
        context_str.replace("\"", "\\\""),
        buttons_list,
        default_button
    );

    let output = Command::new("osascript").arg("-e").arg(script).output();

    match output {
        Ok(output) if output.status.success() => {
            let result = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if let Some(pos) = buttons.iter().position(|b| b == &result) {
                (pos, Some(format!("User selected '{}' via ChaseAI", result)))
            } else {
                (
                    buttons.len(),
                    Some("Verification cancelled or button mismatch".to_string()),
                )
            }
        }
        _ => (
            buttons.len(),
            Some("Verification cancelled or failed".to_string()),
        ),
    }
}

#[cfg(not(target_os = "macos"))]
pub fn show_verification_dialog(
    _action: &str,
    _reason: &str,
    _context: &str,
    _buttons: &[String],
    _task_id: &str,
) -> (usize, Option<String>) {
    (
        0,
        Some("Verification not supported on this platform".to_string()),
    )
}
