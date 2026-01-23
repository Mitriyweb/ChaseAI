# Tasks: Network Interface Management

## 1. Network Interface Detection

- [x] Create `src/rs/network/interface_detector.rs` module
- [x] Implement loopback interface detection
- [x] Implement LAN interface detection
- [x] Implement public IP detection (optional)
- [x] Add unit tests for interface detection (≥85% coverage)
- [x] Validate: Run `cargo test` and verify all interface types are detected

## 2. Port Binding Configuration

- [x] Create `src/rs/network/port_config.rs` module
- [x] Define PortRole enum (Instruction, Verification, Workflow)
- [x] Implement port binding validation (check if port is available)
- [x] Add port-to-interface binding logic
- [x] Add unit tests for port configuration (≥85% coverage)
- [x] Validate: Verify ports can be bound to specific interfaces

## 3. Configuration Persistence

- [x] Create `src/rs/config/network_config.rs` module
- [x] Implement configuration serialization (JSON/TOML)
- [x] Implement configuration loading on startup
- [x] Implement configuration save on changes
- [x] Add unit tests for config persistence (≥85% coverage)
- [x] Validate: Configuration survives application restart

## 4. Tray UI Integration

- [x] Create basic tray UI structure (platform-specific)
- [x] Add interface selection dropdown
- [x] Add port configuration inputs
- [x] Add role assignment selectors
- [x] Add enable/disable toggles
- [x] Validate: UI displays correctly and interactions work

## 5. Integration and Testing

- [x] Create integration test for full flow (detect → configure → persist → load)
- [x] Test with loopback-only configuration
- [x] Test with LAN exposure
- [x] Run `cargo clippy` and fix all warnings
- [x] Run `cargo fmt` and ensure code style compliance
- [x] Validate: All tests pass, coverage ≥85%

## Dependencies

- Task 2 depends on Task 1 (need interface info to bind ports)
- Task 3 can be developed in parallel with Tasks 1-2
- Task 4 depends on Tasks 1-3 (UI needs data layer)
- Task 5 is final validation after all implementation
