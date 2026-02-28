#!/bin/bash

# Create monochrome menu bar icons for macOS
# macOS menu bar icons should be:
# - 22x22 pixels (standard resolution)
# - 44x44 pixels (Retina/2x resolution)
# - Monochrome (black on transparent)
# - PNG format with proper transparency

set -e

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
SOURCE_ICON="${SCRIPT_DIR}/icon.png"
MENUBAR_ICON="${SCRIPT_DIR}/icon_menubar.png"
MENUBAR_ICON_2X="${SCRIPT_DIR}/icon_menubar@2x.png"
TEMP_DIR=$(mktemp -d)

# Cleanup on exit
cleanup() {
    rm -rf "$TEMP_DIR"
}
trap cleanup EXIT

# Verify source icon exists
if [ ! -f "$SOURCE_ICON" ]; then
    echo "✗ Error: Source icon not found at $SOURCE_ICON"
    exit 1
fi

echo "Creating menubar icons from $SOURCE_ICON..."

# Create Python script for monochrome conversion
cat > "$TEMP_DIR/convert_menubar.py" << 'PYTHON_EOF'
#!/usr/bin/env python3
"""Convert icon to monochrome menubar icon."""
import sys
from PIL import Image, ImageOps

def create_menubar_icon(source_path, output_path, size):
    """Create a monochrome menubar icon."""
    try:
        # Open the source image
        img = Image.open(source_path).convert('RGBA')

        # Resize to target size
        img = img.resize((size, size), Image.Resampling.LANCZOS)

        # Convert to grayscale
        gray = ImageOps.grayscale(img)

        # Apply threshold to create monochrome (black on transparent)
        # Use a threshold that preserves the icon's visibility
        threshold = 128
        mono = gray.point(lambda x: 0 if x < threshold else 255, '1')

        # Create RGBA image with transparency
        result = Image.new('RGBA', (size, size), (255, 255, 255, 0))

        # Paste the monochrome image, using it as both image and mask
        # This creates black pixels on transparent background
        for x in range(size):
            for y in range(size):
                if mono.getpixel((x, y)) == 0:  # Black pixel
                    result.putpixel((x, y), (0, 0, 0, 255))  # Black with full opacity

        # Save as PNG
        result.save(output_path, 'PNG')
        return True
    except Exception as e:
        print(f"✗ Error converting icon: {e}", file=sys.stderr)
        return False

if __name__ == '__main__':
    if len(sys.argv) != 4:
        print("Usage: convert_menubar.py <source> <output> <size>")
        sys.exit(1)

    source = sys.argv[1]
    output = sys.argv[2]
    size = int(sys.argv[3])

    if create_menubar_icon(source, output, size):
        print(f"✓ Created {output} ({size}x{size})")
        sys.exit(0)
    else:
        sys.exit(1)
PYTHON_EOF

# Try using Python for conversion
if command -v python3 &> /dev/null; then
    echo "Using Python to create monochrome icons..."
    python3 "$TEMP_DIR/convert_menubar.py" "$SOURCE_ICON" "$MENUBAR_ICON" 22
    python3 "$TEMP_DIR/convert_menubar.py" "$SOURCE_ICON" "$MENUBAR_ICON_2X" 44
    python3 "$TEMP_DIR/convert_menubar.py" "$SOURCE_ICON" "${SCRIPT_DIR}/icon_menubar_64.png" 64

    # Verify files were created
    if [ -f "$MENUBAR_ICON" ] && [ -f "$MENUBAR_ICON_2X" ] && [ -f "${SCRIPT_DIR}/icon_menubar_64.png" ]; then
        echo "✓ Menubar icons created successfully"
        echo "  - $MENUBAR_ICON (22x22)"
        echo "  - $MENUBAR_ICON_2X (44x44)"
        echo "  - ${SCRIPT_DIR}/icon_menubar_64.png (64x64)"
        exit 0
    fi
fi

# Fallback: Use sips for basic conversion
echo "Using sips for icon conversion (fallback)..."
sips -z 22 22 "$SOURCE_ICON" --out "$MENUBAR_ICON" > /dev/null 2>&1
sips -z 44 44 "$SOURCE_ICON" --out "$MENUBAR_ICON_2X" > /dev/null 2>&1
sips -z 64 64 "$SOURCE_ICON" --out "${SCRIPT_DIR}/icon_menubar_64.png" > /dev/null 2>&1

if [ -f "$MENUBAR_ICON" ] && [ -f "$MENUBAR_ICON_2X" ] && [ -f "${SCRIPT_DIR}/icon_menubar_64.png" ]; then
    echo "✓ Menubar icons created (using sips fallback)"
    echo "  - $MENUBAR_ICON (22x22)"
    echo "  - $MENUBAR_ICON_2X (44x44)"
    echo "  - ${SCRIPT_DIR}/icon_menubar_64.png (64x64)"
    exit 0
else
    echo "✗ Failed to create menubar icons"
    exit 1
fi
