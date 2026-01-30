# ai-integration Specification

## Purpose

Define the contract and workflow for AI agents to discover, configure, and interact with ChaseAI services.

## Requirements

### Requirement: Configuration Discovery

AI agents MUST be able to discover and retrieve ChaseAI's configuration.

#### Scenario: Configuration File Download

- **GIVEN** an AI agent needs to integrate with ChaseAI
- **WHEN** the user clicks "Download Config" in the tray menu
- **THEN** a configuration file is generated
- **AND** the file contains all necessary information for integration
- **AND** the file is in a machine-readable format (JSON)

#### Scenario: Configuration Endpoint Discovery

- **GIVEN** an AI agent knows the ChaseAI port (e.g., 8090)
- **WHEN** the agent sends `GET http://localhost:8090/config`
- **THEN** the server responds with the configuration
- **AND** the response includes all available endpoints
- **AND** the response includes port mappings

#### Scenario: Configuration Parsing

- **GIVEN** an AI agent receives a configuration file
- **WHEN** the agent parses the JSON
- **THEN** the agent MUST be able to extract:
  - Available ports and their roles
  - Endpoint paths and methods
  - Request/response formats
  - Verification workflow information

### Requirement: Context Retrieval

AI agents MUST be able to retrieve instruction contexts from ChaseAI.

#### Scenario: Context Endpoint Access

- **GIVEN** an AI agent has the configuration
- **WHEN** the agent sends `GET /context` to the instruction server
- **THEN** the server responds with HTTP 200
- **AND** the response contains the instruction context
- **AND** the context includes system, role, and allowed actions

#### Scenario: Context Format

- **GIVEN** a context is retrieved
- **WHEN** the agent receives the response
- **THEN** the context MUST include:
  - `system` - System description
  - `role` - Agent role
  - `base_instruction` - Base instruction text
  - `allowed_actions` - List of allowed actions
  - `verification_required` - Whether verification is needed

#### Scenario: Multiple Contexts

- **GIVEN** multiple ports are configured
- **WHEN** an AI agent queries different ports
- **THEN** each port MAY have a different context
- **AND** the agent MUST be able to handle multiple contexts
- **AND** the agent MUST use the correct context for each port

### Requirement: Verification Request

AI agents MUST be able to request verification for actions that require human approval.

#### Scenario: Verification Request Format

- **GIVEN** an AI agent needs to request verification
- **WHEN** the agent sends a POST request to `/verify`
- **THEN** the request MUST include:
  - `action` - Description of the action
  - `reason` - Reason for the action
  - `context` - Relevant context (optional)

#### Scenario: Verification Response

- **GIVEN** a verification request is submitted
- **WHEN** the server processes the request
- **THEN** the response MUST include:
  - `status` - Status (pending, approved, rejected)
  - `verification_id` - Unique identifier for tracking
  - `message` - Human-readable message (optional)

#### Scenario: Verification Status Polling

- **GIVEN** a verification request is pending
- **WHEN** the AI agent polls for status
- **THEN** the agent MUST be able to query the status
- **AND** the response MUST indicate current status
- **AND** the agent MUST handle timeouts gracefully

#### Scenario: Verification Timeout

- **GIVEN** a verification request is pending
- **WHEN** the timeout period expires (e.g., 5 minutes)
- **THEN** the verification MUST be automatically rejected
- **AND** the agent MUST be notified of the timeout
- **AND** the agent MUST handle the rejection appropriately

### Requirement: Error Handling

AI agents MUST be able to handle errors and edge cases gracefully.

#### Scenario: Connection Error

- **GIVEN** an AI agent attempts to connect to ChaseAI
- **WHEN** the connection fails
- **THEN** the agent MUST receive an appropriate error
- **AND** the agent MUST implement retry logic
- **AND** the agent MUST not crash

#### Scenario: Invalid Configuration

- **GIVEN** an AI agent receives a configuration file
- **WHEN** the configuration is malformed or incomplete
- **THEN** the agent MUST detect the error
- **AND** the agent MUST report the error clearly
- **AND** the agent MUST not attempt to use invalid configuration

#### Scenario: Endpoint Not Found

- **GIVEN** an AI agent requests an endpoint
- **WHEN** the endpoint does not exist
- **THEN** the server responds with HTTP 404
- **AND** the agent MUST handle the 404 gracefully
- **AND** the agent MUST not retry indefinitely

#### Scenario: Server Error

- **GIVEN** an AI agent makes a request
- **WHEN** the server encounters an error (HTTP 500)
- **THEN** the agent MUST receive the error response
- **AND** the agent MUST implement exponential backoff retry
- **AND** the agent MUST eventually give up and report the error

### Requirement: Security Considerations

AI agents MUST follow security best practices when integrating with ChaseAI.

#### Scenario: Configuration Confidentiality

- **GIVEN** a configuration file is downloaded
- **WHEN** the file is stored
- **THEN** the agent MUST store it securely
- **AND** the agent MUST NOT log sensitive information
- **AND** the agent MUST NOT share the configuration with untrusted parties

#### Scenario: Verification Request Validation

- **GIVEN** a verification request is submitted
- **WHEN** the request is processed
- **THEN** the agent MUST validate the request format
- **AND** the agent MUST NOT submit malformed requests
- **AND** the agent MUST NOT attempt to bypass verification

#### Scenario: Port Binding Verification

- **GIVEN** an AI agent connects to a port
- **WHEN** the connection is established
- **THEN** the agent MUST verify it's connecting to the correct interface
- **AND** the agent MUST NOT connect to unexpected ports
- **AND** the agent MUST validate the server response

### Requirement: Integration Documentation

AI agents MUST have clear documentation on how to integrate with ChaseAI.

#### Scenario: Getting Started Guide

- **GIVEN** a developer wants to integrate an AI agent
- **WHEN** they read the documentation
- **THEN** they MUST find:
  - Step-by-step integration instructions
  - Example code for their platform
  - Common pitfalls and solutions
  - Troubleshooting guide

#### Scenario: API Reference

- **GIVEN** a developer needs API details
- **WHEN** they consult the documentation
- **THEN** they MUST find:
  - Complete endpoint documentation
  - Request/response examples
  - Error codes and meanings
  - Rate limiting information (if applicable)

#### Scenario: Example Code

- **GIVEN** a developer wants to implement integration
- **WHEN** they look for examples
- **THEN** they MUST find:
  - Python example (for Claude integration)
  - JavaScript example (for Node.js)
  - Rust example (for local integration)
  - Error handling examples

#### Scenario: Verification Workflow Guide

- **GIVEN** a developer needs to implement verification
- **WHEN** they read the documentation
- **THEN** they MUST understand:
  - How to request verification
  - How to handle pending status
  - How to handle approval/rejection
  - How to implement timeout logic
