# Proposal: Network Interface Management

## Summary

Implement network interface detection and port management capabilities that enable ChaseAI to bind services to specific network interfaces (loopback, LAN, public) and assign roles to ports (instruction, verification, workflow).

## Rationale

As the foundational layer of ChaseAI's MVP, network interface management provides:

- **Security control**: Explicitly control which network interfaces can access AI agent services
- **Role-based port assignment**: Different ports for different service types (instruction, verification, workflow)
- **Local-first architecture**: Enable loopback-only operation for maximum security
- **LAN/Public flexibility**: Optional exposure for team workflows while maintaining control

This capability is essential before implementing the Instruction Service, as it provides the network layer that instruction contexts will bind to.

## Scope

### In Scope

- Detect available network interfaces (loopback, LAN, public)
- Bind ports to specific interfaces
- Assign roles to ports (instruction, verification, workflow)
- Basic tray UI for IP selection, port configuration, and role assignment
- Enable/disable switches for port bindings
- Persistent configuration storage

### Out of Scope

- Full TCP server implementation (deferred to instruction-service)
- Authentication/authorization (future phase)
- TLS/SSL configuration (future phase)
- Dynamic port discovery protocols
- Advanced firewall integration
- Web UI (only tray UI for MVP)

## Dependencies

- Creates foundation for: `instruction-service`
- No blocking dependencies on other changes

## User-Facing Impact

Users will be able to:

1. See available network interfaces in the tray UI
2. Select which interface to bind services to
3. Configure port assignments for different service roles
4. Enable/disable specific port bindings
5. Operate in fully local mode (loopback only) or expose to LAN/public as needed

## Success Criteria

- [ ] System can detect all available network interfaces (loopback, LAN, public)
- [ ] Ports can be successfully bound to chosen interfaces
- [ ] Role assignments (instruction, verification, workflow) can be configured
- [ ] Configuration is persisted and loaded on startup
- [ ] Tray UI displays interface options and port configuration
- [ ] Enable/disable switches function correctly
- [ ] All unit tests pass with ≥85% coverage
- [ ] Integration test verifies end-to-end interface detection and port binding
