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

# Try to download tar.gz archive
ARCHIVE_URL="https://github.com/$REPO/releases/download/v$LATEST_RELEASE/chase-ai-$LATEST_RELEASE-macos.tar.gz"
ARCHIVE_FILE="/tmp/chase-ai-$LATEST_RELEASE.tar.gz"

echo "📦 Downloading ChaseAI $LATEST_RELEASE..."
if ! curl -L -o "$ARCHIVE_FILE" "$ARCHIVE_URL"; then
    echo -e "${RED}❌ Error: Failed to download ChaseAI${NC}"
    exit 1
fi

# Verify file is not empty
if [ ! -s "$ARCHIVE_FILE" ]; then
    echo -e "${RED}❌ Error: Downloaded file is empty${NC}"
    rm -f "$ARCHIVE_FILE"
    exit 1
fi

# Verify checksum if available
CHECKSUMS_URL="https://github.com/$REPO/releases/download/v$LATEST_RELEASE/checksums.sha256"
CHECKSUMS_FILE="/tmp/checksums.sha256"
if curl -s -f "$CHECKSUMS_URL" > "$CHECKSUMS_FILE" 2>/dev/null && [ -s "$CHECKSUMS_FILE" ]; then
    echo "🔐 Verifying checksum..."

    # Extract the expected checksum for our archive file
    EXPECTED_CHECKSUM=$(grep "chase-ai-$LATEST_RELEASE-macos.tar.gz" "$CHECKSUMS_FILE" | awk '{print $1}')

    if [ -z "$EXPECTED_CHECKSUM" ]; then
        echo -e "${YELLOW}⚠ Warning: Could not find checksum for archive file${NC}"
    else
        # Calculate actual checksum
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
    echo -e "${YELLOW}⚠ Warning: Could not download checksums file, skipping verification${NC}"
fi

# Extract archive
echo "📂 Extracting archive..."
TEMP_DIR=$(mktemp -d)
tar -xzf "$ARCHIVE_FILE" -C "$TEMP_DIR"

# Copy app to Applications
echo "📋 Installing ChaseAI to $INSTALL_DIR..."
if [ -d "$INSTALL_DIR/$APP_NAME" ]; then
    echo "   Removing existing installation..."
    rm -rf "$INSTALL_DIR/$APP_NAME"
fi

cp -r "$TEMP_DIR/$APP_NAME" "$INSTALL_DIR/"

# Clean up
rm -f "$ARCHIVE_FILE"
rm -rf "$TEMP_DIR"

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
