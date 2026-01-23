# Context Model Specification

## ADDED Requirements

### Requirement: Instruction Context Structure

The system MUST define a structured data model for instruction contexts.

#### Scenario: Context Fields

- **GIVEN** an instruction context is created
- **WHEN** inspecting the context structure
- **THEN** it MUST contain the following fields:
  - `system` (string) - System identifier
  - `role` (string) - Agent role
  - `base_instruction` (string) - Base instruction text
  - `allowed_actions` (array of strings) - Allowed action types
  - `verification_required` (boolean) - Whether verification is required

#### Scenario: Context Serialization

- **GIVEN** an instruction context exists
- **WHEN** serializing to JSON
- **THEN** all fields MUST be included in the output
- **AND** the JSON MUST be valid and parseable

#### Scenario: Context Deserialization

- **GIVEN** a valid JSON representation of a context
- **WHEN** deserializing from JSON
- **THEN** the system MUST reconstruct the context object
- **AND** all field values MUST match the original

### Requirement: Context Validation

The system MUST validate instruction contexts before accepting them.

#### Scenario: System Field Validation

- **GIVEN** an instruction context is being created
- **WHEN** the `system` field is empty
- **THEN** the system MUST reject the context
- **AND** return a validation error

#### Scenario: Role Field Validation

- **GIVEN** an instruction context is being created
- **WHEN** the `role` field is empty
- **THEN** the system MUST reject the context
- **AND** return a validation error

#### Scenario: Base Instruction Validation

- **GIVEN** an instruction context is being created
- **WHEN** the `base_instruction` field is empty
- **THEN** the system MUST reject the context
- **AND** return a validation error

#### Scenario: Allowed Actions Validation

- **GIVEN** an instruction context is being created
- **WHEN** the `allowed_actions` array is empty
- **THEN** the system MUST reject the context
- **AND** return a validation error

#### Scenario: Action Name Format

- **GIVEN** an instruction context with allowed actions
- **WHEN** validating action names
- **THEN** each action MUST match the pattern `^[a-z][a-z0-9-]*$`
- **AND** invalid action names MUST be rejected

### Requirement: Context Management Operations

The system MUST support CRUD operations for instruction contexts.

#### Scenario: Create Context

- **GIVEN** a valid instruction context and port number
- **WHEN** creating the context
- **THEN** the system MUST associate the context with the port
- **AND** persist the context

#### Scenario: Read Context

- **GIVEN** a context exists for port 3000
- **WHEN** requesting the context for port 3000
- **THEN** the system MUST return the associated context

#### Scenario: Update Context

- **GIVEN** a context exists for a port
- **WHEN** updating the context with new values
- **THEN** the system MUST replace the old context
- **AND** persist the changes

#### Scenario: Delete Context

- **GIVEN** a context exists for a port
- **WHEN** deleting the context
- **THEN** the system MUST remove the context
- **AND** the port MUST no longer have an associated context

#### Scenario: List All Contexts

- **GIVEN** multiple contexts exist
- **WHEN** requesting all contexts
- **THEN** the system MUST return a list of all port-context pairs

### Requirement: Port Validation

The system MUST validate ports against network configuration.

#### Scenario: Port Exists in Network Config

- **GIVEN** port 3000 is configured in network-interface-management
- **WHEN** creating a context for port 3000
- **THEN** the system MUST allow the operation

#### Scenario: Port Not in Network Config

- **GIVEN** port 9999 is NOT configured in network-interface-management
- **WHEN** attempting to create a context for port 9999
- **THEN** the system MUST reject the operation
- **AND** return an error indicating the port is not configured

### Requirement: Context Persistence

The system MUST persist instruction contexts to disk.

#### Scenario: Save Contexts

- **GIVEN** contexts have been created or modified
- **WHEN** saving contexts
- **THEN** the system MUST write all contexts to disk in JSON format

#### Scenario: Load Contexts on Startup

- **GIVEN** contexts have been previously saved
- **WHEN** the application starts
- **THEN** the system MUST load all contexts from disk
- **AND** make them available for use

#### Scenario: No Contexts File

- **GIVEN** no contexts file exists (first run)
- **WHEN** loading contexts
- **THEN** the system MUST NOT return an error
- **AND** start with an empty context collection

#### Scenario: Corrupted Contexts File

- **GIVEN** the contexts file is corrupted or invalid
- **WHEN** loading contexts
- **THEN** the system MUST log a warning
- **AND** start with an empty context collection
- **AND** NOT crash the application
