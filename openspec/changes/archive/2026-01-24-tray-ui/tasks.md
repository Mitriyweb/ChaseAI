# Tasks: System Tray UI

## 1. Project Initialization & Dependencies

- [x] Select and add tray library to `Cargo.toml` (e.g., `tray-icon` or `tauri-runtime-tray`).
- [x] Set up basic macOS application lifecycle to support tray-only execution (no main window).
- [x] Add basic tray icon asset to the project.
- [x] Validate: Application starts with a visible icon in the tray.

## 2. Dynamic Menu Construction

- [x] Implement `src/rs/ui/tray_menu.rs` module.
- [x] Create function to build the menu based on current `NetworkConfig`.
- [x] Implement IP/Interface selection submenu.
- [x] Implement Port status and Role assignment menu items.
- [x] Validate: Menu correctly displays the current state of the system interfaces.

## 3. Interaction Handling

- [x] Set up event loop for tray menu clicks.
- [x] Implement interface switching logic (updates `NetworkConfig`).
- [x] Implement port role toggling (updates `NetworkConfig`).
- [x] Implement service Enable/Disable state persistence.
- [x] Validate: Clicking menu items updates the underlying JSON/TOML configuration files.

## 4. Cross-Module Integration

- [x] Integrate with `network-interface-management` for interface detection.
- [x] Integrate with `instruction-service` to start/stop servers when toggled from UI.
- [x] Implement observer pattern to update UI when конфигурация files are modified externally.
- [x] Validate: Enabling a port in the UI immediately starts a server on that port.

## 5. Polishing & Error Handling

- [x] Implement menu icons for status (Green for active, Gray for disabled).
- [x] Handle error states (e.g., port conflict errors) and show as "Error" in menu.
- [x] Finalize macOS bundle configuration.
- [x] Validate: Application shuts down cleanly and releases all resources on "Exit".

## Dependencies

- All tasks depend on Task 1 (basic tray setup).
- Task 4 depends on completion of `network-interface-management` and `instruction-service`.
- Integration tasks (4) require the backend proposals to be at least partially implemented.
