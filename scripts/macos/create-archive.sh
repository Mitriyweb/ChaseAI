#!/bin/bash

# Create tar.gz archive for macOS
# This script packages the ChaseAI application into a tar.gz archive
# as a fallback to DMG creation

set -e

# Navigate to the project root
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cd "$SCRIPT_DIR/../.."

echo "📦 Creating tar.gz archive..."

# Configuration
APP_NAME="ChaseAI"
BINARY_NAME="chase-ai"
VERSION="${1:-0.1.0}"
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
  exit 1
fi

echo "✓ App bundle found"

# Create tar.gz archive
echo "🎨 Creating tar.gz archive..."
cd "${RELEASE_DIR}"
tar -czf "${ARCHIVE_NAME}" "${APP_NAME}.app"
cd - > /dev/null

# Verify archive was created
if [ ! -f "${ARCHIVE_PATH}" ]; then
  echo "❌ Error: Archive creation failed"
  exit 1
fi

echo "✓ Archive file created"

# Check archive file size
ARCHIVE_SIZE=$(stat -f%z "${ARCHIVE_PATH}" 2>/dev/null || stat -c%s "${ARCHIVE_PATH}" 2>/dev/null || echo "unknown")
echo "   Size: ${ARCHIVE_SIZE} bytes"

# Generate checksums
echo "🔐 Generating checksums..."
SHA256_FILE="${ARCHIVE_PATH}.sha256"
shasum -a 256 "${ARCHIVE_PATH}" > "${SHA256_FILE}"

# Display checksum
echo "   SHA256: $(cat ${SHA256_FILE})"

# Display file info
ARCHIVE_SIZE=$(du -h "${ARCHIVE_PATH}" | cut -f1)
echo ""
echo "✅ Archive created successfully!"
echo "   Name: ${ARCHIVE_NAME}"
echo "   Size: ${ARCHIVE_SIZE}"
echo "   Path: ${ARCHIVE_PATH}"
echo "   Checksum: ${SHA256_FILE}"
echo ""
echo "To extract the archive:"
echo "  tar -xzf ${ARCHIVE_NAME}"
echo "  cp -r ${APP_NAME}.app /Applications/"
echo ""
