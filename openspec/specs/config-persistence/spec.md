# config-persistence Specification

## Purpose

TBD - created by archiving change network-interface-management. Update Purpose after archive.

## Requirements

### Requirement: Configuration File Format

The system MUST persist network configuration in a structured, human-readable format.

#### Scenario: TOML Format

- **GIVEN** network configuration needs to be saved
- **WHEN** writing configuration to disk
- **THEN** the system MUST use TOML format
- **AND** the file MUST have `.toml` extension

#### Scenario: Configuration Structure

- **GIVEN** a configuration file is created
- **WHEN** inspecting the file structure
- **THEN** it MUST include:
  - `default_interface` field (string)
  - `port_bindings` array of binding objects
- **AND** each binding MUST include: `port`, `interface_name`, `interface_ip`, `role` `enabled`

### Requirement: Configuration File Location

The system MUST store configuration in a platform-appropriate location.

#### Scenario: macOS Configuration Path

- **GIVEN** the system is running on macOS
- **WHEN** determining config file location
- **THEN** the system MUST use `~/.config/chaseai/network.toml`

#### Scenario: Linux Configuration Path

- **GIVEN** the system is running on Linux
- **WHEN** determining config file location
- **THEN** the system MUST use `~/.config/chaseai/network.toml`

#### Scenario: Windows Configuration Path

- **GIVEN** the system is running on Windows
- **WHEN** determining config file location
- **THEN** the system MUST use `%APPDATA%\ChaseAI\network.toml`

#### Scenario: Create Config Directory

- **GIVEN** the config directory does not exist
- **WHEN** saving configuration
- **THEN** the system MUST create the directory structure
- **AND** set appropriate permissions (user-only read/write)

### Requirement: Configuration Loading

The system MUST load configuration on application startup.

#### Scenario: Load Existing Configuration

- **GIVEN** a valid configuration file exists
- **WHEN** the application starts
- **THEN** the system MUST load the configuration
- **AND** apply the loaded port bindings and interface settings

#### Scenario: No Configuration File

- **GIVEN** no configuration file exists (first run)
- **WHEN** the application starts
- **THEN** the system MUST use secure defaults
- **AND** default to loopback interface
- **AND** NOT create an error

#### Scenario: Invalid Configuration File

- **GIVEN** the configuration file is corrupted or invalid
- **WHEN** loading configuration
- **THEN** the system MUST log a warning
- **AND** fall back to secure defaults
- **AND** NOT crash the application

### Requirement: Configuration Saving

The system MUST persist configuration changes to disk.

#### Scenario: Save After Changes

- **GIVEN** port bindings have been modified
- **WHEN** user saves configuration
- **THEN** the system MUST write all current bindings to disk
- **AND** preserve the file format

#### Scenario: Atomic Write

- **GIVEN** configuration is being saved
- **WHEN** writing to disk
- **THEN** the system SHOULD use atomic write operations
- **AND** avoid partial writes that could corrupt the file

#### Scenario: Backup on Overwrite

- **GIVEN** a valid configuration file exists
- **WHEN** saving new configuration
- **THEN** the system MAY create a backup of the previous configuration
- **AND** name it with timestamp (e.g., `network.toml.backup.20260122`)

### Requirement: Configuration Validation

The system MUST validate configuration after loading.

#### Scenario: Validate Interface Availability

- **GIVEN** configuration specifies an interface binding
- **WHEN** loading configuration
- **THEN** the system MUST verify the interface still exists
- **AND** warn if interface is no longer available
- **AND** disable bindings for missing interfaces

#### Scenario: Validate Port Conflicts

- **GIVEN** loaded configuration contains port bindings
- **WHEN** validating configuration
- **THEN** the system MUST check for duplicate port numbers
- **AND** resolve conflicts by disabling later bindings

### Requirement: Configuration Migration

The system MUST handle configuration format changes gracefully.

#### Scenario: Version Field

- **GIVEN** configuration file format
- **WHEN** writing configuration
- **THEN** the system SHOULD include a version field
- **AND** use it for future migration

#### Scenario: Unknown Fields

- **GIVEN** configuration contains unknown fields (from future version)
- **WHEN** loading configuration
- **THEN** the system MUST ignore unknown fields
- **AND** preserve them when saving (forward compatibility)

### Requirement: Security

The system MUST ensure configuration file security.

#### Scenario: File Permissions

- **GIVEN** configuration file is created
- **WHEN** setting file permissions
- **THEN** the system MUST restrict access to owner only (chmod 600 on Unix)

#### Scenario: Sensitive Data Protection

- **GIVEN** configuration may be extended in future
- **WHEN** designing the format
- **THEN** the system MUST NOT store credentials or secrets in this file
- **NOTE**: Future authentication configs should use separate secure storage
