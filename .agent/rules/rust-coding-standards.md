# Rust Coding Standards

## Dependency Management

### ⚠️ CRITICAL: Use exact versions only

**NEVER use semver ranges (^ or ~) in Cargo.toml dependencies.**

```toml
# ❌ Wrong - allows automatic updates
[dependencies]
tokio = "1.0"
serde = "^1.0"

# ✅ Correct - exact versions only
[dependencies]
tokio = "1.36.0"
serde = "1.0.197"
```

---

## Rust Configuration

### ⚠️ CRITICAL: Enforce Lints & Formatting

Zero tolerance for lint warnings in CI. Lints are enforced via `Cargo.toml`.

### Tooling Requirements

- **Rustfmt**: All code MUST be formatted with `cargo fmt`. Settings in `rustfmt.toml`.
- **Clippy**: All code MUST pass `cargo clippy`. Thresholds in `clippy.toml`.
- **Edition 2021**: All crates MUST use `edition = "2021"`.

---

## Lints & Complexity Thresholds

AI agents MUST respect these thresholds defined in `clippy.toml`:

- **Cognitive Complexity**: Max 30
- **Type Complexity**: Max 100 (discourages deep generics)
- **Function Arguments**: Max 8
- **Function Lines**: Max 150

---

## Safety & Ownership

### ⚠️ CRITICAL: No `unsafe` without justification

**NEVER use `unsafe` code unless absolutely necessary.**
It MUST have a `// SAFETY: <reason>` comment.

### Ownership Patterns

- **Stack over Heap**: Prefer `&str` over `String` where possible.
- **Avoid `.clone()`**: Use references or `Arc` for shared data.

---

## Error Handling

### ⚠️ CRITICAL: No `.unwrap()` or `.expect()` in library code

- **Library Code**: MUST return `Result` or `Option`.
- **Lints**: `unwrap_used`, `expect_used`, `panic`, and `todo` are DENIED in production code.

---

## Testing Standards

- **Unit Tests**: MUST be in `mod tests` with `#[cfg(test)]` at the bottom of the file.
- **Integration Tests**: In `tests/` directory.
- **Doc Tests**: Required for public APIs.

## Compiler & Linter Warnings

### ⚠️ CRITICAL: Fix warnings, never suppress them

**NEVER use `#[allow(...)]` to hide compiler or clippy warnings.**
**NEVER use `[lints]` in `Cargo.toml` to suppress warnings globally.**

Warnings MUST be fixed at the source:

```rust
# ❌ Wrong - hiding the problem
#![allow(unexpected_cfgs)]
#[allow(dead_code)]
fn unused_function() {}

# ❌ Wrong - suppressing in Cargo.toml
[lints.rust]
unexpected_cfgs = "allow"

# ✅ Correct - fix the root cause
// Update dependencies, fix code, or remove unused code
```

**Allowed exceptions** (only with explicit justification):

- `#[allow(clippy::all)]` - ONLY for generated code or FFI bindings
- `#[allow(unsafe_code)]` - ONLY with `// SAFETY: <reason>` comment
- Platform-specific code with `#[cfg(...)]` - acceptable

**Process for warnings:**

1. Understand the warning
2. Fix the underlying issue (update deps, refactor code, remove unused items)
3. If truly unavoidable, document WHY in a comment
4. Get code review approval before suppressing
