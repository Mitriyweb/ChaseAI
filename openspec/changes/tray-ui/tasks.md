# Tasks: System Tray UI

## 1. Project Initialization & Dependencies

- [ ] Select and add tray library to `Cargo.toml` (e.g., `tray-icon` or `tauri-runtime-tray`).
- [ ] Set up basic macOS application lifecycle to support tray-only execution (no main window).
- [ ] Add basic tray icon asset to the project.
- [ ] Validate: Application starts with a visible icon in the tray.

## 2. Dynamic Menu Construction

- [ ] Implement `src/rs/ui/tray_menu.rs` module.
- [ ] Create function to build the menu based on current `NetworkConfig`.
- [ ] Implement IP/Interface selection submenu.
- [ ] Implement Port status and Role assignment menu items.
- [ ] Validate: Menu correctly displays the current state of the system interfaces.

## 3. Interaction Handling

- [ ] Set up event loop for tray menu clicks.
- [ ] Implement interface switching logic (updates `NetworkConfig`).
- [ ] Implement port role toggling (updates `NetworkConfig`).
- [ ] Implement service Enable/Disable state persistence.
- [ ] Validate: Clicking menu items updates the underlying JSON/TOML configuration files.

## 4. Cross-Module Integration

- [ ] Integrate with `network-interface-management` for interface detection.
- [ ] Integrate with `instruction-service` to start/stop servers when toggled from UI.
- [ ] Implement observer pattern to update UI when конфигурация files are modified externally.
- [ ] Validate: Enabling a port in the UI immediately starts a server on that port.

## 5. Polishing & Error Handling

- [ ] Implement menu icons for status (Green for active, Gray for disabled).
- [ ] Handle error states (e.g., port conflict errors) and show as "Error" in menu.
- [ ] Finalize macOS bundle configuration.
- [ ] Validate: Application shuts down cleanly and releases all resources on "Exit".

## Dependencies

- All tasks depend on Task 1 (basic tray setup).
- Task 4 depends on completion of `network-interface-management` and `instruction-service`.
- Integration tasks (4) require the backend proposals to be at least partially implemented.
