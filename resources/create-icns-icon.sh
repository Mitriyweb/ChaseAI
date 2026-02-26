#!/bin/bash

# Create macOS .icns icon from PNG source
# The .icns format supports multiple resolutions and is required for macOS app bundles

set -e

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
SOURCE_ICON="${SCRIPT_DIR}/icon.png"
OUTPUT_ICNS="${SCRIPT_DIR}/icon.icns"
TEMP_DIR=$(mktemp -d)
ICONSET_DIR="${TEMP_DIR}/icon.iconset"

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

# Verify iconutil is available
if ! command -v iconutil &> /dev/null; then
    echo "✗ Error: iconutil not found. This is required on macOS."
    exit 1
fi

echo "Creating .icns icon from $SOURCE_ICON..."

# Create iconset directory
mkdir -p "$ICONSET_DIR"

# Create Python script to generate all required sizes
cat > "$TEMP_DIR/generate_sizes.py" << 'PYTHON_EOF'
#!/usr/bin/env python3
"""Generate all required icon sizes for .icns format."""
import sys
from PIL import Image

def generate_icon_sizes(source_path, output_dir):
    """Generate all required icon sizes."""
    # Open source image
    img = Image.open(source_path).convert('RGBA')

    # Define required sizes for .icns format
    # Format: (size, scale, filename)
    sizes = [
        (16, 1, 'icon_16x16.png'),
        (16, 2, 'icon_16x16@2x.png'),
        (32, 1, 'icon_32x32.png'),
        (32, 2, 'icon_32x32@2x.png'),
        (64, 1, 'icon_64x64.png'),
        (64, 2, 'icon_64x64@2x.png'),
        (128, 1, 'icon_128x128.png'),
        (128, 2, 'icon_128x128@2x.png'),
        (256, 1, 'icon_256x256.png'),
        (256, 2, 'icon_256x256@2x.png'),
        (512, 1, 'icon_512x512.png'),
        (512, 2, 'icon_512x512@2x.png'),
    ]

    for size, scale, filename in sizes:
        actual_size = size * scale
        resized = img.resize((actual_size, actual_size), Image.Resampling.LANCZOS)
        output_path = f"{output_dir}/{filename}"
        resized.save(output_path, 'PNG')
        print(f"✓ Generated {filename} ({actual_size}x{actual_size})")

    return True

if __name__ == '__main__':
    if len(sys.argv) != 3:
        print("Usage: generate_sizes.py <source> <output_dir>")
        sys.exit(1)

    source = sys.argv[1]
    output_dir = sys.argv[2]

    try:
        generate_icon_sizes(source, output_dir)
        sys.exit(0)
    except Exception as e:
        print(f"✗ Error: {e}", file=sys.stderr)
        sys.exit(1)
PYTHON_EOF

# Generate all icon sizes
if command -v python3 &> /dev/null; then
    echo "Generating icon sizes..."
    python3 "$TEMP_DIR/generate_sizes.py" "$SOURCE_ICON" "$ICONSET_DIR"
else
    echo "✗ Error: python3 not found. Required for icon generation."
    exit 1
fi

# Verify all sizes were created
expected_files=(
    "icon_16x16.png"
    "icon_16x16@2x.png"
    "icon_32x32.png"
    "icon_32x32@2x.png"
    "icon_64x64.png"
    "icon_64x64@2x.png"
    "icon_128x128.png"
    "icon_128x128@2x.png"
    "icon_256x256.png"
    "icon_256x256@2x.png"
    "icon_512x512.png"
    "icon_512x512@2x.png"
)

for file in "${expected_files[@]}"; do
    if [ ! -f "$ICONSET_DIR/$file" ]; then
        echo "✗ Error: Failed to generate $file"
        exit 1
    fi
done

echo "All icon sizes generated successfully"

# Convert iconset to .icns using iconutil
echo "Converting iconset to .icns format..."
iconutil -c icns "$ICONSET_DIR" -o "$OUTPUT_ICNS"

# Verify .icns was created
if [ ! -f "$OUTPUT_ICNS" ]; then
    echo "✗ Error: Failed to create .icns file"
    exit 1
fi

# Get file size
ICNS_SIZE=$(du -h "$OUTPUT_ICNS" | cut -f1)
echo "✓ Successfully created $OUTPUT_ICNS ($ICNS_SIZE)"
echo ""
echo "Icon file is ready for inclusion in the macOS app bundle."
