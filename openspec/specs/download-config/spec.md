# download-config Specification

## Purpose

Enable users to download a machine-readable configuration file that AI agents can use to discover and integrate with ChaseAI's services.

## Requirements

### Requirement: Download Config Button

The tray menu MUST include a "Download Config" button that allows users to download the current configuration.

#### Scenario: Button Visibility

- **GIVEN** the ChaseAI application is running
- **WHEN** the user opens the tray menu
- **THEN** the menu MUST display a "Download Config" button
- **AND** the button MUST be positioned after "Settings" and before "Quit"

#### Scenario: Button Click

- **GIVEN** the "Download Config" button is visible
- **WHEN** the user clicks the button
- **THEN** the system MUST generate a configuration file
- **AND** trigger a file download dialog
- **AND** show a success notification

#### Scenario: File Download

- **GIVEN** the user clicks "Download Config"
- **WHEN** the file download is triggered
- **THEN** the file MUST be saved to the default Downloads directory
- **AND** the filename MUST include a timestamp (e.g., `chaseai-config-2026-01-28-10-30-00.json`)
- **AND** the file MUST be readable by the user

#### Scenario: Error Handling

- **GIVEN** the download fails (e.g., permission denied)
- **WHEN** the error occurs
- **THEN** the system MUST show an error notification
- **AND** include a helpful error message
- **AND** NOT crash the application

### Requirement: Configuration File Format

The downloaded configuration file MUST contain all information needed for AI agents to integrate with ChaseAI.

#### Scenario: JSON Configuration Structure

- **GIVEN** a configuration file is generated
- **WHEN** the file is in JSON format
- **THEN** it MUST include:
  - `version` - API version (e.g., "1.0.0")
  - `timestamp` - Generation timestamp (ISO 8601)
  - `application` - Application metadata (name, version, description)
  - `ports` - Array of port configurations
  - `endpoints` - Documentation of available endpoints
  - `documentation` - Links to documentation

#### Scenario: Port Configuration

- **GIVEN** multiple ports are configured
- **WHEN** the configuration file is generated
- **THEN** each port MUST include:
  - `port` - Port number
  - `interface` - Network interface (e.g., "127.0.0.1")
  - `role` - Port role (e.g., "instruction", "verification")
  - `enabled` - Whether the port is active
  - `endpoints` - List of available endpoints on this port

#### Scenario: Endpoint Documentation

- **GIVEN** endpoints are documented in the configuration
- **WHEN** the file is generated
- **THEN** each endpoint MUST include:
  - `path` - HTTP path (e.g., "/context")
  - `method` - HTTP method (e.g., "GET", "POST")
  - `description` - Human-readable description
  - `request` - Request format (if applicable)
  - `response` - Response format (if applicable)

#### Scenario: Multiple Format Support

- **GIVEN** a configuration file is requested
- **WHEN** the user specifies a format
- **THEN** the system MUST support:
  - JSON format (default)
  - YAML format (human-readable)
  - Markdown format (documentation)

### Requirement: Configuration Endpoint

The HTTP server MUST expose a `/config` endpoint that returns the current configuration.

#### Scenario: GET Config Endpoint

- **GIVEN** the instruction server is running on port 8090
- **WHEN** a client sends `GET /config` to the server
- **THEN** the server MUST respond with HTTP 200
- **AND** the response body MUST contain the configuration as JSON
- **AND** the `Content-Type` header MUST be `application/json`

#### Scenario: Format Parameter

- **GIVEN** the `/config` endpoint is available
- **WHEN** a client sends `GET /config?format=yaml`
- **THEN** the server MUST respond with HTTP 200
- **AND** the response body MUST be in YAML format
- **AND** the `Content-Type` header MUST be `application/yaml`

#### Scenario: Invalid Format

- **GIVEN** the `/config` endpoint is available
- **WHEN** a client sends `GET /config?format=invalid`
- **THEN** the server MUST respond with HTTP 400
- **AND** include an error message indicating the format is not supported

#### Scenario: Configuration Consistency

- **GIVEN** multiple requests to `/config` are made
- **WHEN** the requests are made in quick succession
- **THEN** all responses MUST contain the same configuration
- **AND** only the timestamp MAY differ

### Requirement: Configuration Content

The configuration file MUST contain accurate and complete information about ChaseAI's services.

#### Scenario: Port Mappings

- **GIVEN** ports are configured in the network configuration
- **WHEN** the configuration file is generated
- **THEN** all enabled ports MUST be included
- **AND** disabled ports MUST NOT be included
- **AND** each port MUST have the correct interface and role

#### Scenario: Endpoint Information

- **GIVEN** endpoints are available on the server
- **WHEN** the configuration file is generated
- **THEN** all endpoints MUST be documented
- **AND** each endpoint MUST have accurate method and path
- **AND** descriptions MUST be clear and helpful

#### Scenario: Version Information

- **GIVEN** a configuration file is generated
- **WHEN** the file is created
- **THEN** it MUST include:
  - API version (for compatibility checking)
  - Application version (for feature detection)
  - Timestamp (for freshness validation)

### Requirement: File Delivery

The configuration file MUST be delivered to the user in a usable format.

#### Scenario: File Naming

- **GIVEN** a configuration file is downloaded
- **WHEN** the file is saved
- **THEN** the filename MUST follow the pattern: `chaseai-config-YYYY-MM-DD-HH-MM-SS.json`
- **AND** the filename MUST be unique (no overwrites)

#### Scenario: File Permissions

- **GIVEN** a configuration file is downloaded
- **WHEN** the file is saved
- **THEN** the file MUST be readable by the user
- **AND** the file MUST NOT require elevated permissions

#### Scenario: File Size

- **GIVEN** a configuration file is generated
- **WHEN** the file is created
- **THEN** the file size MUST be reasonable (< 1 MB)
- **AND** the file MUST be efficiently formatted

### Requirement: Error Handling

The system MUST handle errors gracefully during configuration generation and download.

#### Scenario: Configuration Generation Error

- **GIVEN** an error occurs during configuration generation
- **WHEN** the error is encountered
- **THEN** the system MUST show an error notification
- **AND** include details about what went wrong
- **AND** NOT crash the application

#### Scenario: File System Error

- **GIVEN** the file system is unavailable
- **WHEN** the download is attempted
- **THEN** the system MUST show an error notification
- **AND** suggest alternative actions (e.g., retry, save to different location)

#### Scenario: Permission Denied

- **GIVEN** the user lacks write permissions to the Downloads directory
- **WHEN** the download is attempted
- **THEN** the system MUST show an error notification
- **AND** suggest checking file permissions
