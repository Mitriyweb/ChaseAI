# Tasks: Instruction Service

## 1. Instruction Context Data Model

- [ ] Create `src/rs/instruction/context.rs` module
- [ ] Define `InstructionContext` struct with fields:
  - `system` (String)
  - `role` (String)
  - `base_instruction` (String)
  - `allowed_actions` (Vec<String>)
  - `verification_required` (bool)
- [ ] Implement validation for context fields
- [ ] Add unit tests for context creation and validation (≥85% coverage)
- [ ] Validate: Context struct works as expected

## 2. Context Management Layer

- [ ] Create `src/rs/instruction/manager.rs` module
- [ ] Implement `ContextManager` with CRUD operations:
  - `create_context(port: u16, context: InstructionContext)`
  - `get_context(port: u16) -> Option<InstructionContext>`
  - `update_context(port: u16, context: InstructionContext)`
  - `delete_context(port: u16)`
  - `list_contexts() -> Vec<(u16, InstructionContext)>`
- [ ] Add validation to ensure port exists in network config
- [ ] Add unit tests for manager operations (≥85% coverage)
- [ ] Validate: CRUD operations work correctly

## 3. Context Persistence

- [ ] Create `src/rs/instruction/storage.rs` module
- [ ] Implement context serialization to JSON/TOML
- [ ] Implement context loading on startup
- [ ] Implement auto-save on context changes
- [ ] Add unit tests for persistence (≥85% coverage)
- [ ] Validate: Contexts survive application restart

## 4. HTTP/TCP Server Implementation

- [ ] Create `src/rs/server/instruction_server.rs` module
- [ ] Implement basic HTTP server (using `axum` or `actix-web`)
- [ ] Add GET endpoint: `/context` returns instruction context as JSON
- [ ] Bind server to configured port and interface (from network config)
- [ ] Handle multiple servers (one per port binding)
- [ ] Add error handling for invalid requests
- [ ] Add unit tests for server logic (≥85% coverage)
- [ ] Validate: Server responds correctly on configured ports

## 5. Integration with Network Configuration

- [ ] Integrate with network-interface-management module
- [ ] On startup, create servers for all enabled port bindings
- [ ] Handle dynamic port enable/disable from network config
- [ ] Add validation to prevent contexts on non-existent ports
- [ ] Add integration test for full stack (network + instruction)
- [ ] Validate: Servers correctly bind to network interfaces

## 6. Validation and Testing

- [ ] Create integration test: create context → start server → query endpoint → verify response
- [ ] Test with loopback interface (curl http://127.0.0.1:3000/context)
- [ ] Test with LAN interface
- [ ] Test error cases (invalid port, missing context, etc.)
- [ ] Run `cargo clippy` and fix all warnings
- [ ] Run `cargo fmt` and ensure code style compliance
- [ ] Validate: All tests pass, coverage ≥85%

## Dependencies

- Task 1 is foundational (context data model)
- Task 2 depends on Task 1 (manager uses context)
- Task 3 depends on Tasks 1-2 (persistence uses context and manager)
- Task 4 can be developed in parallel with Tasks 1-3
- Task 5 depends on all previous tasks (integration)
- Task 6 is final validation
