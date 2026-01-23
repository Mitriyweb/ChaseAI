# Tasks: Network Interface Management

## 1. Network Interface Detection

- [ ] Create `src/rs/network/interface_detector.rs` module
- [ ] Implement loopback interface detection
- [ ] Implement LAN interface detection
- [ ] Implement public IP detection (optional)
- [ ] Add unit tests for interface detection (≥85% coverage)
- [ ] Validate: Run `cargo test` and verify all interface types are detected

## 2. Port Binding Configuration

- [ ] Create `src/rs/network/port_config.rs` module
- [ ] Define PortRole enum (Instruction, Verification, Workflow)
- [ ] Implement port binding validation (check if port is available)
- [ ] Add port-to-interface binding logic
- [ ] Add unit tests for port configuration (≥85% coverage)
- [ ] Validate: Verify ports can be bound to specific interfaces

## 3. Configuration Persistence

- [ ] Create `src/rs/config/network_config.rs` module
- [ ] Implement configuration serialization (JSON/TOML)
- [ ] Implement configuration loading on startup
- [ ] Implement configuration save on changes
- [ ] Add unit tests for config persistence (≥85% coverage)
- [ ] Validate: Configuration survives application restart

## 4. Tray UI Integration

- [ ] Create basic tray UI structure (platform-specific)
- [ ] Add interface selection dropdown
- [ ] Add port configuration inputs
- [ ] Add role assignment selectors
- [ ] Add enable/disable toggles
- [ ] Validate: UI displays correctly and interactions work

## 5. Integration and Testing

- [ ] Create integration test for full flow (detect → configure → persist → load)
- [ ] Test with loopback-only configuration
- [ ] Test with LAN exposure
- [ ] Run `cargo clippy` and fix all warnings
- [ ] Run `cargo fmt` and ensure code style compliance
- [ ] Validate: All tests pass, coverage ≥85%

## Dependencies

- Task 2 depends on Task 1 (need interface info to bind ports)
- Task 3 can be developed in parallel with Tasks 1-2
- Task 4 depends on Tasks 1-3 (UI needs data layer)
- Task 5 is final validation after all implementation
