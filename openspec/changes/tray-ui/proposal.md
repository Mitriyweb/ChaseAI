# Proposal: System Tray UI (macOS)

## Summary

Implement the System Tray UI for ChaseAI on macOS. This component provides the visual interface for users to monitor available network interfaces, configure port roles, manage instruction contexts, and control the instruction service state (Enable/Disable).

## Rationale

According to the architecture principles of ChaseAI, the user must be the "governor" of the system. A tray-based UI is the most native and efficient way to provide this control on a local system without requiring a persistent browser window.

- **Immediate Visibility**: Users can see if the instruction service is active at a glance.
- **Quick Access**: Configuration of ports and interfaces is accessible with a single click.
- **Status Monitoring**: Visual feedback for bound ports and active contexts.

## Scope

### In Scope

- MacOS System Tray icon and menu structure.
- Interface Selection: Menu item to choose between Loopback, LAN, or Public interfaces.
- Port Status: Display current port bindings and their roles.
- Role Management: Interface to assign roles (Instruction, Verification, Workflow) to ports.
- Service Control: Global Enable/Disable switch for the instruction service.
- Integration: Communication with `network-interface-management` and `instruction-service` modules.

### Out of Scope

- Sophisticated settings window (MVP focuses on menu-based control).
- Multi-platform UI (this proposal focuses on macOS; Linux/Windows tray implementation is deferred).
- Log viewer (deferred to future phase).

## Dependencies

- **Depends on**: `network-interface-management` (for interface and port data).
- **Depends on**: `instruction-service` (for starting/stopping the server based on UI switches).

## User-Facing Impact

The user will see a ChaseAI icon in the macOS menu bar. Clicking it will reveal:

1. Current active IP and interface name.
2. List of ports with their assigned roles and status.
3. Submenus to change IP/Interface.
4. Toggles to enable/disable specific ports or the whole service.
5. Exit button to shut down the application.

## Success Criteria

- [ ] Tray icon appears in macOS menu bar on application start.
- [ ] Menu correctly lists all interfaces detected by `network-interface-management`.
- [ ] Changing IP in the menu updates the backend configuration.
- [ ] Port roles can be toggled through the menu.
- [ ] Service enable/disable switch correctly starts/stops the local instruction servers.
- [ ] UI reflects changes made to the configuration files in real-time.
