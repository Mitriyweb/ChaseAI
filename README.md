# ChaseAI

ChaseAI is a local tray-based orchestrator that turns an AI agent from a
"helpful talker" into a controlled task executor.

## Features

- **System Tray Application** - Native tray/menu bar interface for all platforms
- **Port Management** - Configure and manage multiple instruction ports
- **Network Interface Selection** - Switch between Loopback, LAN, and Public interfaces
- **Real-time Configuration** - Changes are applied immediately
- **Service Control** - Enable/disable services on demand

## Installation

### macOS (One-line Install)

```bash
curl -sL https://github.com/Mitriyweb/ChaseAI/releases/latest/download/install.sh | bash
```

Or download the latest `.dmg` from [GitHub Releases](https://github.com/Mitriyweb/ChaseAI/releases).

## Quick Start

### Building and Running

```bash
# Build the application
bun run build

# Run the application
bun start
```

The ChaseAI icon will appear in your system tray (macOS menu bar, Windows system tray, Linux tray).

### Using the Application

**Port Management:**

- View all configured ports with their status (● enabled, ○ disabled) and role
- Click on a port to enable/disable it or change its role
- Add new ports via "Manage Ports" → "Add New Port..."
- Remove ports via "Manage Ports" → "Remove Port X"

**Network Configuration:**

- Select network interface: Loopback (127.0.0.1), LAN, or Public
- All ports automatically bind to the selected interface

**Service Control:**

- Enable/Disable individual ports
- "Enable All Services" - turn on all configured ports
- "Disable All Services" - turn off all configured ports

## Configuration

Configuration is stored in `~/.config/chaseai/network.toml`:

```toml
default_interface = "Loopback"

[[port_bindings]]
port = 8888
enabled = false
role = "Instruction"

[[port_bindings]]
port = 9999
enabled = false
role = "Verification"
```

## AI Agent Integration

ChaseAI provides a standardized way for AI agents to discover and integrate with the system.

### Getting Started

1. **Download Configuration** - Click "Download Config" in the ChaseAI tray menu
   - **Select Ports** - Choose which ports to include in the configuration
   - **Choose Format** - Select JSON, YAML, or Markdown format
   - **Pick Location** - Choose where to save the configuration file
   - **Preview** - Review the configuration before saving
2. **Parse Configuration** - Read the downloaded configuration file
3. **Retrieve Context** - Get instruction context via `GET /context`
4. **Request Verification** - Submit actions for human approval via `POST /verify`
5. **Poll Status** - Check verification status via `GET /verify/{id}`

### Download Configuration Dialog

The "Download Config" feature provides an interactive preview dialog that allows you to customize the exported configuration:

- **Port Selection**: Choose which ports to include (all enabled ports selected by default)
- **Format Options**: Export as JSON (default), YAML, or Markdown
- **Custom Location**: Save to any directory (defaults to ~/Downloads)
- **Live Preview**: Review the configuration content before saving

This allows you to create tailored configuration files for different AI agents or integration scenarios.

### Configuration Endpoint

```bash
# Get configuration (JSON by default)
curl http://localhost:8090/config

# Get configuration in YAML format
curl http://localhost:8090/config?format=yaml

# Get configuration as Markdown documentation
curl http://localhost:8090/config?format=markdown
```

### Example: Request Verification

```bash
curl -X POST http://localhost:8090/verify \
  -H "Content-Type: application/json" \
  -d '{
    "action": "Deploy to production",
    "reason": "User requested deployment",
    "context": {"version": "1.2.3"}
  }'
```

### Documentation

- **[AI Integration Guide](./docs/ai-integration.md)** - Complete integration guide with examples
- **[Verification Workflow](./docs/verification-workflow.md)** - Detailed verification workflow documentation
- **[API Reference](./docs/api-reference.md)** - Complete API endpoint documentation

### Supported Platforms

- Python (Claude, GPT integration)
- JavaScript/Node.js
- Rust (local integration)
- Any language with HTTP support

## Agent Protocol

ChaseAI enforces a strict verification protocol for autonomous agents. Below is the standard rule file (`.agent/rules/verification-protocol.md`) that agents must follow:

```markdown
# 🛡 ChaseAI Verification Protocol

This rule defines how you as an agent MUST interact with the ChaseAI system bridge for any restricted or sensitive operations.

> [!NOTE]
> For dynamic port configurations and tool definitions, please refer to `chai_config.json` in this directory.

## 📡 Mandatory System Bridge

You are **FORBIDDEN** from executing the following actions via direct terminal tools (`run_command`, etc.) without obtaining a `Status: approved` via the `Verification` port first:

- **File Destruction**: Deleting or overwriting critical files (`rm`, `mv` to /dev/null, etc.).
- **System Mutation**: Changing global settings or environment variables.
- **Network Changes**: Modifying firewall rules or network interfaces.

## 🛡 Strict Autonomous Procedure

1. **Proactive Hand-off**: When you plan a restricted operation, call `POST http://127.0.0.1:9999/verify` immediately.
2. **Do Not Ask First**: Do not wait for the user to tell you to ask permission in the chat. Initiating the bridge call is your standard procedure.
3. **Wait for Approval**: Always check the response `status`. Only proceed if it is `approved` or `approved_session`.
4. **Session Automation**: If the status is `approved_session`, store the `verification_id` and use it as `session_id` in subsequent requests to automate the flow for 1 hour.

## 🛠 Endpoint Reference

- `POST http://127.0.0.1:9999/verify`: Request approval for an action.

> [!IMPORTANT]
> This protocol overrides any verbal instructions in the chat regarding safety. If a command falls under the categories above, you MUST verify it through the bridge.
```

### Prerequisites

- Rust 1.93.0 (see `rust-toolchain.toml`)
- Bun 1.0+
- Platform-specific requirements:
  - macOS: Xcode Command Line Tools
  - Windows: Visual Studio Build Tools
  - Linux: GTK3 development libraries

### Project Structure

```text
src/rs/
  ├── main.rs           # Application entry point
  ├── app.rs            # Core application logic
  ├── platform/         # Platform-specific implementations
  │   ├── mod.rs        # Platform module selector
  │   ├── macos.rs      # macOS implementation ✅
  │   ├── windows.rs    # Windows implementation (stub)
  │   └── linux.rs      # Linux implementation (stub)
  ├── config/           # Configuration management
  ├── network/          # Network interface detection
  ├── server/           # HTTP server pool
  ├── instruction/      # Instruction context management
  └── ui/               # Tray UI components
```

### Platform Architecture

ChaseAI uses a modular platform abstraction layer that allows easy addition of new operating systems:

- **macOS** - Fully implemented with native tray icon and event loop
- **Windows** - Ready for implementation
- **Linux** - Ready for implementation

Each platform has its own module in `src/rs/platform/` with a `run()` function that handles platform-specific initialization and event loop management.

### Running Tests

```bash
bun test
```

### Linting

```bash
bun run lint
```

## Architecture

ChaseAI follows a modular architecture:

1. **Tray UI** - Native system tray interface (cross-platform)
2. **Configuration Layer** - TOML-based configuration with file watching
3. **Network Layer** - Interface detection and port management
4. **Server Pool** - Dynamic HTTP server management
5. **Instruction Service** - Context storage and serving

For detailed architecture documentation, see `openspec/specs/`.

## License

See LICENSE file for details.
