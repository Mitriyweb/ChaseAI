# Design: Network Interface Management

## Architecture Overview

```
┌─────────────────────────────────────────┐
│          Tray UI Layer                  │
│  (Interface Selection, Port Config)     │
└──────────────┬──────────────────────────┘
               │
┌──────────────▼──────────────────────────┐
│      Network Configuration Layer        │
│   - InterfaceDetector                   │
│   - PortConfig                          │
│   - NetworkConfig (Persistence)         │
└──────────────┬──────────────────────────┘
               │
┌──────────────▼──────────────────────────┐
│        OS Network Stack                 │
│   (Platform-specific bindings)          │
└─────────────────────────────────────────┘
```

## Key Components

### 1. InterfaceDetector

**Responsibility**: Discover available network interfaces on the system

**API**:
```rust
pub struct InterfaceDetector;

impl InterfaceDetector {
    pub fn detect_all() -> Result<Vec<NetworkInterface>>;
    pub fn detect_loopback() -> Result<NetworkInterface>;
    pub fn detect_lan() -> Result<Vec<NetworkInterface>>;
    pub fn detect_public() -> Result<Option<NetworkInterface>>;
}

pub struct NetworkInterface {
    pub name: String,
    pub ip_address: IpAddr,
    pub interface_type: InterfaceType,
}

pub enum InterfaceType {
    Loopback,
    Lan,
    Public,
}
```

**Platform Dependencies**:
- Uses standard library `std::net` for basic IP handling
- May use `if-addrs` or similar crate for cross-platform interface enumeration

### 2. PortConfig

**Responsibility**: Manage port bindings and role assignments

**API**:
```rust
pub struct PortConfig {
    bindings: HashMap<u16, PortBinding>,
}

pub struct PortBinding {
    pub port: u16,
    pub interface: NetworkInterface,
    pub role: PortRole,
    pub enabled: bool,
}

pub enum PortRole {
    Instruction,
    Verification,
    Workflow,
}

impl PortConfig {
    pub fn new() -> Self;
    pub fn add_binding(&mut self, binding: PortBinding) -> Result<()>;
    pub fn remove_binding(&mut self, port: u16) -> Result<()>;
    pub fn get_binding(&self, port: u16) -> Option<&PortBinding>;
    pub fn validate_port(&self, port: u16) -> Result<()>;
}
```

**Validation Rules**:
- Port must be in valid range (1024-65535 for non-root)
- Port must not already be in use by system
- One role per port (but multiple ports can have same role)

### 3. NetworkConfig

**Responsibility**: Persist and load network configuration

**API**:
```rust
pub struct NetworkConfig {
    pub default_interface: InterfaceType,
    pub port_bindings: Vec<PortBinding>,
}

impl NetworkConfig {
    pub fn load() -> Result<Self>;
    pub fn save(&self) -> Result<()>;
    pub fn config_path() -> PathBuf;
}
```

**Storage Format** (TOML):
```toml
default_interface = "Loopback"

[[port_bindings]]
port = 3000
interface_name = "lo0"
interface_ip = "127.0.0.1"
role = "Instruction"
enabled = true

[[port_bindings]]
port = 3001
interface_name = "lo0"
interface_ip = "127.0.0.1"
role = "Verification"
enabled = true
```

**Config Location**:
- macOS: `~/.config/chase ai/network.toml`
- Linux: `~/.config/chaseai/network.toml`
- Windows: `%APPDATA%\ChaseAI\network.toml`

### 4. Tray UI

**Responsibility**: User interface for configuration

**Framework**: To be decided (options: tauri-tray, tray-icon)

**UI Elements**:
- Interface selector dropdown
- Port configuration table (port, role, interface, enabled)
- Add/Remove port binding buttons
- Save configuration button

## Data Flow

### Startup Flow
```
1. Load NetworkConfig from disk
2. Detect available interfaces (InterfaceDetector)
3. Validate loaded config against available interfaces
4. Update UI with current configuration
```

### User Configuration Flow
```
1. User selects interface in UI
2. User configures port binding (port, role)
3. PortConfig validates binding
4. If valid, add to PortConfig
5. Save NetworkConfig to disk
6. Update UI to reflect changes
```

### Port Binding Lifecycle
```
1. InterfaceDetector discovers interfaces
2. User creates PortBinding in UI
3. PortConfig validates (port available, valid range)
4. Binding stored in NetworkConfig
5. On app restart, bindings are restored
   (actual socket binding happens in instruction-service)
```

## Security Considerations

1. **Default to Loopback**: System defaults to loopback interface for maximum security
2. **Explicit LAN/Public**: User must explicitly enable LAN or public exposure
3. **Port Validation**: Validate ports are in safe range (≥1024) for non-root operation
4. **Config File Permissions**: Ensure config file is readable only by user

## Error Handling

All network operations use `anyhow::Result` for error propagation:
- Interface detection failures: Log warning, continue with available interfaces
- Port binding validation failures: Show user-friendly error in UI
- Config load failures: Fall back to secure defaults (loopback-only)
- Config save failures: Retry with backup location, warn user

## Testing Strategy

### Unit Tests
- InterfaceDetector: Mock network stack, test parsing logic
- PortConfig: Test validation rules, binding management
- NetworkConfig: Test serialization/deserialization

### Integration Tests
- Full flow: Detect → Configure → Save → Load → Verify
- Cross-platform: Test on macOS, Linux (Windows optional for MVP)

### Manual Testing
- Verify UI displays correctly
- Test with actual network configuration changes
- Test persistence across app restarts

## Trade-offs

### Decision: Tray UI vs Web UI
**Chosen**: Tray UI
**Rationale**:
- More native feel for system-level tool
- Lower resource usage
- Matches "local-first" philosophy
**Trade-off**: Web UI would be more portable but requires running web server

### Decision: Configuration Format (TOML)
**Chosen**: TOML
**Rationale**:
- Human-readable and editable
- Good Rust ecosystem support (serde)
- Simple structure fits our needs
**Trade-off**: JSON would be more universal, but TOML is more user-friendly

### Decision: Actual Socket Binding Location
**Chosen**: Defer to instruction-service
**Rationale**:
- This module only manages configuration
- Actual binding logic belongs with service implementation
- Separation of concerns
**Trade-off**: Could bind sockets here, but would complicate module responsibility

## Future Extensibility

- TLS/SSL configuration (add cert paths to PortBinding)
- Firewall rule generation (auto-configure OS firewall)
- Dynamic port discovery (mDNS/Bonjour for LAN)
- Multi-instance support (different configs per instance)
