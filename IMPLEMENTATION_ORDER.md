# Implementation Order: ChaseAI MVP

To ensure a smooth development process without broken dependencies, the MVP components should be implemented in the following order:

## 1. Network Interface Management (`network-interface-management`)

**Status**: Foundation
**Why First**: This module defines the core data structures (`NetworkConfig`, `PortBinding`) and the logic for IP detection. Other modules cannot function without knowing which interfaces and ports are available.

- **Outcome**: `network.toml` exists and populated with local interfaces.

---

## 2. Instruction Service (`instruction-service`)

**Status**: Core Logic
**Why Second**: This module implements the actual "work" of ChaseAI (serving contexts). It consumes the configuration provided by NIM. While it can run "headless", it provides the functional value of the service.

- **Outcome**: Local HTTP servers start on ports defined in the NIM config.

---

## 3. System Tray UI (`tray-ui`)

**Status**: Control Plane
**Why Last**: The Tray UI is the "orchestrator" for the user. It needs both the NIM (to show what to configure) and the IS (to provide something to turn on/off). It acts as the final integration layer that wraps the backend services into a Mac application.

- **Outcome**: User can control NIM and IS via the macOS menu bar.

## Summary Checklist

1. [ ] Apply `network-interface-management`
2. [ ] Apply `instruction-service`
3. [ ] Apply `tray-ui`
