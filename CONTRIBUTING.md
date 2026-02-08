# Contributing to ChaseAI

## Code Quality Standards

### Pre-commit Hooks

All commits **must** pass pre-commit hooks. This is **mandatory** and non-negotiable.

**DO NOT use `git commit --no-verify`**

Using `--no-verify` bypasses critical quality checks and is not permitted in this project.

### Pre-commit Validation

Before each commit, the following checks run automatically:

1. **Rust Formatting** - `cargo fmt --all -- --check`
2. **Markdown Linting** - `markdownlint-cli2`
3. **Rust Linting** - `cargo clippy`
4. **Unused Dependencies** - `cargo +nightly udeps`
5. **Test Organization** - Validates test file structure
6. **Build Verification** - Ensures code compiles

### If Pre-commit Fails

1. Fix the issues reported by the failing hook
2. Stage your changes: `git add .`
3. Try committing again: `git commit -m "your message"`
4. Pre-commit hooks will run again automatically

### Manual Hook Execution

To manually run all pre-commit hooks:

```bash
pre-commit run --all-files
```

## Testing Requirements

### Unit Tests

- Co-locate with source code using `#[cfg(test)]`
- Use descriptive test names with `test_` prefix
- Test edge cases and error conditions

### Integration Tests

- Store in `src/test/rs/` with `_integration.rs` suffix
- Test workflows across multiple components
- Use descriptive names

### Running Tests

```bash
# All tests
cargo test

# Unit tests only
cargo test --lib

# Integration tests only
cargo test --test '*'

# Specific test
cargo test test_name
```

## Code Style

- Follow Rust conventions enforced by `rustfmt`
- Use `cargo clippy` for linting suggestions
- Keep functions focused and well-documented
- Write clear commit messages

## Commit Messages

Use conventional commit format:

```text
type: description

Optional body with more details
```

Types: `feat`, `fix`, `docs`, `test`, `refactor`, `chore`

Example:

```text
feat: add download config functionality

- Implement JSON/YAML/Markdown export
- Add configuration preview dialog
- Add comprehensive tests
```

## Feature Development

1. Create a feature branch: `git checkout -b feature/your-feature`
2. Implement your changes
3. Add tests for new functionality
4. Ensure all pre-commit hooks pass
5. Create a pull request

## Beta Features

Beta features are gated behind the `beta` feature flag:

```bash
cargo build --features beta
```

Mark beta-only code with `#[cfg(feature = "beta")]`

## Questions?

See `.agent/AGENTS.md` for detailed agent rules and coding standards.
