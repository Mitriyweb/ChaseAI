# interface-detection Specification

## Purpose

TBD - created by archiving change network-interface-management. Update Purpose after archive.

## Requirements

### Requirement: Loopback Interface Detection

The system MUST detect the loopback network interface for local-only communication.

#### Scenario: Detect Loopback on macOS

- **GIVEN** the system is running on macOS
- **WHEN** interface detection is performed
- **THEN** the system MUST identify the loopback interface (typically `lo0` with IP `127.0.0.1`)

#### Scenario: Detect Loopback on Linux

- **GIVEN** the system is running on Linux
- **WHEN** interface detection is performed
- **THEN** the system MUST identify the loopback interface (typically `lo` with IP `127.0.0.1`)

#### Scenario: Loopback Interface Always Available

- **GIVEN** any supported platform
- **WHEN** interface detection is performed
- **THEN** at least one loopback interface MUST be detected
- **AND** it MUST be marked as `InterfaceType::Loopback`

### Requirement: LAN Interface Detection

The system MUST detect available LAN (Local Area Network) interfaces.

#### Scenario: Detect Active LAN Interfaces

- **GIVEN** the system has active network connections
- **WHEN** interface detection is performed
- **THEN** the system MUST list all LAN interfaces with their IP addresses
- **AND** each interface MUST be marked as `InterfaceType::Lan`

#### Scenario: Multiple LAN Interfaces

- **GIVEN** the system has multiple network adapters (e.g., WiFi and Ethernet)
- **WHEN** interface detection is performed
- **THEN** the system MUST detect ALL active LAN interfaces
- **AND** provide their distinct IP addresses

#### Scenario: No Active LAN Connection

- **GIVEN** the system has no active LAN connections
- **WHEN** interface detection is performed
- **THEN** the system MUST return an empty list for LAN interfaces
- **AND** NOT return an error

### Requirement: Public IP Detection Support

The system MUST support public IP detection when explicitly requested by the user.

#### Scenario: Public IP Available

- **GIVEN** the system has direct public IP assignment
- **WHEN** interface detection is performed
- **THEN** the system MAY identify the public IP
- **AND** mark it as `InterfaceType::Public`

#### Scenario: Public IP Behind NAT

- **GIVEN** the system is behind NAT/firewall
- **WHEN** interface detection is performed
- **THEN** the system MUST NOT incorrectly label LAN IPs as public
- **AND** SHOULD return `None` for public IP

### Requirement: Interface Information Completeness

Each detected interface MUST provide complete identification information.

#### Scenario: Interface Data Structure

- **GIVEN** an interface is detected
- **WHEN** interface information is retrieved
- **THEN** it MUST include:
  - Interface name (e.g., `lo0`, `en0`)
  - IP address (IPv4 or IPv6)
  - Interface type (Loopback, LAN, Public)

#### Scenario: IPv4 Support

- **GIVEN** the system has IPv4 interfaces
- **WHEN** interface detection is performed
- **THEN** IPv4 addresses MUST be correctly parsed and stored

#### Scenario: IPv6 Support (Future)

- **GIVEN** the system has IPv6 interfaces
- **WHEN** interface detection is performed
- **THEN** IPv6 addresses SHOULD be correctly parsed and stored
- **NOTE**: Full IPv6 support is not required for MVP but should be architecturally possible
