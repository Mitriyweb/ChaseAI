# menu-structure Specification

## Purpose

Defines the structure and organization of the ChaseAI system tray interface, including menu items, submenus, and visual indicators. The interface is designed to work across macOS, Windows, and Linux platforms.

## Requirements

### Requirement: Tray Icon Presence

The system MUST display a persistent icon in the system tray.

#### Scenario: Icon Visibility

- **GIVEN** the application is running
- **WHEN** the user looks at the system tray
- **THEN** a ChaseAI icon MUST be visible
  - macOS: in the menu bar (near the clock)
  - Windows: in the system tray (bottom right)
  - Linux: in the system tray (varies by desktop environment)
- **AND** the icon MUST be appropriate for the platform (monochrome for macOS, color for Windows/Linux)

### Requirement: Status Display

The menu MUST display the current application status.

#### Scenario: Display Status

- **GIVEN** the application is running
- **WHEN** the user opens the tray menu
- **THEN** the first item MUST show "ChaseAI: Running" or "ChaseAI: Stopped"
- **AND** the second item MUST show the current IP and interface type

### Requirement: Active Interface Display

The menu MUST clearly display the currently active network interface.

#### Scenario: Display Current IP

- **GIVEN** an interface is bound in configuration
- **WHEN** the user opens the tray menu
- **THEN** the system MUST display the current IP and interface name (e.g., "IP: 127.0.0.1 (Loopback)")

### Requirement: Port Listing with Visual Indicators

The menu MUST list all configured ports with clear visual status indicators.

#### Scenario: Visual Port Status

- **GIVEN** one or more port bindings are configured
- **WHEN** the user opens the tray menu
- **THEN** each port MUST be displayed as: `[status] [port] • [role]`
  - Where status is ● (filled circle) for enabled or ○ (empty circle) for disabled
  - Example: "● 3000 • Instruction"
- **AND** clicking on a port MUST open a submenu with:
  - "Enable" or "Disable" action
  - "Change Role" submenu with role options

### Requirement: Port Management Submenu

The menu MUST provide a submenu for adding and removing ports.

#### Scenario: Port Management Options

- **GIVEN** the application is running
- **WHEN** the user opens "Manage Ports" submenu
- **THEN** the submenu MUST contain:
  - "Add New Port..." - opens dialog to configure new port
  - Separator
  - "Remove Port X" for each configured port

### Requirement: Submenu for Interface Selection

The system MUST provide a submenu to switch between available interfaces.

#### Scenario: List Available Interfaces

- **GIVEN** multiple interfaces are detected by the system
- **WHEN** the user hovers over "Select Interface"
- **THEN** a submenu MUST list all available interfaces with their IP addresses
- **AND** the current selection MUST be marked with a checkmark
- **AND** selecting an interface MUST update all port bindings

### Requirement: Service Control Commands

The menu MUST include commands to control all services at once.

#### Scenario: Bulk Service Control

- **GIVEN** multiple ports are configured
- **WHEN** the user opens the tray menu
- **THEN** the menu MUST include:
  - "Enable All Services" - enables all ports
  - "Disable All Services" - disables all ports

### Requirement: Exit Command

The menu MUST include a command to terminate the application.

#### Scenario: Application Shutdown

- **GIVEN** the application is running
- **WHEN** the user selects "Quit ChaseAI" menu item
- **THEN** the application MUST shut down gracefully
- **AND** release all network resources
- **AND** save current configuration

### Requirement: Menu Organization

The menu MUST follow a logical organization pattern.

#### Scenario: Menu Structure

- **GIVEN** the application is running
- **WHEN** the user opens the tray menu
- **THEN** the menu MUST be organized as:
  1. Status section (ChaseAI status, IP address)
  2. Separator
  3. Interface selection submenu
  4. Separator
  5. Ports section (list of ports or "No ports configured")
  6. Separator
  7. Port management submenu
  8. Separator
  9. Service control commands (Enable/Disable All)
  10. Separator
  11. Quit command
