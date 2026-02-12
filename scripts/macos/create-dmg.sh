#!/bin/bash

# Create DMG installer for macOS
# This script packages the ChaseAI application into a professional DMG installer
# with background image and proper layout.

set -e

# Navigate to the project root
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cd "$SCRIPT_DIR/../.."

echo "📦 Creating DMG installer..."

# Configuration
APP_NAME="ChaseAI"
BINARY_NAME="chase-ai"
VERSION="${1:-0.1.0}"
RELEASE_DIR="target/release"
APP_BUNDLE="${RELEASE_DIR}/${APP_NAME}.app"
DMG_NAME="${BINARY_NAME}-${VERSION}-macos.dmg"
DMG_PATH="${RELEASE_DIR}/${DMG_NAME}"
TEMP_DMG_DIR="${RELEASE_DIR}/dmg-temp"

echo "   Version: ${VERSION}"
echo "   App Bundle: ${APP_BUNDLE}"
echo "   DMG Path: ${DMG_PATH}"

# Verify app bundle exists
if [ ! -d "${APP_BUNDLE}" ]; then
  echo "❌ Error: App bundle not found at ${APP_BUNDLE}"
  echo "   Please run scripts/macos/build-macos-app.sh first"
  ls -la "${RELEASE_DIR}/" || echo "Release directory doesn't exist"
  exit 1
fi

echo "✓ App bundle found"

# Clean up any previous DMG creation
rm -rf "${TEMP_DMG_DIR}"
mkdir -p "${TEMP_DMG_DIR}"

# Copy app bundle to temp directory
echo "📋 Preparing DMG contents..."
cp -r "${APP_BUNDLE}" "${TEMP_DMG_DIR}/"

# Create symlink to Applications folder for drag-and-drop installation
ln -s /Applications "${TEMP_DMG_DIR}/Applications"

# List contents for debugging
echo "   DMG contents:"
ls -la "${TEMP_DMG_DIR}/"

# Create DMG using hdiutil
echo "🎨 Creating DMG with hdiutil..."
hdiutil create -volname "${APP_NAME}" \
  -srcfolder "${TEMP_DMG_DIR}" \
  -ov -format UDZO \
  "${DMG_PATH}"

# Verify DMG was created
if [ ! -f "${DMG_PATH}" ]; then
  echo "❌ Error: DMG creation failed"
  echo "   Expected path: ${DMG_PATH}"
  ls -la "${RELEASE_DIR}/" || echo "Release directory doesn't exist"
  exit 1
fi

echo "✓ DMG file created"

# Check DMG file size
DMG_SIZE=$(stat -f%z "${DMG_PATH}" 2>/dev/null || stat -c%s "${DMG_PATH}" 2>/dev/null || echo "unknown")
echo "   Size: ${DMG_SIZE} bytes"

if [ "${DMG_SIZE}" -lt 1000000 ]; then
  echo "⚠️  Warning: DMG file is very small (${DMG_SIZE} bytes)"
  echo "   This might indicate an issue with the DMG creation"
fi

# Generate checksums
echo "🔐 Generating checksums..."
SHA256_FILE="${DMG_PATH}.sha256"
shasum -a 256 "${DMG_PATH}" > "${SHA256_FILE}"

# Display checksum
echo "   SHA256: $(cat ${SHA256_FILE})"

# Clean up temp directory
rm -rf "${TEMP_DMG_DIR}"

# Display file info
DMG_SIZE=$(du -h "${DMG_PATH}" | cut -f1)
echo ""
echo "✅ DMG created successfully!"
echo "   Name: ${DMG_NAME}"
echo "   Size: ${DMG_SIZE}"
echo "   Path: ${DMG_PATH}"
echo "   Checksum: ${SHA256_FILE}"
echo ""
echo "To test the DMG:"
echo "  hdiutil attach ${DMG_PATH}"
echo "  # Drag ChaseAI.app to Applications folder"
echo "  hdiutil detach /Volumes/${APP_NAME}"
echo ""
