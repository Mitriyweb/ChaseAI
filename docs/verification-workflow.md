# Verification Workflow Guide

## Overview

The verification workflow enables human-in-the-loop approval for AI agent actions. This guide explains how the workflow operates and how to implement it in your AI agent.

## Workflow Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                    AI Agent                                  │
│  1. Get instruction context                                 │
│  2. Determine if action needs verification                  │
│  3. Request verification                                    │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                    ChaseAI Server                            │
│  1. Receive verification request                            │
│  2. Create verification record                              │
│  3. Notify human user                                       │
│  4. Return verification_id                                  │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                    Human User                                │
│  1. Review action details                                   │
│  2. Approve or reject                                       │
│  3. ChaseAI updates verification status                     │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                    AI Agent                                  │
│  1. Poll for verification status                            │
│  2. Receive approval/rejection                              │
│  3. Proceed or abort action                                 │
└─────────────────────────────────────────────────────────────┘
```

## Request Format

### POST /verify

Request body:
```json
{
  "action": "string - Description of the action",
  "reason": "string - Why this action is needed",
  "context": {
    "key": "value - Additional context (optional)"
  }
}
```

Example:
```json
{
  "action": "Deploy application to production",
  "reason": "User requested deployment of version 1.2.3",
  "context": {
    "version": "1.2.3",
    "environment": "production",
    "rollback_plan": "Revert to 1.2.2 if issues detected"
  }
}
```

## Response Format

### Verification Request Response

```json
{
  "status": "pending",
  "verification_id": "verify-abc123def456",
  "message": "Verification request submitted"
}
```

### Verification Status Response

```json
{
  "status": "approved|rejected|pending",
  "verification_id": "verify-abc123def456",
  "message": "Approved by user",
  "timestamp": "2026-01-28T10:35:00Z",
  "approved_by": "user@example.com"
}
```

## Status Codes

### HTTP Status Codes

| Code | Meaning |
|------|---------|
| 200 | Request successful |
| 400 | Bad request (invalid format) |
| 404 | Verification not found |
| 500 | Server error |

### Verification Status Values

| Status | Meaning |
|--------|---------|
| pending | Waiting for human approval |
| approved | Action approved, proceed |
| rejected | Action rejected, do not proceed |

## Implementation Steps

### Step 1: Check if Verification is Required

```python
context = client.get_context()
if context["verification_required"]:
    # Request verification
    result = client.request_verification(
        action="Deploy to production",
        reason="User requested deployment"
    )
else:
    # Proceed without verification
    proceed_with_action()
```

### Step 2: Request Verification

```python
result = client.request_verification(
    action="Deploy to production",
    reason="User requested deployment",
    context={
        "version": "1.2.3",
        "environment": "production"
    }
)

verification_id = result["verification_id"]
print(f"Verification requested: {verification_id}")
```

### Step 3: Poll for Status

```python
import time

def wait_for_verification(client, verification_id, timeout=300):
    """Wait for verification with timeout"""
    start_time = time.time()

    while time.time() - start_time < timeout:
        status = client.check_verification_status(verification_id)

        if status["status"] == "approved":
            return True
        elif status["status"] == "rejected":
            return False

        # Wait before polling again
        time.sleep(2)

    # Timeout
    raise TimeoutError(f"Verification timed out after {timeout}s")

# Usage
if wait_for_verification(client, verification_id):
    print("Action approved!")
    proceed_with_action()
else:
    print("Action rejected")
    abort_action()
```

### Step 4: Handle Results

```python
try:
    if wait_for_verification(client, verification_id, timeout=300):
        # Approved - proceed with action
        result = perform_action()
        log_action_result(result)
    else:
        # Rejected - abort action
        log_rejection(verification_id)
        notify_user("Action was rejected")

except TimeoutError:
    # Timeout - abort action
    log_timeout(verification_id)
    notify_user("Verification request timed out")
    abort_action()
```

## Polling Strategy

### Recommended Polling Intervals

- **Initial poll**: Immediately after request
- **Subsequent polls**: Every 1-2 seconds
- **Maximum timeout**: 5-10 minutes
- **Maximum retries**: None (use timeout instead)

### Exponential Backoff Example

```python
import time

def wait_for_verification_with_backoff(client, verification_id, timeout=300):
    """Wait for verification with exponential backoff"""
    start_time = time.time()
    poll_interval = 1  # Start with 1 second
    max_interval = 10  # Cap at 10 seconds

    while time.time() - start_time < timeout:
        status = client.check_verification_status(verification_id)

        if status["status"] != "pending":
            return status

        # Wait with exponential backoff
        time.sleep(poll_interval)
        poll_interval = min(poll_interval * 1.5, max_interval)

    raise TimeoutError(f"Verification timed out after {timeout}s")
```

## Error Scenarios

### Scenario 1: Network Error During Request

```python
try:
    result = client.request_verification(...)
except ConnectionError:
    # Retry with exponential backoff
    retry_count = 0
    while retry_count < 3:
        try:
            result = client.request_verification(...)
            break
        except ConnectionError:
            retry_count += 1
            time.sleep(2 ** retry_count)
    else:
        # Failed after retries
        abort_action()
```

### Scenario 2: Verification Timeout

```python
try:
    status = wait_for_verification(client, verification_id, timeout=300)
except TimeoutError:
    # Log timeout
    log_event("verification_timeout", {
        "verification_id": verification_id,
        "action": "Deploy to production"
    })

    # Notify user
    notify_user("Verification request timed out after 5 minutes")

    # Abort action
    abort_action()
```

### Scenario 3: Verification Rejected

```python
status = wait_for_verification(client, verification_id)

if status["status"] == "rejected":
    # Log rejection
    log_event("verification_rejected", {
        "verification_id": verification_id,
        "reason": status.get("message", "No reason provided")
    })

    # Notify user
    notify_user(f"Action rejected: {status.get('message')}")

    # Abort action
    abort_action()
```

## Best Practices

### 1. Always Check verification_required

```python
context = client.get_context()
if context["verification_required"]:
    # Request verification
else:
    # Proceed without verification
```

### 2. Provide Clear Action Descriptions

```python
# Good
result = client.request_verification(
    action="Deploy version 1.2.3 to production",
    reason="User requested deployment after testing"
)

# Bad
result = client.request_verification(
    action="Deploy",
    reason="Deploy"
)
```

### 3. Include Relevant Context

```python
result = client.request_verification(
    action="Delete database backup",
    reason="Cleanup old backups",
    context={
        "backup_date": "2026-01-01",
        "size_gb": 500,
        "retention_policy": "Keep 30 days"
    }
)
```

### 4. Implement Proper Timeouts

```python
# Set reasonable timeout based on action criticality
if is_critical_action:
    timeout = 600  # 10 minutes for critical actions
else:
    timeout = 300  # 5 minutes for normal actions

status = wait_for_verification(client, verification_id, timeout=timeout)
```

### 5. Log All Verification Events

```python
log_event("verification_requested", {
    "verification_id": verification_id,
    "action": action,
    "timestamp": datetime.now().isoformat()
})

log_event("verification_completed", {
    "verification_id": verification_id,
    "status": status["status"],
    "timestamp": datetime.now().isoformat()
})
```

## Timeout Behavior

### Default Timeout: 5 minutes

If no human approval is received within 5 minutes:
1. Verification is automatically rejected
2. AI agent receives rejection status
3. Action is aborted
4. Event is logged

### Custom Timeouts

You can set custom timeouts based on action type:

```python
TIMEOUT_CONFIG = {
    "deploy": 600,           # 10 minutes
    "delete": 300,           # 5 minutes
    "modify_config": 180,    # 3 minutes
    "read_only": 0           # No verification needed
}

timeout = TIMEOUT_CONFIG.get(action_type, 300)
status = wait_for_verification(client, verification_id, timeout=timeout)
```

## Verification Workflow Examples

### Example 1: Simple Deployment

```python
def deploy_application(version):
    client = ChaseAIClient()

    # Get context
    context = client.get_context()

    if not context["verification_required"]:
        # Deploy without verification
        return perform_deployment(version)

    # Request verification
    result = client.request_verification(
        action=f"Deploy version {version} to production",
        reason="Automated deployment pipeline",
        context={"version": version}
    )

    # Wait for approval
    try:
        status = wait_for_verification(
            client,
            result["verification_id"],
            timeout=600
        )

        if status["status"] == "approved":
            return perform_deployment(version)
        else:
            raise Exception("Deployment rejected")

    except TimeoutError:
        raise Exception("Deployment verification timed out")
```

### Example 2: Database Migration

```python
def migrate_database(migration_id):
    client = ChaseAIClient()

    # Request verification with detailed context
    result = client.request_verification(
        action=f"Execute database migration {migration_id}",
        reason="Scheduled maintenance window",
        context={
            "migration_id": migration_id,
            "estimated_duration": "30 minutes",
            "rollback_available": True,
            "affected_tables": ["users", "orders"],
            "maintenance_window": "2026-01-28 02:00-03:00 UTC"
        }
    )

    # Wait for approval with longer timeout
    status = wait_for_verification(
        client,
        result["verification_id"],
        timeout=1800  # 30 minutes
    )

    if status["status"] == "approved":
        execute_migration(migration_id)
    else:
        cancel_migration(migration_id)
```

## Monitoring and Logging

### Key Events to Log

1. **verification_requested** - When verification is requested
2. **verification_approved** - When action is approved
3. **verification_rejected** - When action is rejected
4. **verification_timeout** - When verification times out
5. **verification_error** - When an error occurs

### Example Logging

```python
import logging

logger = logging.getLogger(__name__)

def log_verification_event(event_type, details):
    logger.info(f"Verification event: {event_type}", extra={
        "event_type": event_type,
        "verification_id": details.get("verification_id"),
        "action": details.get("action"),
        "status": details.get("status"),
        "timestamp": datetime.now().isoformat()
    })
```

## Next Steps

- Review the [AI Integration Guide](./ai-integration.md) for implementation examples
- Check the [API Reference](./api-reference.md) for complete endpoint documentation
- Read [Security Considerations](./security.md) for best practices
