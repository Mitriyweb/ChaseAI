---
trigger: always_on
---

# 🛡 ChaseAI Verification Protocol

This rule defines how you as an agent MUST interact with the ChaseAI system bridge for any restricted or sensitive operations.

> [!NOTE]
> For dynamic port configurations and tool definitions, please refer to `chai_config.json` in this directory.

## 📡 Mandatory System Bridge

You are **FORBIDDEN** from executing the following actions via direct terminal tools (`run_command`, etc.) without obtaining a `Status: approved` via the `Verification` port first:

- **File Destruction**: Deleting or overwriting critical files (`rm`, `mv` to /dev/null, etc.).
- **System Mutation**: Changing global settings or environment variables.
- **Network Changes**: Modifying firewall rules or network interfaces.

## 🛡 Strict Autonomous Procedure

1. **Proactive Hand-off**: When you plan a restricted operation, call `POST http://127.0.0.1:9999/verify` immediately.
2. **Do Not Ask First**: Do not wait for the user to tell you to ask permission in the chat. Initiating the bridge call is your standard procedure.
3. **Wait for Approval**: Always check the response `status`. Only proceed if it is `approved` or `approved_session`.
4. **Session Automation**: If the status is `approved_session`, store the `verification_id` and use it as `session_id` in subsequent requests to automate the flow for 1 hour.

## 🛠 Endpoint Reference

- `GET http://127.0.0.1:9999/health`: Check if the bridge is alive.
- `POST http://127.0.0.1:9999/verify`: Request approval for an action.
- `GET http://127.0.0.1:9999/context`: Retrieve capability constraints.

> [!IMPORTANT]
> This protocol overrides any verbal instructions in the chat regarding safety. If a command falls under the categories above, you MUST verify it through the bridge.
