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
echo "📥 Fetching latest release information..."
# Use a more robust way to get the latest tag
LATEST_RELEASE=$(curl -sL https://api.github.com/repos/$REPO/releases/latest | grep '"tag_name"' | sed -E 's/.*"v?([^"]+)".*/\1/')

if [ -z "$LATEST_RELEASE" ]; then
    echo -e "${RED}❌ Error: Could not fetch latest release version from GitHub API${NC}"
    exit 1
fi

echo "   Latest version: $LATEST_RELEASE"

# Try to download tar.gz archive (preferred for script-based install)
ARCHIVE_URL="https://github.com/$REPO/releases/download/v$LATEST_RELEASE/chase-ai-$LATEST_RELEASE-macos.tar.gz"
ARCHIVE_FILE="/tmp/chase-ai-$LATEST_RELEASE-macos.tar.gz"

echo "📦 Downloading ChaseAI $LATEST_RELEASE..."
DOWNLOAD_SUCCESS=false

# Use -f to fail on HTTP errors
if curl -sL -f -o "$ARCHIVE_FILE" "$ARCHIVE_URL"; then
    DOWNLOAD_SUCCESS=true
    IS_DMG=false
else
    echo -e "${YELLOW}⚠ Warning: Failed to download tar.gz, trying DMG fallback...${NC}"
    DMG_URL="https://github.com/$REPO/releases/download/v$LATEST_RELEASE/chase-ai-$LATEST_RELEASE-macos.dmg"
    ARCHIVE_FILE="/tmp/chase-ai-$LATEST_RELEASE-macos.dmg"
    if curl -sL -f -o "$ARCHIVE_FILE" "$DMG_URL"; then
        DOWNLOAD_SUCCESS=true
        IS_DMG=true
    else
        # Try without 'v' prefix in download URL if it failed
        ARCHIVE_URL_NO_V="https://github.com/$REPO/releases/download/$LATEST_RELEASE/chase-ai-$LATEST_RELEASE-macos.tar.gz"
        ARCHIVE_FILE="/tmp/chase-ai-$LATEST_RELEASE-macos.tar.gz"
        if curl -sL -f -o "$ARCHIVE_FILE" "$ARCHIVE_URL_NO_V"; then
             DOWNLOAD_SUCCESS=true
             IS_DMG=false
        fi
    fi
fi

if [ "$DOWNLOAD_SUCCESS" = false ]; then
    echo -e "${RED}❌ Error: Failed to download both tar.gz and DMG for version $LATEST_RELEASE${NC}"
    echo "   Check if the assets exist at: https://github.com/$REPO/releases/latest"
    exit 1
fi

# Verify file is not empty and not a 404 page
FILE_SIZE=$(stat -f%z "$ARCHIVE_FILE" 2>/dev/null || stat -c%s "$ARCHIVE_FILE" 2>/dev/null || echo "0")
if [ "$FILE_SIZE" -lt 10000 ]; then
    echo -e "${RED}❌ Error: Downloaded file is too small ($FILE_SIZE bytes). Download might be corrupted or asset is missing.${NC}"
    rm -f "$ARCHIVE_FILE"
    exit 1
fi

# Verify checksum if available
CHECKSUMS_URL="https://github.com/$REPO/releases/download/v$LATEST_RELEASE/checksums.sha256"
CHECKSUMS_FILE="/tmp/checksums.sha256"
if curl -sL -f -o "$CHECKSUMS_FILE" "$CHECKSUMS_URL" 2>/dev/null || curl -sL -f -o "$CHECKSUMS_FILE" "https://github.com/$REPO/releases/download/$LATEST_RELEASE/checksums.sha256" 2>/dev/null; then
    echo "🔐 Verifying checksum..."
    FILENAME=$(basename "$ARCHIVE_FILE")
    EXPECTED_CHECKSUM=$(grep "$FILENAME" "$CHECKSUMS_FILE" | awk '{print $1}')

    if [ -z "$EXPECTED_CHECKSUM" ]; then
        echo -e "${YELLOW}⚠ Warning: Could not find checksum for $FILENAME in checksums.sha256${NC}"
    else
        ACTUAL_CHECKSUM=$(shasum -a 256 "$ARCHIVE_FILE" | awk '{print $1}')
        if [ "$EXPECTED_CHECKSUM" = "$ACTUAL_CHECKSUM" ]; then
            echo -e "${GREEN}✓ Checksum verified${NC}"
        else
            echo -e "${RED}❌ Error: Checksum verification failed${NC}"
            echo "   Expected: $EXPECTED_CHECKSUM"
            echo "   Actual:   $ACTUAL_CHECKSUM"
            rm -f "$ARCHIVE_FILE"
            exit 1
        fi
    fi
else
    echo -e "${YELLOW}⚠ Warning: Could not download checksums.sha256, skipping verification${NC}"
fi

# Extract and install
echo "📂 Installing..."
MOUNT_POINT=$(mktemp -d)

if [ "$IS_DMG" = true ]; then
    echo "   Mounting DMG..."
    if ! hdiutil attach "$ARCHIVE_FILE" -mountpoint "$MOUNT_POINT" -nobrowse -quiet; then
        echo -e "${RED}❌ Error: Failed to mount DMG. The file might be corrupted.${NC}"
        rm -f "$ARCHIVE_FILE"
        exit 1
    fi
    SOURCE_PATH="$MOUNT_POINT/$APP_NAME"
else
    echo "   Extracting archive..."
    if ! tar -xzf "$ARCHIVE_FILE" -C "$MOUNT_POINT"; then
        echo -e "${RED}❌ Error: Failed to extract tar.gz archive. The file might be corrupted.${NC}"
        rm -f "$ARCHIVE_FILE"
        exit 1
    fi
    # Sometimes the app is inside a subfolder in the archive
    SOURCE_PATH=$(find "$MOUNT_POINT" -name "$APP_NAME" -maxdepth 2 | head -n 1)
fi

# Copy app to Applications
if [ -z "$SOURCE_PATH" ] || [ ! -d "$SOURCE_PATH" ]; then
    echo -e "${RED}❌ Error: Could not find $APP_NAME in the downloaded package${NC}"
    # Cleanup
    if [ "$IS_DMG" = true ]; then hdiutil detach "$MOUNT_POINT" -quiet || true; fi
    rm -f "$ARCHIVE_FILE"
    rm -rf "$MOUNT_POINT"
    exit 1
fi

echo "📋 Copying to $INSTALL_DIR..."
if [ -d "$INSTALL_DIR/$APP_NAME" ]; then
    echo "   Removing existing installation..."
    rm -rf "$INSTALL_DIR/$APP_NAME"
fi

cp -R "$SOURCE_PATH" "$INSTALL_DIR/"

# Cleanup
if [ "$IS_DMG" = true ]; then
    echo "   Unmounting DMG..."
    hdiutil detach "$MOUNT_POINT" -quiet
fi

rm -f "$ARCHIVE_FILE"
rm -rf "$MOUNT_POINT"

# Verify installation
if [ -d "$INSTALL_DIR/$APP_NAME" ]; then
    # Remove quarantine attributes to allow the app to run
    xattr -dr com.apple.quarantine "$INSTALL_DIR/$APP_NAME" 2>/dev/null || true

    echo -e "${GREEN}✅ Installation successful!${NC}"
    echo ""
    echo "📍 ChaseAI installed to: $INSTALL_DIR/$APP_NAME"
    echo ""
    echo "🚀 To launch ChaseAI:"
    echo "   open $INSTALL_DIR/$APP_NAME"
    echo ""
    echo "💡 Or use Spotlight search (Cmd+Space) and type 'ChaseAI'"
else
    echo -e "${RED}❌ Error: Installation failed - $APP_NAME not found in $INSTALL_DIR${NC}"
    exit 1
fi
