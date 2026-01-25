#!/bin/bash

# Build universal binary for macOS (x86_64 + aarch64)
# This script builds the ChaseAI application for both Intel and Apple Silicon architectures
# and combines them into a single universal binary.

set -e

echo "🔨 Building universal binary for macOS..."

# Configuration
BINARY_NAME="chase-ai"
RELEASE_DIR="target/release"
X86_64_DIR="${RELEASE_DIR}/x86_64"
AARCH64_DIR="${RELEASE_DIR}/aarch64"
UNIVERSAL_DIR="${RELEASE_DIR}/universal"

# Create output directories
mkdir -p "${X86_64_DIR}"
mkdir -p "${AARCH64_DIR}"
mkdir -p "${UNIVERSAL_DIR}"

# Build for x86_64
echo "📦 Building for x86_64..."
cargo build --release --target x86_64-apple-darwin

# Build for aarch64
echo "📦 Building for aarch64..."
cargo build --release --target aarch64-apple-darwin

# Create universal binary using lipo
echo "🔗 Creating universal binary..."
lipo -create \
  "target/x86_64-apple-darwin/release/${BINARY_NAME}" \
  "target/aarch64-apple-darwin/release/${BINARY_NAME}" \
  -output "${UNIVERSAL_DIR}/${BINARY_NAME}"

# Verify universal binary
echo "✅ Verifying universal binary..."
file "${UNIVERSAL_DIR}/${BINARY_NAME}"

# Check that both architectures are present
if file "${UNIVERSAL_DIR}/${BINARY_NAME}" | grep -q "x86_64"; then
  echo "✓ x86_64 architecture found"
else
  echo "✗ x86_64 architecture NOT found"
  exit 1
fi

if file "${UNIVERSAL_DIR}/${BINARY_NAME}" | grep -q "arm64"; then
  echo "✓ arm64 (aarch64) architecture found"
else
  echo "✗ arm64 (aarch64) architecture NOT found"
  exit 1
fi

# Copy to standard release directory for consistency
cp "${UNIVERSAL_DIR}/${BINARY_NAME}" "${RELEASE_DIR}/${BINARY_NAME}"

echo ""
echo "✅ Universal binary created successfully!"
echo "   Location: ${RELEASE_DIR}/${BINARY_NAME}"
echo ""
