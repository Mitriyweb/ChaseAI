# Interaction Flow Specification

## ADDED Requirements

### Requirement: Interface Switching Interaction

The system MUST update the configuration when an interface is selected from the menu.

#### Scenario: Switch Interface

- **GIVEN** the tray menu is open
- **WHEN** the user selects a new IP address from the "Select Interface" submenu
- **THEN** the system MUST update the `network.toml` configuration
- **AND** the menu MUST refresh to show the new IP as active

### Requirement: Port Toggle Interaction

The system MUST allow enabling or disabling specific port servers from the menu.

#### Scenario: Toggle Port State

- **GIVEN** a port is currently enabled
- **WHEN** the user clicks on the port item in the tray menu
- **THEN** the system MUST toggle the `enabled` state in the configuration
- **AND** trigger the `instruction-service` to stop the server for that port

### Requirement: Visual Feedback for Actions

The system MUST provide immediate visual feedback for user actions.

#### Scenario: State Transition Feedback

- **GIVEN** the user performs an action (like switching interface)
- **WHEN** the transaction is complete
- **THEN** the menu MUST update its state immediately
- **AND** any error MUST be displayed in the menu if the action fails

### Requirement: External Configuration Sync

The UI MUST stay synchronized with external configuration changes.

#### Scenario: Sync with Config File

- **GIVEN** the `network.toml` file is modified by a CLI tool or manually
- **WHEN** the UI detects the file modification
- **THEN** the tray menu MUST be rebuilt to match the new configuration state

### Requirement: Service Lifecycle Control

The UI MUST allow controlling the entire instruction service suite.

#### Scenario: Disable All Services

- **GIVEN** multiple ports are active
- **WHEN** the user selects "Disable All Services"
- **THEN** all individual port servers MUST be stopped
- **AND** the configuration MUST reflect that all services are off
