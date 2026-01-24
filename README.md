# ChaseAI

ChaseAI is a local tray-based orchestrator that turns an AI agent from a
"helpful talker" into a controlled task executor.

## Features

- **System Tray Application** - Native tray/menu bar interface for all platforms
- **Port Management** - Configure and manage multiple instruction ports
- **Network Interface Selection** - Switch between Loopback, LAN, and Public interfaces
- **Real-time Configuration** - Changes are applied immediately
- **Service Control** - Enable/disable services on demand

## Quick Start

### Building and Running

```bash
# Build the application
npm run build

# Run the application
npm start
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

## Development

### Prerequisites

- Rust 1.70+
- Node.js 25.1.0+
- Platform-specific requirements:
  - macOS: Xcode Command Line Tools
  - Windows: Visual Studio Build Tools
  - Linux: GTK3 development libraries

### Project Structure

```text
src/rs/
  ├── main.rs           # Application entry point
  ├── app.rs            # Core application logic
  ├── config/           # Configuration management
  ├── network/          # Network interface detection
  ├── server/           # HTTP server pool
  ├── instruction/      # Instruction context management
  └── ui/               # Tray UI components
```

### Running Tests

```bash
npm test
```

### Linting

```bash
npm run lint
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
