# Rust Testing Standards

## Test Organization

All Rust tests MUST be organized in the following structure:

### Unit Tests (Co-located)

Unit tests for specific modules are co-located with source code using the `#[cfg(test)]` module pattern:

```rust
// src/rs/module/mod.rs

pub fn my_function() {
    // implementation
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_function() {
        // test implementation
    }
}
```

### Integration Tests (Separate Directory)

Integration tests and end-to-end tests are stored in `src/test/rs/`:

```text
src/test/rs/
├── app.rs                    # Application integration tests
├── instruction_integration.rs # Instruction service tests
├── network_integration.rs    # Network service tests
└── ...
```

## Test File Naming

- **Unit tests**: Use `#[cfg(test)]` modules in source files
- **Integration tests**: Use `_integration.rs` suffix in `src/test/rs/`
- **Test functions**: Use `test_` prefix (e.g., `test_config_generation`)

## Test Execution

Run tests with:

```bash
# All tests
cargo test

# Unit tests only
cargo test --lib

# Integration tests only
cargo test --test '*'

# Specific test
cargo test test_my_function
```

## Test Guidelines

1. **Keep unit tests close to code** - Use `#[cfg(test)]` for unit tests
2. **Use integration tests for workflows** - Test multiple components together in `src/test/rs/`
3. **Use descriptive names** - Test names should describe what is being tested
4. **Test edge cases** - Include tests for error conditions and boundary cases
5. **Avoid mocking when possible** - Test real functionality, not mocks

## Property-Based Testing

When implementing property-based tests:

1. Use the testing framework specified in the design document
2. Annotate with requirement links: `**Validates: Requirements X.Y**`
3. Write smart generators that constrain to the input space
4. Test core logic across many inputs
5. Update PBT status after running tests

Aim for:

- **Unit tests**: 80%+ coverage for critical paths
- **Integration tests**: Cover main workflows and error scenarios
- **Property-based tests**: Cover universal properties and invariants
