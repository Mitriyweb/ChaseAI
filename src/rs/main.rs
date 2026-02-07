mod platform;
use clap::Parser;

// Re-export app module so it's accessible from main
pub use app::App;

#[derive(Parser)]
#[command(name = "chase")]
#[command(about = "Controlled Local AI Execution Platform", long_about = None)]
struct Cli {
    /// Verification request in JSON format
    #[arg(long)]
    verification: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(verification_data) = cli.verification {
        return handle_verification(verification_data);
    }

    platform::run()
}

fn handle_verification(data: String) -> anyhow::Result<()> {
    // 1. Load config to find the port
    let config = app::config::network_config::NetworkConfig::load()?;

    // 2. Find the first enabled verification port
    let binding = config
        .port_bindings
        .iter()
        .find(|b| b.role == app::network::port_config::PortRole::Verification && b.enabled)
        .ok_or_else(|| {
            anyhow::anyhow!("No enabled verification port found. Is ChaseAI running and configured with a verification port?")
        })?;

    let url = format!("http://{}:{}/verify", binding.interface.ip_address, binding.port);

    // 3. Parse data to ensure it's valid JSON
    let json_data: serde_json::Value = serde_json::from_str(&data)
        .map_err(|e| anyhow::anyhow!("Invalid JSON provided for verification: {}", e))?;

    // 4. Send request
    let client = reqwest::blocking::Client::new();
    let resp = client.post(url).json(&json_data).send()?;

    if resp.status().is_success() {
        let result: serde_json::Value = resp.json()?;
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        let status = resp.status();
        let text = resp.text().unwrap_or_else(|_| "Unknown error".to_string());
        eprintln!("Verification failed with status {}: {}", status, text);
        std::process::exit(1);
    }

    Ok(())
}
