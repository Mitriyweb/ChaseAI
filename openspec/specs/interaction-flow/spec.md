# interaction-flow Specification

## Purpose

Defines the user interaction patterns for the ChaseAI system tray application, including port management, interface switching, and service control. The interactions are designed to work consistently across macOS, Windows, and Linux platforms.

## Requirements

### Requirement: Port Management Interaction

The system MUST allow users to add, remove, and configure ports through the menu interface.

#### Scenario: Add New Port

- **GIVEN** the user opens "Manage Ports" submenu
- **WHEN** the user selects "Add New Port..."
- **THEN** the system MUST display a dialog to input port number, role, and enabled state
- **AND** add the new port to the configuration upon confirmation
- **AND** refresh the menu to show the new port

#### Scenario: Remove Port

- **GIVEN** one or more ports are configured
- **WHEN** the user selects "Remove Port X" from "Manage Ports"
- **THEN** the system MUST remove the port from configuration
- **AND** stop any running server on that port
- **AND** refresh the menu

#### Scenario: Change Port Role

- **GIVEN** a port is configured
- **WHEN** the user selects a new role from "Change Role" submenu
- **THEN** the system MUST update the port's role in configuration
- **AND** restart the server with the new role if enabled

### Requirement: Interface Switching Interaction

The system MUST update the configuration when an interface is selected from the menu.

#### Scenario: Switch Interface

- **GIVEN** the tray menu is open
- **WHEN** the user selects a new IP address from the "Select Interface" submenu
- **THEN** the system MUST update the `network.toml` configuration
- **AND** update all port bindings to use the new interface
- **AND** the menu MUST refresh to show the new IP as active

### Requirement: Port Toggle Interaction

The system MUST allow enabling or disabling specific port servers from the menu.

#### Scenario: Toggle Port State

- **GIVEN** a port is currently enabled (shown as ● in menu)
- **WHEN** the user clicks "Disable" in the port's submenu
- **THEN** the system MUST toggle the `enabled` state to false
- **AND** trigger the server pool to stop the server for that port
- **AND** update the menu to show ○ (disabled)

#### Scenario: Enable Port

- **GIVEN** a port is currently disabled (shown as ○ in menu)
- **WHEN** the user clicks "Enable" in the port's submenu
- **THEN** the system MUST toggle the `enabled` state to true
- **AND** trigger the server pool to start the server for that port
- **AND** update the menu to show ● (enabled)

### Requirement: Visual Status Indicators

The system MUST provide clear visual indicators for port status.

#### Scenario: Port Status Display

- **GIVEN** ports are configured
- **WHEN** the user opens the tray menu
- **THEN** each port MUST show:
  - ● (filled circle) if enabled
  - ○ (empty circle) if disabled
  - Port number
  - Role name (Instruction, Verification, or Workflow)

### Requirement: Visual Feedback for Actions

The system MUST provide immediate visual feedback for user actions.

#### Scenario: State Transition Feedback

- **GIVEN** the user performs an action (like switching interface)
- **WHEN** the transaction is complete
- **THEN** the menu MUST update its state immediately
- **AND** any error MUST be logged to console

### Requirement: External Configuration Sync

The UI MUST stay synchronized with external configuration changes.

#### Scenario: Sync with Config File

- **GIVEN** the `network.toml` file is modified by a CLI tool or manually
- **WHEN** the UI detects the file modification via file watcher
- **THEN** the tray menu MUST be rebuilt to match the new configuration state
- **AND** servers MUST be updated to match the new configuration

### Requirement: Service Lifecycle Control

The UI MUST allow controlling the entire instruction service suite.

#### Scenario: Enable All Services

- **GIVEN** multiple ports are configured
- **WHEN** the user selects "Enable All Services"
- **THEN** all individual port servers MUST be started
- **AND** the configuration MUST reflect that all services are enabled
- **AND** all ports MUST show ● indicator

#### Scenario: Disable All Services

- **GIVEN** multiple ports are active
- **WHEN** the user selects "Disable All Services"
- **THEN** all individual port servers MUST be stopped
- **AND** the configuration MUST reflect that all services are disabled
- **AND** all ports MUST show ○ indicator
