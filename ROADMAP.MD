# ChaseAI — Roadmap

## Controlled Local AI Execution Platform

---

## Project Mission

ChaseAI is a local control and orchestration system for AI agents that ensures:

- valid instructions,
- verifiable task execution,
- mandatory human-in-the-loop acceptance.

ChaseAI acts as an **execution governor** between AI agents and the user’s real
system.

---

## Architectural Principles

1. **Local-first** — all critical decisions happen locally.
2. **AI is a contractor, not an owner** — the agent executes, not decides.
3. **Explicit workflows** — no improvisation without a plan.
4. **Step-level verification** — control per step, not per conversation.
5. **Rust Core + Web UI** — safety, low resource usage, extensibility.

---

## Stage 1: Core & Network Control (MVP)

### Goal

Provide a secure and controlled entry point for AI agents into the local system
via ChaseAI.

---

### 1.1 Network Interface Management

**Capabilities:**

- detect available IPs:
  - loopback
  - LAN
  - public (optional)
- bind ports to roles:
  - instruction
  - verification

**Tray UI:**

- IP selection
- port configuration
- port role assignment
- enable / disable switches

---

### 1.2 Instruction Service

Each port is associated with an **Instruction Context**.

```json
{
  "system": "WinSF",
  "role": "execution-agent",
  "base_instruction": "...",
  "allowed_actions": ["analyze", "propose", "execute-safe"],
  "verification_required": true
}
```
