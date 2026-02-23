#!/bin/bash

# Create DMG installer for macOS
# This script packages the ChaseAI application into a DMG file

set -e

# Navigate to the project root
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cd "$SCRIPT_DIR/../.."

echo "📦 Creating macOS DMG installer..."

# Configuration
APP_NAME="ChaseAI"
BINARY_NAME="chase-ai"
VERSION="${1:-$(grep '^version =' Cargo.toml | head -1 | cut -d '"' -f2)}"
RELEASE_DIR="target/release"
APP_BUNDLE="${RELEASE_DIR}/${APP_NAME}.app"
DMG_NAME="${BINARY_NAME}-${VERSION}-macos.dmg"
DMG_PATH="${RELEASE_DIR}/${DMG_NAME}"

echo "   Version: ${VERSION}"
echo "   App Bundle: ${APP_BUNDLE}"
echo "   DMG Path: ${DMG_PATH}"

# Verify app bundle exists
if [ ! -d "${APP_BUNDLE}" ]; then
  echo "❌ Error: App bundle not found at ${APP_BUNDLE}"
  echo "   Please run scripts/macos/build-macos-app.sh first"
  exit 1
fi

echo "✓ App bundle found"

# Check if running on macOS
if [[ "$OSTYPE" != "darwin"* ]]; then
    echo "⚠️  Warning: hdiutil is only available on macOS. Skipping DMG creation on this platform."
    # On non-macOS, we don't want to fail the build if it's just a check,
    # but for a real release it must be run on macOS.
    exit 0
fi

# Create DMG
echo "🎨 Creating DMG..."
rm -f "${DMG_PATH}"
hdiutil create -volname "${APP_NAME}" -srcfolder "${APP_BUNDLE}" -ov -format UDZO "${DMG_PATH}"

# Verify DMG was created
if [ ! -f "${DMG_PATH}" ]; then
  echo "❌ Error: DMG creation failed"
  exit 1
fi

echo "✓ DMG created successfully"

# Generate checksums
echo "🔐 Generating checksums..."
SHA256_FILE="${DMG_PATH}.sha256"
shasum -a 256 "${DMG_PATH}" > "${SHA256_FILE}"

# Display checksum
echo "   SHA256: $(cat ${SHA256_FILE} | awk '{print $1}')"

# Display file info
DMG_SIZE=$(du -h "${DMG_PATH}" | cut -f1)
echo ""
echo "✅ DMG created successfully!"
echo "   Name: ${DMG_NAME}"
echo "   Size: ${DMG_SIZE}"
echo "   Path: ${DMG_PATH}"
echo "   Checksum: ${SHA256_FILE}"
echo ""
