# selective-config-generation Specification

## Purpose

TBD - created by archiving change config-download-preview. Update Purpose after archive.

### Unit Tests

```bash
cargo test
```

## Requirements

### Requirement: Filtered Configuration Generation

The configuration generator MUST support generating configurations with a subset of ports.

#### Scenario: Generate with All Ports

- **GIVEN** a network configuration with multiple ports
- **WHEN** configuration is generated without a filter
- **THEN** all enabled ports MUST be included
- **AND** disabled ports MUST be excluded
- **AND** the output MUST match existing behavior

#### Scenario: Generate with Single Port

- **GIVEN** a network configuration with multiple ports
- **WHEN** configuration is generated with a filter for one port
- **THEN** only the specified port MUST be included
- **AND** the port MUST have all its endpoint information
- **AND** other ports MUST be excluded

#### Scenario: Generate with Port Subset

- **GIVEN** a network configuration with multiple ports
- **WHEN** configuration is generated with a filter for some ports
- **THEN** only the specified ports MUST be included
- **AND** each port MUST have complete information
- **AND** unspecified ports MUST be excluded

#### Scenario: Generate with Empty Filter

- **GIVEN** a network configuration with ports
- **WHEN** configuration is generated with an empty port filter
- **THEN** the system MUST return an error
- **AND** the error MUST indicate at least one port is required

### Requirement: JSON Format with Filtering

The generator MUST support filtered JSON configuration generation.

#### Scenario: Filtered JSON Generation

- **GIVEN** a port filter is provided
- **WHEN** JSON configuration is generated
- **THEN** the output MUST be valid JSON
- **AND** the `ports` array MUST contain only filtered ports
- **AND** the `endpoints` object MUST include all available endpoints
- **AND** the `version`, `timestamp`, and `application` fields MUST be present

#### Scenario: JSON Structure Consistency

- **GIVEN** a filtered JSON configuration is generated
- **WHEN** compared to unfiltered configuration
- **THEN** the structure MUST be identical
- **AND** only the `ports` array content MUST differ
- **AND** the `timestamp` MAY differ

### Requirement: YAML Format with Filtering

The generator MUST support filtered YAML configuration generation.

#### Scenario: Filtered YAML Generation

- **GIVEN** a port filter is provided
- **WHEN** YAML configuration is generated
- **THEN** the output MUST be valid YAML
- **AND** the `ports` list MUST contain only filtered ports
- **AND** the structure MUST match JSON equivalent

#### Scenario: YAML Readability

- **GIVEN** a filtered YAML configuration is generated
- **WHEN** the output is examined
- **THEN** the YAML MUST be properly indented
- **AND** the YAML MUST be human-readable
- **AND** the YAML MUST be parseable by standard YAML parsers

### Requirement: Markdown Format with Filtering

The generator MUST support filtered Markdown configuration generation.

#### Scenario: Filtered Markdown Generation

- **GIVEN** a port filter is provided
- **WHEN** Markdown configuration is generated
- **THEN** the output MUST be valid Markdown
- **AND** only filtered ports MUST have sections
- **AND** each port section MUST include all endpoint documentation

#### Scenario: Markdown Documentation Quality

- **GIVEN** a filtered Markdown configuration is generated
- **WHEN** the output is examined
- **THEN** the Markdown MUST have proper heading hierarchy
- **AND** the Markdown MUST include code blocks for examples
- **AND** the Markdown MUST be readable as documentation

### Requirement: Port Filter Validation

The system MUST validate port filters before generating configuration.

#### Scenario: Valid Port Numbers

- **GIVEN** a port filter is provided
- **WHEN** the filter contains valid port numbers
- **THEN** the generation MUST succeed
- **AND** only valid ports MUST be included

#### Scenario: Invalid Port Numbers

- **GIVEN** a port filter contains a non-existent port number
- **WHEN** configuration is generated
- **THEN** the invalid port MUST be silently ignored
- **AND** only existing ports from the filter MUST be included
- **AND** if no valid ports remain, an error MUST be returned

#### Scenario: Disabled Port in Filter

- **GIVEN** a port filter includes a disabled port
- **WHEN** configuration is generated
- **THEN** the disabled port MUST be excluded
- **AND** only enabled ports from the filter MUST be included

### Requirement: Endpoint Information Completeness

Filtered configurations MUST include complete endpoint information for included ports.

#### Scenario: Instruction Port Endpoints

- **GIVEN** an Instruction port is in the filter
- **WHEN** configuration is generated
- **THEN** the port MUST include `/context` endpoint
- **AND** the port MUST include `/config` endpoint
- **AND** the port MUST include `/health` endpoint
- **AND** each endpoint MUST have complete documentation

#### Scenario: Verification Port Endpoints

- **GIVEN** a Verification port is in the filter
- **WHEN** configuration is generated
- **THEN** the port MUST include `/verify` endpoint
- **AND** the port MUST include `/health` endpoint
- **AND** each endpoint MUST have complete documentation

#### Scenario: Mixed Port Roles

- **GIVEN** the filter includes both Instruction and Verification ports
- **WHEN** configuration is generated
- **THEN** each port MUST have role-appropriate endpoints
- **AND** endpoint documentation MUST be accurate for each role

### Requirement: API Consistency

Filtered configuration generation MUST maintain API consistency.

#### Scenario: Method Signature Compatibility

- **GIVEN** existing configuration generation methods
- **WHEN** filtered methods are added
- **THEN** the new methods MUST accept an optional filter parameter
- **AND** passing `None` MUST behave identically to existing methods
- **AND** existing method signatures MUST remain unchanged

#### Scenario: Return Type Consistency

- **GIVEN** filtered configuration generation methods
- **WHEN** called with various filters
- **THEN** the return types MUST match existing methods
- **AND** JSON methods MUST return `Result<Value>`
- **AND** YAML methods MUST return `Result<String>`
- **AND** Markdown methods MUST return `Result<String>`

### Requirement: Performance

Filtered configuration generation MUST perform efficiently.

#### Scenario: Generation Speed

- **GIVEN** a configuration with many ports
- **WHEN** a filtered configuration is generated
- **THEN** the generation MUST complete in under 100ms
- **AND** the performance MUST be comparable to unfiltered generation

#### Scenario: Memory Usage

- **GIVEN** a large configuration is generated
- **WHEN** filtering is applied
- **THEN** memory usage MUST be proportional to filtered port count
- **AND** memory MUST be released after generation
