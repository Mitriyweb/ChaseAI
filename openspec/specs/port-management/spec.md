# port-management Specification

## Purpose

TBD - created by archiving change network-interface-management. Update Purpose after archive.

## Requirements

### Requirement: Port Role Assignment

The system MUST support assigning specific roles to ports.

#### Scenario: Define Port Roles

- **GIVEN** the port management system
- **WHEN** configuring a port binding
- **THEN** the system MUST support three role types:
  - `Instruction` - for instruction service endpoints
  - `Verification` - for verification service endpoints
  - `Workflow` - for workflow management endpoints

#### Scenario: Assign Role to Port

- **GIVEN** a valid port number
- **WHEN** creating a port binding
- **THEN** the user MUST be able to assign exactly one role to that port

#### Scenario: Multiple Ports Same Role

- **GIVEN** multiple port bindings exist
- **WHEN** assigning roles
- **THEN** multiple ports MAY have the same role
- **AND** this MUST be allowed for horizontal scaling or redundancy

### Requirement: Port Binding Configuration

The system MUST allow binding ports to specific network interfaces.

#### Scenario: Bind Port to Interface

- **GIVEN** a detected network interface
- **AND** a valid port number
- **WHEN** creating a port binding
- **THEN** the system MUST associate the port with the specified interface
- **AND** store this binding for later use

#### Scenario: Bind to Loopback Only

- **GIVEN** security-first configuration
- **WHEN** user creates a port binding
- **THEN** the system MUST allow binding to loopback interface only
- **AND** this SHOULD be the default configuration

#### Scenario: Bind to LAN Interface

- **GIVEN** a LAN interface is available
- **WHEN** user explicitly chooses LAN exposure
- **THEN** the system MUST allow binding to the selected LAN interface
- **AND** warn the user about security implications

### Requirement: Port Validation

The system MUST validate port configurations before allowing bindings.

#### Scenario: Valid Port Range

- **GIVEN** a port binding is being created
- **WHEN** the port number is provided
- **THEN** the system MUST reject ports below 1024 (privileged ports)
- **AND** MUST reject ports above 65535
- **AND** MUST accept ports in range 1024-65535

#### Scenario: Port Availability Check

- **GIVEN** a port binding is being created
- **WHEN** validating the port
- **THEN** the system SHOULD check if the port is already in use
- **AND** warn the user if the port may conflict

#### Scenario: Duplicate Port Prevention

- **GIVEN** a port binding already exists for port N
- **WHEN** attempting to create another binding for port N
- **THEN** the system MUST reject the duplicate binding
- **AND** provide a clear error message

### Requirement: Port Binding Lifecycle

The system MUST support enable/disable state for port bindings.

#### Scenario: Enable Port Binding

- **GIVEN** a port binding exists
- **WHEN** the binding is set to enabled
- **THEN** the system MUST mark it as active
- **AND** it SHOULD be available for service binding (in instruction-service)

#### Scenario: Disable Port Binding

- **GIVEN** an enabled port binding
- **WHEN** the binding is disabled
- **THEN** the system MUST mark it as inactive
- **AND** it MUST NOT be used for service binding
- **AND** the configuration MUST persist the disabled state

#### Scenario: Remove Port Binding

- **GIVEN** a port binding exists
- **WHEN** user removes the binding
- **THEN** the system MUST delete the binding from configuration
- **AND** free the port for other use

### Requirement: Configuration Query Interface

The system MUST provide interface to query port bindings.

#### Scenario: Get Binding by Port

- **GIVEN** port bindings exist
- **WHEN** querying for a specific port number
- **THEN** the system MUST return the binding if it exists
- **OR** return `None` if no binding exists for that port

#### Scenario: List All Bindings

- **GIVEN** multiple port bindings exist
- **WHEN** requesting all bindings
- **THEN** the system MUST return complete list of all configured bindings
- **AND** include both enabled and disabled bindings

#### Scenario: Filter Bindings by Role

- **GIVEN** multiple port bindings with different roles
- **WHEN** filtering by a specific role (e.g., Instruction)
- **THEN** the system MUST return only bindings with that role
