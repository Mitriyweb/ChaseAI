#!/bin/bash

# Build the release binary
echo "Building release binary..."
cargo build --release

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

# Copy icon if exists
if [ -f "resources/icon.png" ]; then
    echo "Copying icons..."
    cp "resources/icon.png" "${RESOURCES_DIR}/"

    # Create menu bar icon if it doesn't exist
    if [ ! -f "resources/icon_menubar.png" ]; then
        echo "Creating menu bar icon (22x22)..."
        sips -z 22 22 "resources/icon.png" --out "resources/icon_menubar.png" > /dev/null 2>&1
    fi

    if [ -f "resources/icon_menubar.png" ]; then
        cp "resources/icon_menubar.png" "${RESOURCES_DIR}/"
    fi
fi

# Remove quarantine attributes
echo "Removing quarantine attributes..."
xattr -cr "${APP_DIR}"

echo ""
echo "✅ App bundle created at: ${APP_DIR}"
echo ""
echo "To run the app:"
echo "  open ${APP_DIR}"
echo ""
echo "Or double-click the app in Finder at:"
echo "  $(pwd)/${APP_DIR}"
