#!/bin/bash

# Create tar.gz archive for macOS
# This script packages the ChaseAI application into a tar.gz archive

set -e

# Navigate to the project root
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cd "$SCRIPT_DIR/../.."

echo "📦 Creating macOS tar.gz archive..."

# Configuration
APP_NAME="ChaseAI"
BINARY_NAME="chase-ai"
VERSION="${1:-$(grep '^version =' Cargo.toml | head -1 | cut -d '"' -f2)}"
RELEASE_DIR="target/release"
APP_BUNDLE="${RELEASE_DIR}/${APP_NAME}.app"
ARCHIVE_NAME="${BINARY_NAME}-${VERSION}-macos.tar.gz"
ARCHIVE_PATH="${RELEASE_DIR}/${ARCHIVE_NAME}"

echo "   Version: ${VERSION}"
echo "   App Bundle: ${APP_BUNDLE}"
echo "   Archive Path: ${ARCHIVE_PATH}"

# Verify app bundle exists
if [ ! -d "${APP_BUNDLE}" ]; then
  echo "❌ Error: App bundle not found at ${APP_BUNDLE}"
  echo "   Please run scripts/macos/build-macos-app.sh first"
  exit 1
fi

echo "✓ App bundle found"

# Create tar.gz archive
echo "🎨 Creating tar.gz archive..."
rm -f "${ARCHIVE_PATH}"
cd "${RELEASE_DIR}"
tar -czf "${ARCHIVE_NAME}" "${APP_NAME}.app"
cd - > /dev/null

# Verify archive was created
if [ ! -f "${ARCHIVE_PATH}" ]; then
  echo "❌ Error: Archive creation failed"
  exit 1
fi

echo "✓ Archive created successfully"

# Generate checksums
echo "🔐 Generating checksums..."
SHA256_FILE="${ARCHIVE_PATH}.sha256"
shasum -a 256 "${ARCHIVE_PATH}" > "${SHA256_FILE}"

# Display checksum
echo "   SHA256: $(cat ${SHA256_FILE} | awk '{print $1}')"

# Display file info
if [[ "$OSTYPE" == "darwin"* ]]; then
    ARCHIVE_SIZE=$(du -h "${ARCHIVE_PATH}" | cut -f1)
else
    ARCHIVE_SIZE=$(ls -lh "${ARCHIVE_PATH}" | awk '{print $5}')
fi

echo ""
echo "✅ Archive created successfully!"
echo "   Name: ${ARCHIVE_NAME}"
echo "   Size: ${ARCHIVE_SIZE}"
echo "   Path: ${ARCHIVE_PATH}"
echo "   Checksum: ${SHA256_FILE}"
echo ""
