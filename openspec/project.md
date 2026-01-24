# Project Context

## Purpose

ChaseAI is a local control and orchestration system for AI agents that ensures
valid instructions, verifiable task execution, and mandatory human-in-the-loop
acceptance. It acts as an execution governor between AI agents and the user’s
real system.

## Tech Stack

- **Languages**: Rust (Core), TypeScript (Tooling)
- **UI**: Native system tray (cross-platform)
- **Runtime**: Bun (JavaScript/TypeScript tooling)
- **Tooling**: Cargo, Bun, OpenSpec, pre-commit

## Project Conventions

### Code Style

- **Rust**: Follow standard Rust naming (snake_case for functions/variables,
  PascalCase for types). Use `clippy` and `rustfmt` with specific project
  thresholds.
- **Directory Structure**:
  - Rust source: `src/rs/`
  - Rust tests: `src/test/rs/`
- **Error Handling**: Use `anyhow` for applications, no `unwrap()` in library
  code.

### Architecture Patterns

- **Local-first**: Critical decisions happen locally.
- **Contractor Model**: AI agents execute within fixed contexts.
- **Step-level verification**: Mandatory check for each step.

### Testing Strategy

- Unit tests co-located in `src/rs/` within `mod tests` OR located in
  `src/test/rs/` for integration/extended testing.
- Mandatory 85% coverage baseline.

### Git Workflow

- Change-id based proposals with OpenSpec.
- Mandatory pre-commit hooks (Clippy, Rustfmt, Markdownlint).

## Domain Context

- **Instruction Context**: Secure environment for agent execution.
- **Execution Governor**: Intercepts and validates agent actions.

## Important Constraints

- No improvisation without a plan.
- Human-in-the-loop is mandatory for system-altering actions.
