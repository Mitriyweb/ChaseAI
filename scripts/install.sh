#!/bin/bash

# ChaseAI Installer
# Installs the latest version of ChaseAI from GitHub Releases

set -e

REPO="Mitriyweb/ChaseAI"
APP_NAME="ChaseAI.app"
INSTALL_DIR="/Applications"

echo "🔍 Checking system compatibility..."
if [[ "$OSTYPE" != "darwin"* ]]; then
    echo "❌ Error: ChaseAI is currently only available for macOS."
    exit 1
fi

echo "⬇️  Fetching latest release info..."
LATEST_RELEASE=$(curl -s "https://api.github.com/repos/$REPO/releases/latest")
TAG_NAME=$(echo "$LATEST_RELEASE" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

if [ -z "$TAG_NAME" ]; then
    echo "❌ Error: Could not find latest release tag."
    exit 1
fi

echo "🏷️  Latest version: $TAG_NAME"

# Find the DMG asset
DMG_URL=$(echo "$LATEST_RELEASE" | grep '"browser_download_url":' | grep '.dmg"' | sed -E 's/.*"([^"]+)".*/\1/')

if [ -z "$DMG_URL" ]; then
    echo "❌ Error: Could not find DMG asset in release."
    exit 1
fi

# Create temp directory
TEMP_DIR=$(mktemp -d)
DMG_FILE="$TEMP_DIR/ChaseAI.dmg"

echo "📥 Downloading ChaseAI..."
curl -L -o "$DMG_FILE" "$DMG_URL" --progress-bar

echo "📦 Mounting DMG..."
MOUNT_POINT=$(hdiutil attach "$DMG_FILE" -nobrowse | grep "/Volumes" | awk '{print $3}')

if [ -z "$MOUNT_POINT" ]; then
    # Sometimes awk fails if volume name has spaces, try fallback
    MOUNT_POINT="/Volumes/ChaseAI"
fi

echo "🚀 Installing to $INSTALL_DIR..."

if [ -d "$INSTALL_DIR/ChaseAI.app" ]; then
    echo "   Removing existing installation..."
    rm -rf "$INSTALL_DIR/ChaseAI.app"
fi

cp -r "$MOUNT_POINT/$APP_NAME" "$INSTALL_DIR/"

echo "🧹 Cleaning up..."
hdiutil detach "$MOUNT_POINT" -quiet
rm -rf "$TEMP_DIR"

echo ""
echo "✅ ChaseAI installed successfully!"
echo "   Run it from Applications or type: open -a ChaseAI"
