use crate::network::port_config::PortRole;
use std::process::Command;

pub struct PortConfig {
    pub port: u16,
    pub role: PortRole,
    pub enabled: bool,
}

#[cfg(target_os = "macos")]
pub fn show_add_port_dialog() -> Option<PortConfig> {
    // Use AppleScript to show a dialog
    let script = r#"
        set portNumber to text returned of (display dialog "Enter port number:" default answer "8888" buttons {"Cancel", "Next"} default button "Next")

        set roleChoice to button returned of (display dialog "Select role:" buttons {"Instruction", "Verification"} default button "Instruction")

        set enableChoice to button returned of (display dialog "Enable immediately?" buttons {"No", "Yes"} default button "No")

        return portNumber & "|" & roleChoice & "|" & enableChoice
    "#;

    let output = Command::new("osascript").arg("-e").arg(script).output();

    match output {
        Ok(output) if output.status.success() => {
            let result = String::from_utf8_lossy(&output.stdout);
            let parts: Vec<&str> = result.trim().split('|').collect();

            if parts.len() == 3 {
                if let Ok(port) = parts[0].parse::<u16>() {
                    let role = if parts[1] == "Instruction" {
                        PortRole::Instruction
                    } else {
                        PortRole::Verification
                    };

                    let enabled = parts[2] == "Yes";

                    return Some(PortConfig {
                        port,
                        role,
                        enabled,
                    });
                }
            }
            None
        }
        _ => {
            println!("Dialog cancelled or failed");
            None
        }
    }
}

#[cfg(not(target_os = "macos"))]
pub fn show_add_port_dialog() -> Option<PortConfig> {
    // Fallback for non-macOS platforms
    None
}
