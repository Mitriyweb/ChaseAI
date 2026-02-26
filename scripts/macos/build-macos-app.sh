#!/bin/bash

# Build the macOS app bundle
# This script packages the ChaseAI binary into a standard macOS .app bundle.

set -e

# Navigate to the project root
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cd "$SCRIPT_DIR/../.."

# Determine environment and features
ENV=${1:-prod}
shift || true
EXTRA_ARGS="$@"
FEATURES=""

if [ "$ENV" == "beta" ]; then
    FEATURES="--features beta"
    echo "Building for BETA environment..."
elif [ "$ENV" == "dev" ]; then
    FEATURES="--features dev"
    echo "Building for DEV environment..."
else
    echo "Building for PROD environment..."
fi

# Build the release binary
echo "Building release binary..."
cargo build --release $FEATURES $EXTRA_ARGS

# Skip macOS-specific app bundle creation on non-macOS platforms
if [[ "$OSTYPE" != "darwin"* ]]; then
    echo "Skipping macOS app bundle creation on non-macOS platform ($OSTYPE)"
    exit 0
fi

# Create app bundle structure
APP_NAME="ChaseAI"
APP_DIR="target/release/${APP_NAME}.app"
CONTENTS_DIR="${APP_DIR}/Contents"
MACOS_DIR="${CONTENTS_DIR}/MacOS"
RESOURCES_DIR="${CONTENTS_DIR}/Resources"

echo "Creating app bundle structure..."
rm -rf "${APP_DIR}"
mkdir -p "${MACOS_DIR}"
mkdir -p "${RESOURCES_DIR}"

# Copy binary
echo "Copying binary..."
cp "target/release/chase-ai" "${MACOS_DIR}/${APP_NAME}"

# Copy Info.plist
echo "Copying Info.plist..."
cp "Info.plist" "${CONTENTS_DIR}/"

# Copy and verify icon files
echo "Copying icon files..."

# Copy .icns icon (required for dock display)
if [ -f "resources/icon.icns" ]; then
    cp "resources/icon.icns" "${RESOURCES_DIR}/"
    echo "  ✓ Copied icon.icns"
else
    echo "  ⚠ Warning: icon.icns not found. Generating..."
    bash resources/create-icns-icon.sh
    if [ -f "resources/icon.icns" ]; then
        cp "resources/icon.icns" "${RESOURCES_DIR}/"
        echo "  ✓ Generated and copied icon.icns"
    else
        echo "  ✗ Error: Failed to generate icon.icns"
        exit 1
    fi
fi

# Copy source icon for reference
if [ -f "resources/icon.png" ]; then
    cp "resources/icon.png" "${RESOURCES_DIR}/"
    echo "  ✓ Copied icon.png"
fi

# Remove quarantine attributes
echo "Removing quarantine attributes..."
xattr -cr "${APP_DIR}"

# Verify icon bundle
echo ""
echo "Verifying icon bundle..."
bash "scripts/macos/verify-icon-bundle.sh" "${APP_DIR}"

echo ""
echo "✅ App bundle created at: ${APP_DIR}"
echo ""
echo "To run the app:"
echo "  open ${APP_DIR}"
echo ""
echo "Or double-click the app in Finder at:"
echo "  $(pwd)/${APP_DIR}"
