# Proposal: Instruction Service

## Summary

Implement the instruction service that creates and manages instruction contexts for AI agents. Each port binding (from network-interface-management) will have an associated instruction context that defines allowed actions, verification requirements, and system integration points.

## Rationale

The instruction service is the core control mechanism of ChaseAI:

- **Execution governor**: Enforces what AI agents can and cannot do
- **Context isolation**: Each port has its own instruction context with specific permissions
- **Human-in-the-loop**: Verification requirements ensure human approval for critical actions
- **System integration**: Defines which system (WinSF, etc.) the agent is working with

This builds directly on network-interface-management by adding the application logic layer on top of the network infrastructure.

## Scope

### In Scope

- Instruction context data structure (system, role, base_instruction, allowed_actions, verification_required)
- Create/read/update/delete operations for instruction contexts
- Association between port bindings and instruction contexts
- Persistence of instruction contexts
- Basic HTTP/TCP server to serve instruction contexts on configured ports
- Validation of instruction requests against contexts

### Out of Scope

- Full agent execution environment (future phase)
- Complex workflow orchestration (future phase)
- Authentication/authorization (future phase)
- Instruction versioning and rollback (future enhancement)
- Multi-agent coordination (future phase)

## Dependencies

- **Blocks on**: `network-interface-management` (need port bindings to attach contexts)
- **Creates foundation for**: Future agent execution and workflow features

## User-Facing Impact

Users will be able to:

1. Create instruction contexts for each configured port
2. Define which system (e.g., "WinSF") the context applies to
3. Set the agent role (e.g., "execution-agent")
4. Specify base instructions that define agent behavior
5. Configure allowed actions (e.g., ["analyze", "propose", "execute-safe"])
6. Enable/disable verification requirements
7. AI agents connecting to ports will receive their instruction context as JSON

## Success Criteria

- [ ] Instruction contexts can be created and persisted
- [ ] Contexts are correctly associated with port bindings
- [ ] HTTP/TCP server serves contexts on configured ports
- [ ] Invalid requests are rejected with clear error messages
- [ ] Configuration survives application restart
- [ ] All unit tests pass with ≥85% coverage
- [ ] Integration test verifies end-to-end context creation and retrieval
