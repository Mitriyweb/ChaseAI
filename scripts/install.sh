#!/bin/bash

# ChaseAI Installation Script
# Downloads and installs ChaseAI on macOS

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
REPO="Mitriyweb/ChaseAI"
INSTALL_DIR="/Applications"
APP_NAME="ChaseAI.app"

echo -e "${GREEN}🚀 ChaseAI Installation Script${NC}"
echo ""

# Check if running on macOS
if [[ "$OSTYPE" != "darwin"* ]]; then
    echo -e "${RED}❌ Error: This script only works on macOS${NC}"
    exit 1
fi

# Get the latest release version
echo "📥 Fetching latest release..."
LATEST_RELEASE=$(curl -s https://api.github.com/repos/$REPO/releases/latest | grep '"tag_name"' | sed -E 's/.*"v([^"]+)".*/\1/')

if [ -z "$LATEST_RELEASE" ]; then
    echo -e "${RED}❌ Error: Could not fetch latest release${NC}"
    exit 1
fi

echo "   Latest version: $LATEST_RELEASE"

# Download DMG
DMG_URL="https://github.com/$REPO/releases/download/v$LATEST_RELEASE/chaseai-$LATEST_RELEASE-macos.dmg"
DMG_FILE="/tmp/chaseai-$LATEST_RELEASE.dmg"

echo "📦 Downloading ChaseAI $LATEST_RELEASE..."
if ! curl -L -o "$DMG_FILE" "$DMG_URL"; then
    echo -e "${RED}❌ Error: Failed to download DMG${NC}"
    exit 1
fi

# Verify checksum if available
CHECKSUMS_URL="https://github.com/$REPO/releases/download/v$LATEST_RELEASE/checksums.sha256"
if curl -s -f "$CHECKSUMS_URL" > /tmp/checksums.sha256; then
    echo "🔐 Verifying checksum..."
    cd /tmp
    if ! shasum -a 256 -c checksums.sha256 | grep -q "chaseai-$LATEST_RELEASE-macos.dmg"; then
        echo -e "${RED}❌ Error: Checksum verification failed${NC}"
        rm -f "$DMG_FILE"
        exit 1
    fi
    echo -e "${GREEN}✓ Checksum verified${NC}"
fi

# Mount DMG
echo "📂 Mounting DMG..."
MOUNT_POINT=$(mktemp -d)
hdiutil attach "$DMG_FILE" -mountpoint "$MOUNT_POINT" -nobrowse

# Copy app to Applications
echo "📋 Installing ChaseAI to $INSTALL_DIR..."
if [ -d "$INSTALL_DIR/$APP_NAME" ]; then
    echo "   Removing existing installation..."
    rm -rf "$INSTALL_DIR/$APP_NAME"
fi

cp -r "$MOUNT_POINT/$APP_NAME" "$INSTALL_DIR/"

# Unmount DMG
echo "🔓 Unmounting DMG..."
hdiutil detach "$MOUNT_POINT"

# Clean up
rm -f "$DMG_FILE"
rm -rf "$MOUNT_POINT"

# Verify installation
if [ -d "$INSTALL_DIR/$APP_NAME" ]; then
    echo -e "${GREEN}✅ Installation successful!${NC}"
    echo ""
    echo "📍 ChaseAI installed to: $INSTALL_DIR/$APP_NAME"
    echo ""
    echo "🚀 To launch ChaseAI:"
    echo "   open $INSTALL_DIR/$APP_NAME"
    echo ""
    echo "💡 Or use Spotlight search (Cmd+Space) and type 'ChaseAI'"
else
    echo -e "${RED}❌ Error: Installation failed${NC}"
    exit 1
fi
