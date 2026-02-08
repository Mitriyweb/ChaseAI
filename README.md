# ChaseAI

ChaseAI is a local tray-based orchestrator that turns an AI agent from a
"helpful talker" into a controlled task executor.

## Features

### Production Ready ✅

- **System Tray Application** - Native tray/menu bar interface for all platforms
- **Verification System** - Human-in-the-loop approval for sensitive operations
- **Real-time Configuration** - Changes are applied immediately
- **Service Control** - Enable/disable services on demand

### Beta Features 🧪

- **Port Management** - Configure and manage multiple instruction ports
- **Instruction Server** - Retrieve config via HTTP API (`GET /config`)
- **Download Config** - Export configuration as JSON, YAML, Markdown, or Agent Rule
- **Add/Remove Ports** - Dynamically manage ports from the UI
- **Network Interface Selection** - Switch between Loopback, LAN, and Public interfaces

## Installation

### macOS (One-line Install)

```bash
curl -sL https://github.com/Mitriyweb/ChaseAI/releases/latest/download/install.sh | bash
```

Or download the latest `.dmg` from [GitHub Releases](https://github.com/Mitriyweb/ChaseAI/releases).

## Quick Start

### Building and Running

```bash
# Production build (stable features only)
cargo build

# Beta build (includes Port Management, Instruction Server, Download Config, etc.)
cargo build --features beta

# Run the application
./target/debug/chase-ai
```

The ChaseAI icon will appear in your system tray (macOS menu bar, Windows system tray, Linux tray).

### Using the Application

**Production Build:**

- View verification port status (● enabled, ○ disabled)
- Enable/Disable verification port
- "Enable All Services" - turn on all services
- "Disable All Services" - turn off all services

**Beta Build:**

- View all configured ports with their status and role
- Click on a port to enable/disable it or change its role
- Add new ports via "Add New Port..."
- Remove ports via port menu
- Select network interface: Loopback, LAN, or Public
- Download configuration in JSON, YAML, Markdown, or Agent Rule format

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

## Beta Features

To enable beta features, build with the `beta` feature flag:

```bash
# Build with beta features
cargo build --features beta

# Or with bun
bun run build --features beta
```

### Available in Beta

- **Download Config** - Export configuration in multiple formats
- **Add/Remove Ports** - Manage ports from the UI
- **Interface Selection** - Switch network interfaces from menu

### Testing Beta Features

```bash
# Run tests for beta features
cargo test --features beta

# Run all tests
cargo test
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
# All tests
cargo test

# Unit tests only
cargo test --lib

# Integration tests only
cargo test --test '*'

# Tests with beta features
cargo test --features beta

# Specific test
cargo test test_download_config
```

All tests follow the [Rust Testing Standards](.kiro/steering/rust-testing-standards.md):

- Unit tests are co-located with source code using `#[cfg(test)]`
- Integration tests are in `src/test/rs/` with `_integration.rs` suffix
- Currently 47 tests covering core functionality

### Linting and Formatting

```bash
# Format code
cargo fmt --all

# Check formatting
cargo fmt --all -- --check

# Run clippy lints
cargo clippy --all-targets

# Check for unused dependencies
cargo +nightly udeps --all-targets
```

### Pre-commit Hooks

All commits must pass pre-commit hooks. **Do not use `git commit --no-verify`**.

Pre-commit hooks automatically run:

- Rust formatting (`cargo fmt`)
- Markdown linting (`markdownlint-cli2`)
- Rust linting (`clippy`)
- Unused dependencies check (`cargo udeps`)
- Test organization validation
- Build verification

```bash
# Pre-commit hooks run automatically before each commit
# If a hook fails, fix the issues and try committing again

# To manually run all pre-commit hooks:
pre-commit run --all-files
```

**Important:** Using `--no-verify` bypasses quality checks and is **not allowed** in this project. All commits must pass pre-commit validation.

**Setup:** Git hooks are configured in `.githooks/` and automatically used via `core.hooksPath`.

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
