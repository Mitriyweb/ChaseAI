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

# Function to create DMG using hdiutil (fallback)
create_dmg_hdiutil() {
  # Create a temporary DMG
  TEMP_DMG="${RELEASE_DIR}/temp-${BINARY_NAME}.dmg"

  echo "🎨 Creating DMG with hdiutil..."
  echo "   Source folder: ${TEMP_DMG_DIR}"
  echo "   Contents:"
  ls -la "${TEMP_DMG_DIR}/"

  hdiutil create -volname "${APP_NAME}" \
    -srcfolder "${TEMP_DMG_DIR}" \
    -ov -format UDZO \
    "${TEMP_DMG}"

  # Move to final location
  mv "${TEMP_DMG}" "${DMG_PATH}"
}

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

# Create DMG using create-dmg if available, otherwise use hdiutil
if command -v create-dmg &> /dev/null; then
  echo "🎨 Creating DMG with create-dmg..."
  if create-dmg \
    --volname "${APP_NAME}" \
    --volicon "resources/icon.png" \
    --window-pos 200 120 \
    --window-size 800 400 \
    --icon-size 100 \
    --icon "${APP_NAME}.app" 200 190 \
    --hide-extension "${APP_NAME}.app" \
    --app-drop-link 600 190 \
    "${DMG_PATH}" \
    "${TEMP_DMG_DIR}/"; then
    echo "✓ DMG created with create-dmg"
  else
    echo "⚠️  create-dmg failed, falling back to hdiutil..."
    create_dmg_hdiutil
  fi
else
  echo "⚠️  create-dmg not found, using hdiutil..."
  create_dmg_hdiutil
fi

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

if [ "${DMG_SIZE}" -eq 0 ]; then
  echo "❌ Error: DMG file is empty (0 bytes)"
  exit 1
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
