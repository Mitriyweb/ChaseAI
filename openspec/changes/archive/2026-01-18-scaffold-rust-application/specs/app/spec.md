## ADDED Requirements

### Requirement: Custom Rust Structure
The ChaseAI core MUST be built in Rust with a non-standard directory structure to cleanly separate source and tests.

#### Scenario: Source Code Location
- **WHEN** the project is inspected
- **THEN** all Rust source files MUST reside in `src/rs/`

#### Scenario: Test Code Location
- **WHEN** integration tests are executed
- **THEN** the system MUST support tests located in `src/test/rs/`

### Requirement: Controlled Execution Baseline
The system MUST provide a basic library structure that allows for controlled execution of AI-driven commands.

#### Scenario: Library Initialization
- **WHEN** the library is compiled
- **THEN** it MUST expose a core entry point in `lib.rs`
