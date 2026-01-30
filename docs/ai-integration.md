# AI Integration Guide

## Overview

This guide explains how to integrate AI agents with ChaseAI to enable controlled execution with human-in-the-loop verification.

## Getting Started

### 1. Discover ChaseAI Configuration

First, obtain the ChaseAI configuration file. You can do this in two ways:

### Option A: Download via UI

- Click "Download Config" in the ChaseAI tray menu
- Save the configuration file locally

### Option B: Retrieve via API

```bash
curl http://localhost:8090/config
```

### 2. Parse Configuration

The configuration file contains all information needed to integrate with ChaseAI:

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
      "interface": {
        "name": "lo0",
        "ip_address": "127.0.0.1",
        "type": "Loopback"
      },
      "role": "Instruction",
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
    "/context": { ... },
    "/verify": { ... },
    "/health": { ... },
    "/config": { ... }
  },
  "documentation": {
    "getting_started": "...",
    "api_reference": "...",
    "verification_workflow": "..."
  }
}
```

### 3. Retrieve Instruction Context

Get the instruction context for a specific port:

```bash
curl http://localhost:8090/context
```

Response:

```json
{
  "system": "System description",
  "role": "Agent role",
  "base_instruction": "Base instruction text",
  "allowed_actions": ["action1", "action2"],
  "verification_required": true
}
```

The context tells you:

- **system**: What system you're controlling
- **role**: Your role in the system
- **base_instruction**: Core instructions to follow
- **allowed_actions**: Actions you're permitted to perform
- **verification_required**: Whether human approval is needed

### 4. Request Verification

When you need to perform an action that requires verification:

```bash
curl -X POST http://localhost:8090/verify \
  -H "Content-Type: application/json" \
  -d '{
    "action": "Deploy to production",
    "reason": "User requested deployment",
    "context": {
      "environment": "production",
      "version": "1.2.3"
    }
  }'
```

Response:

```json
{
  "status": "pending",
  "verification_id": "verify-abc123",
  "message": "Verification request submitted"
}
```

### 5. Poll for Verification Status

Check the status of your verification request:

```bash
curl http://localhost:8090/verify/verify-abc123
```

Response:

```json
{
  "status": "approved",
  "verification_id": "verify-abc123",
  "message": "Approved by user"
}
```

Possible statuses:

- **pending**: Waiting for human approval
- **approved**: Action approved, proceed
- **rejected**: Action rejected, do not proceed

## Implementation Examples

### Python (Claude Integration)

```python
import requests
import json
import time

class ChaseAIClient:
    def __init__(self, host="localhost", port=8090):
        self.base_url = f"http://{host}:{port}"
        self.session = requests.Session()

    def get_config(self, format="json"):
        """Retrieve configuration"""
        response = self.session.get(
            f"{self.base_url}/config",
            params={"format": format}
        )
        response.raise_for_status()
        return response.json() if format == "json" else response.text

    def get_context(self):
        """Retrieve instruction context"""
        response = self.session.get(f"{self.base_url}/context")
        response.raise_for_status()
        return response.json()

    def request_verification(self, action, reason, context=None):
        """Request verification for an action"""
        payload = {
            "action": action,
            "reason": reason,
            "context": context or {}
        }
        response = self.session.post(
            f"{self.base_url}/verify",
            json=payload
        )
        response.raise_for_status()
        return response.json()

    def check_verification_status(self, verification_id, timeout=300):
        """Poll for verification status with timeout"""
        start_time = time.time()
        while time.time() - start_time < timeout:
            response = self.session.get(
                f"{self.base_url}/verify/{verification_id}"
            )
            response.raise_for_status()
            result = response.json()

            if result["status"] != "pending":
                return result

            time.sleep(1)  # Poll every second

        raise TimeoutError(f"Verification {verification_id} timed out")

# Usage
client = ChaseAIClient()

# Get context
context = client.get_context()
print(f"Role: {context['role']}")
print(f"Allowed actions: {context['allowed_actions']}")

# Request verification
result = client.request_verification(
    action="Deploy to production",
    reason="User requested deployment",
    context={"version": "1.2.3"}
)
verification_id = result["verification_id"]

# Wait for approval
status = client.check_verification_status(verification_id)
if status["status"] == "approved":
    print("Action approved, proceeding...")
else:
    print("Action rejected")
```

### JavaScript (Node.js)

```javascript
const axios = require('axios');

class ChaseAIClient {
  constructor(host = 'localhost', port = 8090) {
    this.baseUrl = `http://${host}:${port}`;
    this.client = axios.create({
      baseURL: this.baseUrl,
      timeout: 5000
    });
  }

  async getConfig(format = 'json') {
    const response = await this.client.get('/config', {
      params: { format }
    });
    return response.data;
  }

  async getContext() {
    const response = await this.client.get('/context');
    return response.data;
  }

  async requestVerification(action, reason, context = {}) {
    const response = await this.client.post('/verify', {
      action,
      reason,
      context
    });
    return response.data;
  }

  async checkVerificationStatus(verificationId, timeout = 300000) {
    const startTime = Date.now();
    while (Date.now() - startTime < timeout) {
      const response = await this.client.get(`/verify/${verificationId}`);
      const result = response.data;

      if (result.status !== 'pending') {
        return result;
      }

      await new Promise(resolve => setTimeout(resolve, 1000));
    }

    throw new Error(`Verification ${verificationId} timed out`);
  }
}

// Usage
(async () => {
  const client = new ChaseAIClient();

  // Get context
  const context = await client.getContext();
  console.log(`Role: ${context.role}`);
  console.log(`Allowed actions: ${context.allowed_actions}`);

  // Request verification
  const result = await client.requestVerification(
    'Deploy to production',
    'User requested deployment',
    { version: '1.2.3' }
  );
  const verificationId = result.verification_id;

  // Wait for approval
  const status = await client.checkVerificationStatus(verificationId);
  if (status.status === 'approved') {
    console.log('Action approved, proceeding...');
  } else {
    console.log('Action rejected');
  }
})();
```

### Rust (Local Integration)

```rust
use reqwest::Client;
use serde_json::json;
use std::time::Duration;
use tokio::time::sleep;

pub struct ChaseAIClient {
    base_url: String,
    client: Client,
}

impl ChaseAIClient {
    pub fn new(host: &str, port: u16) -> Self {
        Self {
            base_url: format!("http://{}:{}", host, port),
            client: Client::new(),
        }
    }

    pub async fn get_context(&self) -> anyhow::Result<serde_json::Value> {
        let response = self.client
            .get(&format!("{}/context", self.base_url))
            .send()
            .await?;
        Ok(response.json().await?)
    }

    pub async fn request_verification(
        &self,
        action: &str,
        reason: &str,
        context: Option<serde_json::Value>,
    ) -> anyhow::Result<serde_json::Value> {
        let payload = json!({
            "action": action,
            "reason": reason,
            "context": context.unwrap_or(json!({}))
        });

        let response = self.client
            .post(&format!("{}/verify", self.base_url))
            .json(&payload)
            .send()
            .await?;

        Ok(response.json().await?)
    }

    pub async fn check_verification_status(
        &self,
        verification_id: &str,
        timeout: Duration,
    ) -> anyhow::Result<serde_json::Value> {
        let start = std::time::Instant::now();

        loop {
            let response = self.client
                .get(&format!("{}/verify/{}", self.base_url, verification_id))
                .send()
                .await?;

            let result: serde_json::Value = response.json().await?;

            if result["status"].as_str() != Some("pending") {
                return Ok(result);
            }

            if start.elapsed() > timeout {
                anyhow::bail!("Verification {} timed out", verification_id);
            }

            sleep(Duration::from_secs(1)).await;
        }
    }
}

// Usage
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = ChaseAIClient::new("localhost", 8090);

    // Get context
    let context = client.get_context().await?;
    println!("Role: {}", context["role"]);

    // Request verification
    let result = client.request_verification(
        "Deploy to production",
        "User requested deployment",
        Some(json!({"version": "1.2.3"})),
    ).await?;

    let verification_id = result["verification_id"].as_str().unwrap();

    // Wait for approval
    let status = client
        .check_verification_status(verification_id, Duration::from_secs(300))
        .await?;

    if status["status"].as_str() == Some("approved") {
        println!("Action approved, proceeding...");
    } else {
        println!("Action rejected");
    }

    Ok(())
}
```

## Error Handling

Always implement proper error handling:

```python
try:
    context = client.get_context()
except requests.exceptions.ConnectionError:
    print("Error: Cannot connect to ChaseAI")
except requests.exceptions.HTTPError as e:
    if e.response.status_code == 404:
        print("Error: Context not found")
    elif e.response.status_code == 500:
        print("Error: Server error")
except TimeoutError:
    print("Error: Verification request timed out")
```

## Security Best Practices

1. **Store Configuration Securely**
   - Don't log configuration files
   - Don't share with untrusted parties
   - Use secure storage for sensitive data

2. **Validate Responses**
   - Always validate response format
   - Check status codes
   - Verify verification IDs

3. **Implement Retry Logic**
   - Use exponential backoff for retries
   - Set reasonable timeouts
   - Don't retry indefinitely

4. **Handle Verification Timeouts**
   - Set appropriate timeout values
   - Gracefully handle rejections
   - Log verification decisions

## Troubleshooting

### Connection Refused

- Ensure ChaseAI is running
- Check the correct host and port
- Verify firewall settings

### 404 Not Found

- Ensure the port has an instruction context configured
- Check that the endpoint path is correct

### 500 Internal Server Error

- Check ChaseAI logs for details
- Verify configuration is valid
- Restart ChaseAI if needed

### Verification Timeout

- Increase timeout value if needed
- Check if human is available to approve
- Verify verification endpoint is working

## Next Steps

- Read the [Verification Workflow Guide](./verification-workflow.md) for detailed workflow information
- Check the [API Reference](./api-reference.md) for complete endpoint documentation
- Review [Security Considerations](./security.md) for best practices
