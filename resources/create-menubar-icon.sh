#!/bin/bash

# Create a simple monochrome menu bar icon
# macOS menu bar icons should be:
# - 22x22 pixels (or 16x16)
# - Monochrome (black on transparent)
# - Simple design

# Convert the existing icon to grayscale and resize
sips -z 22 22 resources/icon.png --out resources/icon_menubar_temp.png > /dev/null 2>&1

# Convert to grayscale using ImageMagick if available
if command -v convert &> /dev/null; then
    echo "Using ImageMagick to create monochrome icon..."
    convert resources/icon_menubar_temp.png \
        -colorspace Gray \
        -threshold 50% \
        resources/icon_menubar.png
    rm resources/icon_menubar_temp.png
else
    echo "ImageMagick not found, using sips for basic conversion..."
    # Use sips to convert to grayscale
    sips -s format png -s formatOptions best resources/icon_menubar_temp.png --out resources/icon_menubar.png > /dev/null 2>&1
    rm resources/icon_menubar_temp.png
fi

echo "Menu bar icon created: resources/icon_menubar.png"
