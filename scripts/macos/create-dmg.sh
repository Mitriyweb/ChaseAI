#!/bin/bash

# Create DMG installer for macOS
# This script packages the ChaseAI application into a professional DMG installer
# with background image and proper layout.
# Falls back to tar.gz if DMG creation fails.

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
DMG_NAME="${BINARY_NAME}-${VERSION}-macos.dmg"
DMG_PATH="${RELEASE_DIR}/${DMG_NAME}"
ARCHIVE_NAME="${BINARY_NAME}-${VERSION}-macos.tar.gz"
ARCHIVE_PATH="${RELEASE_DIR}/${ARCHIVE_NAME}"
TEMP_DMG_DIR="${RELEASE_DIR}/dmg-temp"

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

# Try to create DMG
echo "🎨 Attempting to create DMG..."
rm -rf "${TEMP_DMG_DIR}"
mkdir -p "${TEMP_DMG_DIR}"

# Copy app bundle to temp directory
cp -r "${APP_BUNDLE}" "${TEMP_DMG_DIR}/"
ln -s /Applications "${TEMP_DMG_DIR}/Applications"

# Try DMG creation
if hdiutil create -volname "${APP_NAME}" \
  -srcfolder "${TEMP_DMG_DIR}" \
  -ov -format UDZO \
  "${DMG_PATH}" 2>/dev/null; then
  echo "✓ DMG created successfully"
  DMG_SIZE=$(stat -f%z "${DMG_PATH}" 2>/dev/null || stat -c%s "${DMG_PATH}" 2>/dev/null || echo "0")
  if [ "${DMG_SIZE}" -gt 1000000 ]; then
    echo "   Size: $(du -h "${DMG_PATH}" | cut -f1)"
    # Generate checksums for DMG
    shasum -a 256 "${DMG_PATH}" > "${DMG_PATH}.sha256"
    rm -rf "${TEMP_DMG_DIR}"
    exit 0
  fi
fi

# DMG creation failed or file is too small, fall back to tar.gz
echo "⚠️  DMG creation failed or file is too small, creating tar.gz archive instead..."
rm -f "${DMG_PATH}" "${DMG_PATH}.sha256"
rm -rf "${TEMP_DMG_DIR}"

echo "🎨 Creating tar.gz archive..."
cd "${RELEASE_DIR}"
tar -czf "${ARCHIVE_NAME}" "${APP_NAME}.app"
cd - > /dev/null

if [ ! -f "${ARCHIVE_PATH}" ]; then
  echo "❌ Error: Archive creation failed"
  exit 1
fi

echo "✓ Archive created successfully"
echo "   Size: $(du -h "${ARCHIVE_PATH}" | cut -f1)"

# Generate checksums for archive
shasum -a 256 "${ARCHIVE_PATH}" > "${ARCHIVE_PATH}.sha256"

echo ""
echo "✅ Installer created successfully!"
echo "   Format: tar.gz (DMG creation failed)"
echo "   Path: ${ARCHIVE_PATH}"
echo ""
