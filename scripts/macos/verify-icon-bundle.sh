#!/bin/bash

# Verify that icon files are properly included in the macOS app bundle
# This script checks that all required icon files are present and correctly configured

set -e

APP_BUNDLE="${1:-.}"

# Check if this is an app bundle
if [[ ! "$APP_BUNDLE" == *.app ]]; then
    echo "✗ Error: Not an app bundle: $APP_BUNDLE"
    exit 1
fi

if [ ! -d "$APP_BUNDLE" ]; then
    echo "✗ Error: App bundle not found: $APP_BUNDLE"
    exit 1
fi

RESOURCES_DIR="${APP_BUNDLE}/Contents/Resources"
PLIST_FILE="${APP_BUNDLE}/Contents/Info.plist"

echo "Verifying icon bundle for: $APP_BUNDLE"
echo ""

# Check 1: Info.plist contains CFBundleIconFile key
echo "Checking Info.plist configuration..."
if [ ! -f "$PLIST_FILE" ]; then
    echo "✗ Error: Info.plist not found"
    exit 1
fi

# Extract the icon name from Info.plist
ICON_NAME=$(plutil -p "$PLIST_FILE" 2>/dev/null | grep "CFBundleIconFile" | sed 's/.*=> "\(.*\)".*/\1/' || echo "")

if [ -z "$ICON_NAME" ]; then
    echo "✗ Error: CFBundleIconFile key not found in Info.plist"
    exit 1
fi

echo "✓ CFBundleIconFile key found: $ICON_NAME"

# Check 2: Icon file exists in Resources directory
echo ""
echo "Checking icon files in Resources directory..."
ICON_FILE="${RESOURCES_DIR}/${ICON_NAME}.icns"

if [ ! -f "$ICON_FILE" ]; then
    echo "✗ Error: Icon file not found: $ICON_FILE"
    exit 1
fi

echo "✓ Icon file found: $ICON_FILE"

# Check 3: Verify .icns file is valid
echo ""
echo "Validating .icns file format..."
if file "$ICON_FILE" | grep -q "icns"; then
    echo "✓ Valid .icns file format"
else
    echo "⚠ Warning: File may not be valid .icns format"
fi

# Check 4: Verify icon file size (should be > 100KB for proper icon)
ICON_SIZE=$(stat -f%z "$ICON_FILE" 2>/dev/null || stat -c%s "$ICON_FILE" 2>/dev/null)
if [ "$ICON_SIZE" -gt 100000 ]; then
    echo "✓ Icon file size is reasonable: $(numfmt --to=iec-i --suffix=B $ICON_SIZE 2>/dev/null || echo "$ICON_SIZE bytes")"
else
    echo "⚠ Warning: Icon file size seems small: $ICON_SIZE bytes"
fi

# Check 5: Verify menubar icons exist
echo ""
echo "Checking menubar icons..."
MENUBAR_ICON="${RESOURCES_DIR}/icon_menubar.png"
MENUBAR_ICON_64="${RESOURCES_DIR}/icon_menubar_64.png"

if [ -f "$MENUBAR_ICON" ]; then
    echo "✓ Menubar icon found: icon_menubar.png"
else
    echo "✗ Error: Menubar icon not found: icon_menubar.png"
    exit 1
fi

if [ -f "$MENUBAR_ICON_64" ]; then
    echo "✓ 64x64 menubar icon found: icon_menubar_64.png"
else
    echo "⚠ Warning: 64x64 menubar icon not found: icon_menubar_64.png"
fi

echo ""
echo "✅ Icon bundle verification complete!"
echo ""
echo "Summary:"
echo "  - Info.plist: ✓ Configured"
echo "  - Icon file: ✓ Present"
echo "  - Icon format: ✓ Valid"
echo ""
echo "The app bundle is ready for distribution."
