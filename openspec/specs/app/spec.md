# App Specification

## Purpose

To provide a secure, local-first control plane for AI agents, capable of managing network bindings, instruction contexts, and execution lifecycles through a consolidated application interface.

## Requirements

### Requirement: Custom Rust Structure

The ChaseAI core MUST be built in Rust with a non-standard directory structure to cleanly separate source and tests.

#### Scenario: Source Code Location

- **WHEN** the project is inspected
- **THEN** all Rust source files MUST reside in `src/rs/`

#### Scenario: Test Code Location

- **WHEN** integration tests are executed
- **THEN** the system MUST support tests located in `src/test/rs/`

### Requirement: Controlled Execution Baseline

The system MUST provide a basic library structure and an executable for controlled execution of AI-driven commands.

#### Scenario: Library Initialization

- **WHEN** the library is compiled
- **THEN** it MUST expose a core entry point in `app.rs` (configured in `Cargo.toml` as `[lib] name = "app"`)

#### Scenario: Executable Entry Point

- **WHEN** the application is run
- **THEN** it MUST execute the entry point defined in `src/rs/main.rs`

### Requirement: Cross-Platform Architecture

The system MUST support multiple operating systems through a modular platform abstraction layer.

#### Scenario: Platform Selection

- **WHEN** the application is compiled for a specific OS
- **THEN** the system MUST automatically select the appropriate platform module using conditional compilation (`#[cfg(target_os = "...")]`)

#### Scenario: macOS Implementation

- **WHEN** the application runs on macOS
- **THEN** the system MUST:
  - Initialize NSApplication with Accessory activation policy (menu bar only)
  - Create a native tray icon with menu
  - Use `tao` event loop for proper event handling
  - Display the application in the system menu bar

#### Scenario: Platform Extensibility

- **WHEN** adding support for a new operating system
- **THEN** the system MUST:
  - Create a new module in `src/rs/platform/{os_name}.rs`
  - Implement a `run()` function with the same signature
  - Update `src/rs/platform/mod.rs` to conditionally export the new module
  - Require no changes to `src/rs/main.rs`

#### Scenario: Unsupported Platform

- **WHEN** the application is compiled for an unsupported OS
- **THEN** the system MUST display an error message and exit gracefully
