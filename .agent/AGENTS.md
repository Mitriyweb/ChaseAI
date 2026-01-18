# Agent Rules

Agents working in this project MUST follow these rules without exception:

## 0. Follow Coding Standards

Agents **MUST** read and follow specialized coding standards in the `@/.agent/rules/` directory based on the task:

- `rust-coding-standards.md` - Rust rules, Cargo, safety, and performance.
- `project-constants.md` - Core constants and standards.
- `testing-standards.md` - Test structure and coverage.
- `code-quality-tools.md` - Pre-commit hooks, security.
- `documentation-standards.md` - README and changelog.
- `performance-standards.md` - Startup and memory limits.
- `backlog-roadmap-consistency.md` - BACKLOG vs ROADMAP sync.

## 1. No Skipping Tests & Linting

- **MUST** run `cargo test`, `cargo clippy`, and `cargo fmt`.
- All code changes require test coverage.

## Commit Checklist

- ✓ All tests pass: `cargo test`
- ✓ Code formatted: `cargo fmt`
- ✓ Clippy passes: `cargo clippy`
- ✓ Follow coding standards in `@/.agent/rules/`
