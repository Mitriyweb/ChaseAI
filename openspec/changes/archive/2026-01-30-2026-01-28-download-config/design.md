# Design: Download Config Button and AI Integration

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                    ChaseAI Application                       │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  ┌──────────────────────────────────────────────────────┐   │
│  │              Tray Menu                               │   │
│  │  ┌─────────────────────────────────────────────────┐ │   │
│  │  │ • Show Window                                   │ │   │
│  │  │ • Settings                                      │ │   │
│  │  │ • Download Config  ← NEW                        │ │   │
│  │  │ • Quit                                          │ │   │
│  │  └─────────────────────────────────────────────────┘ │   │
│  └──────────────────────────────────────────────────────┘   │
│                           │                                   │
│                           ▼                                   │
│  ┌──────────────────────────────────────────────────────┐   │
│  │         Configuration Generator                      │   │
│  │  - Collect port mappings                            │   │
│  │  - Gather endpoint information                      │   │
│  │  - Generate documentation                          │   │
│  │  - Create JSON/YAML/Markdown files                 │   │
│  └──────────────────────────────────────────────────────┘   │
│                           │                                   │
│                           ▼                                   │
│  ┌──────────────────────────────────────────────────────┐   │
│  │         HTTP Server (Port 8090+)                     │   │
│  │  GET /config          - Full configuration          │   │
│  │  GET /config/ports    - Port mappings only          │   │
│  │  GET /config/docs     - Markdown documentation      │   │
│  └──────────────────────────────────────────────────────┘   │
│                           │                                   │
│                           ▼                                   │
│  ┌──────────────────────────────────────────────────────┐   │
│  │         File Download                               │   │
│  │  - Browser download triggered                       │   │
│  │  - File: chaseai-config-TIMESTAMP.json              │   │
│  │  - Includes all necessary information               │   │
│  └──────────────────────────────────────────────────────┘   │
│                                                               │
└─────────────────────────────────────────────────────────────┘
                              │
                ┌─────────────┴─────────────┐
                ▼                           ▼
        ┌──────────────────┐        ┌──────────────────┐
        │  User Downloads  │        │  AI Agent        │
        │  Configuration   │        │  Reads Config    │
        │  File            │        │  Integrates      │
        └──────────────────┘        └──────────────────┘
```

## Component Design

### 1. Tray Menu Integration

**Location**: `src/rs/ui/tray_menu.rs`

**Responsibilities**:
- Add "Download Config" menu item
- Handle menu click event
- Trigger configuration download
- Show success/error notification

**Key Decisions**:
- Place button after "Settings" and before "Quit"
- Use system file download dialog
- Show notification when download completes
- Handle errors gracefully

### 2. Configuration Generator

**Location**: New module `src/rs/config/generator.rs`

**Responsibilities**:
- Collect current port mappings
- Gather endpoint information
- Generate JSON configuration
- Generate YAML configuration
- Generate Markdown documentation
- Create timestamped filename

**Key Decisions**:
- Generate on-demand (not cached)
- Include all active ports
- Document all endpoints
- Provide example requests
- Include verification workflow

### 3. Configuration Endpoint

**Location**: `src/rs/server/instruction_server.rs` (extend)

**Responsibilities**:
- Expose `GET /config` endpoint
- Return JSON configuration
- Support format parameter (json, yaml, markdown)
- Set appropriate content-type headers
- Include version information

**Key Decisions**:
- Endpoint available on all instruction servers
- Same configuration on all ports
- No authentication required (configuration is not sensitive)
- Version field for future compatibility

### 4. Configuration File Format

**JSON Structure**:
```json
{
  "version": "1.0.0",
  "timestamp": "2026-01-28T10:30:00Z",
  "application": {
    "name": "ChaseAI",
    "version": "0.1.0",
    "description": "Local control and orchestration system for AI agents"
  },
  "ports": [
    {
      "port": 8090,
      "interface": "127.0.0.1",
      "role": "instruction",
      "enabled": true,
      "endpoints": [
        {
          "path": "/context",
          "method": "GET",
          "description": "Retrieve instruction context"
        },
        {
          "path": "/verify",
          "method": "POST",
          "description": "Request verification for action"
        }
      ]
    }
  ],
  "endpoints": {
    "/context": {
      "method": "GET",
      "description": "Retrieve instruction context for this port",
      "response": {
        "system": "string",
        "role": "string",
        "base_instruction": "string",
        "allowed_actions": ["string"],
        "verification_required": "boolean"
      }
    },
    "/verify": {
      "method": "POST",
      "description": "Request verification for an action",
      "request": {
        "action": "string",
        "reason": "string",
        "context": "object"
      },
      "response": {
        "status": "pending|approved|rejected",
        "verification_id": "string"
      }
    }
  },
  "documentation": {
    "getting_started": "URL to markdown docs",
    "api_reference": "URL to API docs",
    "verification_workflow": "URL to verification docs"
  }
}
```

### 5. AI Integration Documentation

**Location**: New file `docs/ai-integration.md`

**Contents**:
- How to discover ChaseAI
- How to download configuration
- How to parse configuration file
- How to retrieve instruction context
- How to request verification
- Example code for different AI platforms
- Error handling and retry logic
- Security best practices

### 6. Verification Workflow Documentation

**Location**: New file `docs/verification-workflow.md`

**Contents**:
- Overview of verification process
- Request format and fields
- Response format and status codes
- Human approval flow
- Timeout and retry behavior
- Example verification requests
- Error scenarios and handling

## Implementation Phases

### Phase 1: Foundation (Week 1)
- Add "Download Config" button to tray menu
- Create configuration generator module
- Generate JSON configuration file
- Implement file download mechanism

### Phase 2: Endpoints (Week 2)
- Add `/config` endpoint to HTTP server
- Support multiple formats (JSON, YAML)
- Add format parameter support
- Test endpoint accessibility

### Phase 3: Documentation (Week 2-3)
- Write AI integration guide
- Document verification workflow
- Create example code snippets
- Document error handling

### Phase 4: Testing & Polish (Week 3)
- Integration tests for configuration generation
- End-to-end tests for download flow
- Documentation review
- User testing with AI agents

## Technical Decisions

### Configuration Generation
**Decision**: Generate on-demand, not cached
**Rationale**:
- Always reflects current state
- No stale configuration issues
- Minimal performance impact
- Simpler implementation

### File Format
**Decision**: JSON as primary, YAML and Markdown as alternatives
**Rationale**:
- JSON is universally supported
- YAML is human-readable
- Markdown for documentation
- Easy to parse in any language

### Endpoint Location
**Decision**: Available on all instruction servers
**Rationale**:
- Consistent across all ports
- No special configuration needed
- Easy for AI agents to discover
- Follows REST conventions

### Security
**Decision**: No authentication required for configuration endpoint
**Rationale**:
- Configuration contains no sensitive data
- Port information is already discoverable
- Simplifies integration for AI agents
- Can add authentication in future if needed

## Dependencies & Requirements

### External Tools
- None (uses existing HTTP server)

### New Modules
- `src/rs/config/generator.rs` - Configuration generation
- `docs/ai-integration.md` - AI integration guide
- `docs/verification-workflow.md` - Verification documentation

### Existing Dependencies
- `serde_json` - JSON serialization
- `serde_yaml` - YAML serialization (may need to add)
- `axum` - HTTP server (already used)

## Testing Strategy

### Unit Testing
- Configuration generator produces valid JSON
- All required fields present in configuration
- Port mappings correctly serialized
- Endpoint information accurate

### Integration Testing
- `/config` endpoint returns valid JSON
- Format parameter works correctly
- File download completes successfully
- Configuration file can be parsed by AI agents

### End-to-End Testing
- User clicks "Download Config" button
- File downloads to default location
- File contains all necessary information
- AI agent can parse and use configuration

## Rollback & Maintenance

### Rollback Procedure
- Remove "Download Config" button from menu
- Disable `/config` endpoint
- Remove configuration generator module

### Maintenance Tasks
- Update configuration format when API changes
- Keep documentation in sync with implementation
- Monitor for AI agent integration issues
- Gather feedback for improvements

## Future Enhancements

1. **Automatic Discovery**: Add mDNS/Bonjour support for network discovery
2. **Authentication**: Add API key or OAuth support
3. **Rate Limiting**: Implement rate limiting for endpoints
4. **Webhooks**: Add webhook support for verification results
5. **Configuration UI**: Add GUI for editing configuration
6. **Agent Registry**: Maintain registry of integrated AI agents
7. **Analytics**: Track configuration downloads and usage
8. **Versioning**: Support multiple API versions
