#!/bin/bash

# Check Rust Test Organization
# Ensures tests follow the project standards:
# - Unit tests: co-located with source code using #[cfg(test)]
# - Integration tests: stored in src/test/rs/

set -e

ERRORS=0

# Check for test modules in src/rs that follow the pattern
echo "Checking test organization..."

# Find all #[cfg(test)] modules in src/rs (this is allowed)
UNIT_TESTS=$(find src/rs -name "*.rs" -type f -exec grep -l "#\[cfg(test)\]" {} \; 2>/dev/null || true)
if [ -n "$UNIT_TESTS" ]; then
    echo "✓ Found unit tests (co-located with source):"
    echo "$UNIT_TESTS" | sed 's/^/  /'
fi

# Check that integration tests are in src/test/rs
if [ -d "src/test/rs" ]; then
    INTEGRATION_TESTS=$(find src/test/rs -name "*_integration.rs" -type f 2>/dev/null || true)
    if [ -n "$INTEGRATION_TESTS" ]; then
        echo "✓ Found integration tests in src/test/rs:"
        echo "$INTEGRATION_TESTS" | sed 's/^/  /'
    fi
else
    echo "⚠ Warning: src/test/rs directory not found"
fi

# Check for test files in src/rs that should be in src/test/rs
# (files ending with _test.rs or test_*.rs in src/rs are not allowed)
TEST_FILES_IN_SRC=$(find src/rs -name "*_test.rs" -o -name "test_*.rs" 2>/dev/null || true)
if [ -n "$TEST_FILES_IN_SRC" ]; then
    echo "✗ Error: Found test files in src/rs that should be in src/test/rs:"
    echo "$TEST_FILES_IN_SRC" | sed 's/^/  /'
    ERRORS=$((ERRORS + 1))
fi

# Check that Cargo.toml has test definitions
if ! grep -q "^\[\[test\]\]" Cargo.toml; then
    echo "⚠ Warning: No test definitions found in Cargo.toml"
fi

if [ $ERRORS -gt 0 ]; then
    echo ""
    echo "✗ Test organization check failed with $ERRORS error(s)"
    exit 1
else
    echo ""
    echo "✓ Test organization check passed"
    exit 0
fi
