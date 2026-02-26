# Icon Generation Guide

This document explains how to generate and manage icons for the ChaseAI macOS application.

## Overview

The ChaseAI application uses a single icon format for the dock display:

- **Dock Icon**: `.icns` format with multiple resolutions (16x16 to 1024x1024)
- **Source Icon**: High-contrast PNG (512x512) used for generation

## Icon Files

All icon files are located in the `resources/` directory:

```text
resources/
├── icon.png                    # Source icon (512x512, high contrast)
├── icon.icns                   # macOS application icon (generated)
└── create-icns-icon.sh         # Script to generate .icns
```

## Generating Icons

### Prerequisites

- macOS (for `iconutil` command)
- Python 3 (for image processing)
- Bash shell

### Generate .icns Icon

The `.icns` format is required for the macOS dock and Finder display.

```bash
bash resources/create-icns-icon.sh
```

This script:

1. Takes `resources/icon.png` as input
2. Generates all required resolutions (16x16, 32x32, 64x64, 128x128, 256x256, 512x512, 1024x1024)
3. Creates both standard (1x) and Retina (2x) variants
4. Outputs `resources/icon.icns`

**Output**: `resources/icon.icns` (~192 KB)

## Source Icon Requirements

The source icon (`resources/icon.png`) must meet these requirements:

- **Minimum size**: 512x512 pixels
- **Format**: PNG with transparency support
- **Color space**: RGB or RGBA
- **Design**: High contrast, recognizable at small sizes (16x16)
- **Style**: Should work well in both color (dock) and monochrome (menubar) contexts

### Design Guidelines

1. **Simplicity**: Keep the design simple enough to remain recognizable at 16x16 pixels
2. **Contrast**: Use high contrast colors to ensure visibility when converted to monochrome
3. **Symmetry**: Symmetric designs tend to scale better across resolutions
4. **Distinctive**: Make the icon distinctive and identifiable as ChaseAI
5. **Padding**: Include some padding around the main design element

## Build Integration

The icon generation is automatically integrated into the build process:

```bash
bash scripts/macos/build-macos-app.sh
```

The build script:

1. Checks if icon files exist
2. Generates missing icon files automatically
3. Copies the .icns icon file to the app bundle
4. Verifies the icon bundle configuration

## Icon Verification

To verify that icons are properly configured in an app bundle:

```bash
bash scripts/macos/verify-icon-bundle.sh target/release/ChaseAI.app
```

This script checks:

- Info.plist contains `CFBundleIconFile` key
- Icon file exists in the bundle Resources directory
- Icon file is in valid `.icns` format

## Info.plist Configuration

The `Info.plist` file must contain the `CFBundleIconFile` key:

```xml
<key>CFBundleIconFile</key>
<string>icon</string>
```

This tells macOS to use `icon.icns` from the Resources directory.

## Troubleshooting

### Icon not appearing in dock

1. Verify `CFBundleIconFile` key is in `Info.plist`
2. Check that `icon.icns` exists in the app bundle Resources directory
3. Clear the macOS icon cache:

   ```bash
   rm -rf ~/Library/Caches/com.apple.iconservices.store
   killall Finder
   ```

4. Rebuild the app bundle

## Updating Icons

To update the application icon:

1. Replace `resources/icon.png` with the new source icon
2. Run the generation scripts:

   ```bash
   bash resources/create-icns-icon.sh
   bash resources/create-menubar-icon.sh
   ```

3. Rebuild the application:

   ```bash
   bash scripts/macos/build-macos-app.sh
   ```

4. Test the new icon in the app bundle

## Icon Specifications

### .icns Format

- **Resolutions**: 16x16, 32x32, 64x64, 128x128, 256x256, 512x512, 1024x1024
- **Variants**: Standard (1x) and Retina (2x) for each resolution
- **Color depth**: 32-bit RGBA
- **File size**: Typically 150-250 KB

## References

- [Apple macOS App Icon Guidelines](https://developer.apple.com/design/human-interface-guidelines/app-icons)
- [macOS Bundle Structure](https://developer.apple.com/library/archive/documentation/CoreFoundation/Conceptual/CFBundles/BundleTypes/BundleTypes.html)
- [Info.plist Key Reference](https://developer.apple.com/library/archive/documentation/General/Reference/InfoPlistKeyReference/Articles/CoreFoundationKeys.html)
- [iconutil Command Reference](https://ss64.com/osx/iconutil.html)
