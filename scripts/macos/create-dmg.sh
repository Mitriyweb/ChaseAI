#!/bin/bash

# Create installer for macOS
# Creates a tar.gz archive of the ChaseAI application

set -e

# Navigate to the project root
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cd "$SCRIPT_DIR/../.."

echo "📦 Creating macOS installer..."

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

# Verify app bundle exists
if [ ! -d "${APP_BUNDLE}" ]; then
  echo "❌ Error: App bundle not found at ${APP_BUNDLE}"
  echo "   Please run scripts/macos/build-macos-app.sh first"
  ls -la "${RELEASE_DIR}/" || echo "Release directory doesn't exist"
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

echo "✓ Archive created successfully"

# Check archive file size
ARCHIVE_SIZE=$(stat -f%z "${ARCHIVE_PATH}" 2>/dev/null || stat -c%s "${ARCHIVE_PATH}" 2>/dev/null || echo "0")
echo "   Size: ${ARCHIVE_SIZE} bytes"

if [ "${ARCHIVE_SIZE}" -lt 1000000 ]; then
  echo "⚠️  Warning: Archive file is very small (${ARCHIVE_SIZE} bytes)"
  exit 1
fi

# Generate checksums
echo "🔐 Generating checksums..."
SHA256_FILE="${ARCHIVE_PATH}.sha256"
shasum -a 256 "${ARCHIVE_PATH}" > "${SHA256_FILE}"

# Display checksum
echo "   SHA256: $(cat ${SHA256_FILE})"

# Display file info
ARCHIVE_SIZE=$(du -h "${ARCHIVE_PATH}" | cut -f1)
echo ""
echo "✅ Installer created successfully!"
echo "   Name: ${ARCHIVE_NAME}"
echo "   Size: ${ARCHIVE_SIZE}"
echo "   Path: ${ARCHIVE_PATH}"
echo "   Checksum: ${SHA256_FILE}"
echo ""
echo "To extract and install:"
echo "  tar -xzf ${ARCHIVE_NAME}"
echo "  cp -r ${APP_NAME}.app /Applications/"
echo ""
