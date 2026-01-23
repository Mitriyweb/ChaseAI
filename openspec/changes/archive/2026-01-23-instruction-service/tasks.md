# Tasks: Instruction Service

## 1. Instruction Context Data Model

- [x] Create `src/rs/instruction/context.rs` module
- [x] Define `InstructionContext` struct with fields:
  - `system` (String)
  - `role` (String)
  - `base_instruction` (String)
  - `allowed_actions` (`Vec<String>`)
  - `verification_required` (bool)
- [x] Implement validation for context fields
- [x] Add unit tests for context creation and validation (≥85% coverage)
- [x] Validate: Context struct works as expected

## 2. Context Management Layer

- [x] Create `src/rs/instruction/manager.rs` module
- [x] Implement `ContextManager` with CRUD operations:
  - `create_context(port: u16, context: InstructionContext)`
  - `get_context(port: u16) -> Option<InstructionContext>`
  - `update_context(port: u16, context: InstructionContext)`
  - `delete_context(port: u16)`
  - `list_contexts() -> Vec<(u16, InstructionContext)>`
- [x] Add validation to ensure port exists in network config
- [x] Add unit tests for manager operations (≥85% coverage)
- [x] Validate: CRUD operations work correctly

## 3. Context Persistence

- [x] Create `src/rs/instruction/storage.rs` module
- [x] Implement context serialization to JSON/TOML
- [x] Implement context loading on startup
- [x] Implement auto-save on context changes
- [x] Add unit tests for persistence (≥85% coverage)
- [x] Validate: Contexts survive application restart

## 4. HTTP/TCP Server Implementation

- [x] Create `src/rs/server/instruction_server.rs` module
- [x] Implement basic HTTP server (using `axum` or `actix-web`)
- [x] Add GET endpoint: `/context` returns instruction context as JSON
- [x] Bind server to configured port and interface (from network config)
- [x] Handle multiple servers (one per port binding)
- [x] Add error handling for invalid requests
- [x] Add unit tests for server logic (≥85% coverage)
- [x] Validate: Server responds correctly on configured ports

## 5. Integration with Network Configuration

- [x] Integrate with network-interface-management module
- [x] On startup, create servers for all enabled port bindings
- [x] Handle dynamic port enable/disable from network config
- [x] Add validation to prevent contexts on non-existent ports
- [x] Add integration test for full stack (network + instruction)
- [x] Validate: Servers correctly bind to network interfaces

## 6. Validation and Testing

- [x] Create integration test: create context → start server → query endpoint → verify response
- [x] Test with loopback interface (curl <http://127.0.0.1:3000/context>)
- [x] Test with LAN interface
- [x] Test error cases (invalid port, missing context, etc.)
- [x] Run `cargo clippy` and fix all warnings
- [x] Run `cargo fmt` and ensure code style compliance
- [x] Validate: All tests pass, coverage ≥85%

## Dependencies

- Task 1 is foundational (context data model)
- Task 2 depends on Task 1 (manager uses context)
- Task 3 depends on Tasks 1-2 (persistence uses context and manager)
- Task 4 can be developed in parallel with Tasks 1-3
- Task 5 depends on all previous tasks (integration)
- Task 6 is final validation
