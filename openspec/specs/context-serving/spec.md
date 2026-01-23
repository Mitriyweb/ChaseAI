# context-serving Specification

## Purpose

TBD - created by archiving change instruction-service. Update Purpose after archive.

## Requirements

### Requirement: HTTP Server per Port

The system MUST create an HTTP server for each enabled port binding.

#### Scenario: Server Creation

- **GIVEN** a port binding is enabled in network configuration
- **WHEN** the instruction service starts
- **THEN** the system MUST create an HTTP server for that port
- **AND** bind it to the configured network interface

#### Scenario: Multiple Servers

- **GIVEN** multiple port bindings are enabled
- **WHEN** the instruction service starts
- **THEN** the system MUST create a separate server for each port
- **AND** each server MUST operate independently

#### Scenario: Server Bound to Correct Interface

- **GIVEN** port 3000 is configured to bind to loopback (127.0.0.1)
- **WHEN** the server starts
- **THEN** it MUST bind to 127.0.0.1:3000
- **AND** NOT be accessible from other interfaces

### Requirement: Context Endpoint

The system MUST provide an HTTP endpoint to retrieve instruction contexts.

#### Scenario: GET Context Endpoint

- **GIVEN** a server is running on port 3000
- **AND** a context exists for port 3000
- **WHEN** a client sends `GET /context` to the server
- **THEN** the server MUST respond with HTTP 200
- **AND** the response body MUST contain the context as JSON

#### Scenario: Context JSON Format

- **GIVEN** a context is being served
- **WHEN** the JSON response is generated
- **THEN** it MUST include all context fields:
  - `system`
  - `role`
  - `base_instruction`
  - `allowed_actions`
  - `verification_required`

#### Scenario: No Context for Port

- **GIVEN** a server is running on port 3001
- **AND** NO context exists for port 3001
- **WHEN** a client sends `GET /context` to the server
- **THEN** the server MUST respond with HTTP 404
- **AND** include an error message indicating no context is configured

### Requirement: Health Check Endpoint

The system MUST provide a health check endpoint for monitoring.

#### Scenario: Health Check Response

- **GIVEN** a server is running
- **WHEN** a client sends `GET /health` to the server
- **THEN** the server MUST respond with HTTP 200
- **AND** indicate the server is healthy

### Requirement: Error Handling

The system MUST handle errors gracefully and return appropriate HTTP status codes.

#### Scenario: Invalid Endpoint

- **GIVEN** a server is running
- **WHEN** a client requests an undefined endpoint (e.g., `GET /invalid`)
- **THEN** the server MUST respond with HTTP 404

#### Scenario: Server Internal Error

- **GIVEN** a server encounters an internal error
- **WHEN** processing a request
- **THEN** the server MUST respond with HTTP 500
- **AND** NOT leak internal implementation details

#### Scenario: Method Not Allowed

- **GIVEN** a server is running
- **WHEN** a client sends a POST request to `GET`-only endpoint
- **THEN** the server MUST respond with HTTP 405 Method Not Allowed

### Requirement: Content Type Headers

The system MUST set appropriate content type headers for responses.

#### Scenario: JSON Response Content Type

- **GIVEN** the server is returning a JSON response
- **WHEN** sending the response
- **THEN** the `Content-Type` header MUST be set to `application/json`

### Requirement: CORS Headers Support

The system MUST support CORS headers for browser-based clients when enabled.

#### Scenario: CORS Preflight

- **GIVEN** a browser client sends an OPTIONS request
- **WHEN** the server receives the preflight request
- **THEN** the server SHOULD respond with appropriate CORS headers
- **NOTE**: This is optional for MVP if AI agents don't require CORS

### Requirement: Concurrent Request Handling

The system MUST handle concurrent requests correctly.

#### Scenario: Multiple Simultaneous Requests

- **GIVEN** a server is running
- **WHEN** multiple clients send requests simultaneously
- **THEN** the server MUST handle all requests
- **AND** each request MUST receive the correct response
- **AND** responses MUST NOT be mixed between clients

### Requirement: Request Logging

The system MUST log incoming requests for debugging and monitoring.

#### Scenario: Log Request Details

- **GIVEN** a request is received
- **WHEN** the server processes the request
- **THEN** the system SHOULD log:
  - Timestamp
  - Client IP (if available)
  - HTTP method and path
  - Response status code
