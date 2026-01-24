# Menu Structure Specification

## ADDED Requirements

### Requirement: Tray Icon Presence

The system MUST display a persistent icon in the macOS menu bar.

#### Scenario: Icon Visibility

- **GIVEN** the application is running on macOS
- **WHEN** the user looks at the system menu bar
- **THEN** a ChaseAI icon MUST be visible

### Requirement: Active Interface Display

The menu MUST clearly display the currently active network interface.

#### Scenario: Display Current IP

- **GIVEN** an interface is bound in `network-interface-management`
- **WHEN** the user opens the tray menu
- **THEN** the system MUST display the current IP and interface name (e.g., "Active IP: 127.0.0.1 (Loopback)")

### Requirement: Port Listing

The menu MUST list all configured ports and their assigned roles.

#### Scenario: Visual Port Status

- **GIVEN** one or more port bindings are configured
- **WHEN** the user opens the tray menu
- **THEN** each port MUST be listed with its assigned role (e.g., "Port 3000: Instruction")
- **AND** the item MUST indicate if the port is currently enabled or disabled

### Requirement: Submenu for Interface Selection

The system MUST provide a submenu to switch between available interfaces.

#### Scenario: List Available Interfaces

- **GIVEN** multiple interfaces are detected by the system
- **WHEN** the user hovers over "Select Interface"
- **THEN** a submenu MUST list all available IP addresses
- **AND** the current selection MUST be marked with a checkmark

### Requirement: Exit Command

The menu MUST include a command to terminate the application.

#### Scenario: Application Shutdown

- **GIVEN** the application is running
- **WHEN** the user selects the "Exit" or "Quit" menu item
- **THEN** the application MUST shut down gracefully
- **AND** release all network resources
