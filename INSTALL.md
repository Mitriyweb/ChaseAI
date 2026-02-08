# ChaseAI Installation Guide

## Quick Install (macOS)

### One-liner Installation

```bash
curl -sL https://github.com/Mitriyweb/ChaseAI/releases/latest/download/install.sh | bash
```

This will:

1. Download the latest ChaseAI DMG
2. Verify the checksum
3. Mount the DMG
4. Copy ChaseAI.app to `/Applications`
5. Clean up temporary files

### Manual Installation

1. Download the latest DMG from [Releases](https://github.com/Mitriyweb/ChaseAI/releases)
2. Mount the DMG by double-clicking it
3. Drag `ChaseAI.app` to the `Applications` folder
4. Eject the DMG
5. Launch ChaseAI from Applications or Spotlight (Cmd+Space)

### Homebrew Installation

```bash
brew tap Mitriyweb/chaseai
brew install chaseai
```

## Verify Installation

After installation, verify ChaseAI is working:

```bash
# Check if app exists
ls -la /Applications/ChaseAI.app

# Launch ChaseAI
open /Applications/ChaseAI.app
```

## Troubleshooting

### "Cannot open ChaseAI.app" error

If you get a security warning, allow it in System Preferences:

1. Go to System Preferences → Security & Privacy
2. Click "Open Anyway" next to ChaseAI
3. Or run: `xattr -d com.apple.quarantine /Applications/ChaseAI.app`

### Installation script fails

Make sure you have:

- macOS 10.13 or later
- Internet connection
- Write access to `/Applications`

If the script fails, try manual installation instead.

### Checksum verification fails

This usually means the download was corrupted. Try again:

```bash
curl -sL https://github.com/Mitriyweb/ChaseAI/releases/latest/download/install.sh | bash
```

## Uninstall

To remove ChaseAI:

```bash
rm -rf /Applications/ChaseAI.app
```

Or if installed via Homebrew:

```bash
brew uninstall chaseai
```

## Support

For issues or questions, visit:

- [GitHub Issues](https://github.com/Mitriyweb/ChaseAI/issues)
- [Documentation](https://github.com/Mitriyweb/ChaseAI/docs)
