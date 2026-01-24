# Design: System Tray UI (macOS)

## Architecture Overview

The Tray UI serves as the control plane for the backend services. It operates as a thin layer that reads from and writes to common configuration files, while also triggering lifecycle events in the instruction service.

```text
┌─────────────────────────────────────────┐
│              macOS Tray UI              │
│  (Menu Bar Icon, Dropdowns, Toggles)    │
└──────────────┬──────────────────────────┘
               │ Event Loop
               ▼
┌─────────────────────────────────────────┐
│         UI Event Handler Layer          │
│   (Menu Actions -> Config Updates)      │
└──────────────┬─────────────┬────────────┘
               │             │
┌──────────────▼──────┐  ┌───▼────────────┐
│   Network Config    │  │ Instruction    │
│    (shared file)    │  │ Service Control│
└─────────────────────┘  └────────────────┘
```

## Key Components

### 1. Tray Manager

**Responsibility**: Initialize the system tray icon and manage the application event loop on macOS.

**API Pattern**:

```rust
pub struct TrayManager {
    tray: TrayIcon,
    menu: Menu,
}

impl TrayManager {
    pub fn init() -> Result<Self>;
    pub fn update_menu(&mut self, config: &NetworkConfig) -> Result<()>;
    pub fn run_event_loop(&mut self);
}
```

### 2. Menu Builder

**Responsibility**: Construct a dynamic menu structure that reflects the real-time state of network interfaces and port roles.

**Menu Map**:

- **Status Section**:
  - `ChaseAI: Running` (or Disabled)
  - `IP: 127.0.0.1 (Loopback)`
- **Configuration Section**:
  - `Select Interface >` (Loopback | LAN | Public)
  - `Port 3000: Instruction [ON/OFF]`
  - `Port 3001: Verification [ON/OFF]`
- **Global Commands**:
  - `Enable All Services` / `Disable All Services`
  - `Restart Services`
  - `Exit`

### 3. State Sync

**Responsibility**: Ensure the UI remains in sync with the configuration files even if they are modified by other processes or CLI tools.

- **File Watcher**: Uses the `notify` crate to watch for changes in `network.toml` and `contexts.json`.
- **UI Refresh**: When a change is detected, the `TrayManager` re-builds the menu to reflect the new state.

## Implementation Details

- **Technology**: Rust with `tray-icon` and `menu` crates (or similar lightweight macOS-friendly libraries).
- **Process Model**: Single process. The UI thread runs the event loop, while the backend servers run in separate Tokio tasks.
- **Icon Assets**: Minimalist black/white icon (SVG or PNG) following macOS Human Interface Guidelines for menu bar items.

## User Interactions

### Interface Switching

1. User clicks "Select Interface".
2. User selects "LAN (192.168.1.10)".
3. UI updates `network.toml` with `default_interface = "Lan"`.
4. UI triggers a reload of the `instruction-service`.

### Port Role Management

1. User sees "Port 3000: Instruction".
2. User clicks the item.
3. UI toggles the `enabled` field for that port in `network.toml`.
4. `instruction-service` is notified to start or stop the server on port 3000.

## Error Handling

- **Bind Failure**: If a port cannot be bound, the menu item for that port should show a "Warning" icon or text: `! Port 3000: Conflict`.
- **Config Corruption**: If the config file is unparseable, the UI should show a "Setup Required" menu item and default to a safe loopback-only state.

## Security

- No sensitive data is managed directly in the UI.
- All configuration changes are persisted to the user-owned config directory.
- UI does not require elevated privileges (running on ports >1024).
