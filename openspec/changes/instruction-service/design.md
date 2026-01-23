# Design: Instruction Service

## Architecture Overview

```text
┌───────────────────────────────────────────────┐
│              AI Agent (External)              │
│         HTTP Request: GET /context            │
└──────────────────┬────────────────────────────┘
                   │
┌──────────────────▼────────────────────────────┐
│         Instruction Server (per port)         │
│  - Bound to specific interface:port          │
│  - Serves InstructionContext as JSON          │
└──────────────────┬────────────────────────────┘
                   │
┌──────────────────▼────────────────────────────┐
│          Context Manager Layer                │
│  - CRUD operations for contexts               │
│  - Validation against network config          │
│  - In-memory cache + persistence              │
└──────────────────┬────────────────────────────┘
                   │
         ┌─────────┴─────────┐
         │                   │
┌────────▼─────┐  ┌─────────▼────────┐
│   Storage    │  │  Network Config  │
│  (contexts)  │  │  (port bindings) │
└──────────────┘  └──────────────────┘
```

## Key Components

### 1. InstructionContext

**Responsibility**: Data structure representing an instruction context for an AI agent

**Definition**:

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InstructionContext {
    /// System identifier (e.g., "WinSF")
    pub system: String,

    /// Agent role (e.g., "execution-agent")
    pub role: String,

    /// Base instruction text defining agent behavior
    pub base_instruction: String,

    /// List of allowed action types
    pub allowed_actions: Vec<String>,

    /// Whether verification is required for actions
    pub verification_required: bool,
}

impl InstructionContext {
    pub fn new(
        system: String,
        role: String,
        base_instruction: String,
        allowed_actions: Vec<String>,
        verification_required: bool,
    ) -> Result<Self>;

    pub fn validate(&self) -> Result<()>;
}
```

**Validation Rules**:

- `system` must not be empty
- `role` must not be empty
- `base_instruction` must not be empty
- `allowed_actions` must contain at least one action
- Action names must match pattern `^[a-z][a-z0-9-]*$`

### 2. ContextManager

**Responsibility**: Manage CRUD operations for instruction contexts

**API**:

```rust
use std::collections::HashMap;

pub struct ContextManager {
    contexts: HashMap<u16, InstructionContext>,
    network_config: Arc<NetworkConfig>, // from network-interface-management
    storage: ContextStorage,
}

impl ContextManager {
    pub fn new(network_config: Arc<NetworkConfig>) -> Result<Self>;

    /// Create or update context for a port
    pub fn set_context(&mut self, port: u16, context: InstructionContext) -> Result<()>;

    /// Get context for a port
    pub fn get_context(&self, port: u16) -> Option<&InstructionContext>;

    /// Delete context for a port
    pub fn delete_context(&mut self, port: u16) -> Result<()>;

    /// List all contexts
    pub fn list_contexts(&self) -> Vec<(u16, &InstructionContext)>;

    /// Validate port exists in network config
    fn validate_port(&self, port: u16) -> Result<()>;
}
```

**Behavior**:

- Before creating context, verify port exists in network config
- Automatically persist changes via `ContextStorage`
- Cache contexts in memory for fast access
- Thread-safe for concurrent access

### 3. ContextStorage

**Responsibility**: Persist and load instruction contexts

**API**:

```rust
pub struct ContextStorage {
    config_dir: PathBuf,
}

impl ContextStorage {
    pub fn new() -> Result<Self>;

    /// Load all contexts from disk
    pub fn load_all(&self) -> Result<HashMap<u16, InstructionContext>>;

    /// Save all contexts to disk
    pub fn save_all(&self, contexts: &HashMap<u16, InstructionContext>) -> Result<()>;

    /// Get config file path
    pub fn config_path(&self) -> PathBuf;
}
```

**Storage Format** (JSON):

```json
{
  "contexts": [
    {
      "port": 3000,
      "context": {
        "system": "WinSF",
        "role": "execution-agent",
        "base_instruction": "You are an execution agent for WinSF workflows...",
        "allowed_actions": ["analyze", "propose", "execute-safe"],
        "verification_required": true
      }
    },
    {
      "port": 3001,
      "context": {
        "system": "ChaseAI",
        "role": "verification-agent",
        "base_instruction": "You verify execution results...",
        "allowed_actions": ["verify", "approve", "reject"],
        "verification_required": false
      }
    }
  ]
}
```

**File Location**:

- macOS/Linux: `~/.config/chaseai/contexts.json`
- Windows: `%APPDATA%\ChaseAI\contexts.json`

### 4. InstructionServer

**Responsibility**: HTTP server that serves instruction contexts to AI agents

**API**:

```rust
use axum::{Router, Json};

pub struct InstructionServer {
    port: u16,
    interface: NetworkInterface,
    context_manager: Arc<Mutex<ContextManager>>,
}

impl InstructionServer {
    pub fn new(
        port: u16,
        interface: NetworkInterface,
        context_manager: Arc<Mutex<ContextManager>>,
    ) -> Self;

    pub async fn start(&self) -> Result<()>;
    pub async fn stop(&self) -> Result<()>;

    /// Build axum router with endpoints
    fn router(&self) -> Router;
}

// Endpoint handlers
async fn get_context(
    State(manager): State<Arc<Mutex<ContextManager>>>,
    Extension(port): Extension<u16>,
) -> Result<Json<InstructionContext>, StatusCode>;
```

**Endpoints**:

- `GET /context` - Returns the instruction context for this port as JSON
- `GET /health` - Health check endpoint (returns 200 OK)

**Error Responses**:

- 404 - No context configured for this port
- 500 - Internal server error

### 5. ServerPool

**Responsibility**: Manage multiple instruction servers (one per enabled port)

**API**:

```rust
pub struct ServerPool {
    servers: HashMap<u16, InstructionServer>,
    context_manager: Arc<Mutex<ContextManager>>,
    network_config: Arc<NetworkConfig>,
}

impl ServerPool {
    pub fn new(
        context_manager: Arc<Mutex<ContextManager>>,
        network_config: Arc<NetworkConfig>,
    ) -> Self;

    /// Start servers for all enabled port bindings
    pub async fn start_all(&mut self) -> Result<()>;

    /// Stop all servers
    pub async fn stop_all(&mut self) -> Result<()>;

    /// Start server for specific port
    pub async fn start_server(&mut self, port: u16) -> Result<()>;

    /// Stop server for specific port
    pub async fn stop_server(&mut self, port: u16) -> Result<()>;
}
```

**Behavior**:

- On startup, read network config and start server for each enabled port binding
- When port is enabled in network config, start corresponding server
- When port is disabled, stop corresponding server
- Handle server failures gracefully (log error, continue with other servers)

## Data Flow

### Startup Flow

```text
1. Load NetworkConfig (from network-interface-management)
2. Load ContextStorage → get all saved contexts
3. Initialize ContextManager with loaded contexts
4. Create ServerPool with ContextManager and NetworkConfig
5. ServerPool.start_all() → create InstructionServer for each enabled port
6. Each InstructionServer binds to its interface:port and starts listening
```

### Agent Request Flow

```text
1. AI Agent sends: GET http://127.0.0.1:3000/context
2. InstructionServer receives request
3. Server extracts port number (3000)
4. Server calls ContextManager.get_context(3000)
5. ContextManager returns InstructionContext from cache
6. Server serializes context to JSON
7. Server responds with 200 OK + JSON body
```

### Context Update Flow

```text
1. User modifies context via UI (future) or config file
2. ContextManager.set_context(port, new_context)
3. Manager validates port exists in network config
4. Manager validates context fields
5. Manager updates in-memory cache
6. Manager calls storage.save_all() to persist
7. Changes take effect immediately for new requests
```

## Security Considerations

1. **Network Binding**: Servers only bind to interfaces configured in network-interface-management
2. **Default Loopback**: Default configuration serves contexts only on loopback
3. **No Authentication (MVP)**: Authentication deferred to future phase
4. **Context Validation**: All contexts validated before serving
5. **Error Messages**: Don't leak internal details in error responses

## Error Handling

- **Port not in network config**: Return error when creating context
- **No context for port**: HTTP 404 with clear message
- **Server bind failure**: Log error, mark port as failed, continue with other ports
- **Invalid context**: Reject with validation error
- **Storage failures**: Log warning, operate with in-memory state

## Testing Strategy

### Unit Tests

- InstructionContext: Validation rules, serialization
- ContextManager: CRUD operations, validation
- ContextStorage: Save/load, file handling
- InstructionServer: Endpoint handlers (with mock state)

### Integration Tests

- Full flow: Create context → Start server → HTTP request → Verify response
- Test with different interfaces (loopback, LAN)
- Test error cases (missing context, invalid port)
- Test persistence (restart simulation)

### Manual Testing

- Use `curl` to test endpoints: `curl http://127.0.0.1:3000/context`
- Verify JSON response format
- Test with real AI agent client (if available)

## Dependencies

- **Axum** or **Actix-web**: HTTP server framework
- **Serde**: JSON serialization
- **Tokio**: Async runtime
- **network-interface-management**: Port bindings and network config

## Trade-offs

### Decision: HTTP vs TCP Protocol

**Chosen**: HTTP (REST)
**Rationale**:

- Simpler for AI agents to integrate (standard HTTP clients)
- Easier debugging (curl, browser)
- JSON is human-readable
**Trade-off**: Binary TCP protocol would be more efficient, but adds complexity

### Decision: Pull vs Push Model

**Chosen**: Pull (agent requests context)
**Rationale**:

- Simpler implementation for MVP
- Agent controls when it needs context
- Stateless server design
**Trade-off**: Push model would allow real-time context updates, but requires persistent connections

### Decision: In-Memory Cache

**Chosen**: Cache all contexts in memory
**Rationale**:

- Fast access for every request
- Context data is small
- Simplifies implementation
**Trade-off**: Not suitable for thousands of contexts, but MVP has <10 ports

### Decision: One Server Per Port

**Chosen**: Separate server instance for each port
**Rationale**:

- Each port can bind to different interface
- Independent lifecycle (enable/disable per port)
- Isolation for future multi-tenancy
**Trade-off**: More resource usage, but necessary for MVP requirements

## Future Extensibility

- **Instruction versioning**: Add version field to context, track history
- **Dynamic context updates**: WebSocket endpoint for real-time updates
- **Context templates**: Pre-defined templates for common agent roles
- **Analytics**: Track which contexts are accessed, by whom
- **Authentication**: Add API key or JWT validation
