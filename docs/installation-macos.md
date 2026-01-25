# macOS Installation Guide

This guide covers installing ChaseAI on macOS using either the DMG installer or Homebrew package manager.

## System Requirements

- **macOS Version**: 10.13 (High Sierra) or later
- **Architecture**: Intel (x86_64) or Apple Silicon (aarch64)
- **Disk Space**: ~100 MB for application and dependencies
- **RAM**: 2 GB minimum (4 GB recommended)

## Installation Methods

### Method 1: DMG Installer (Recommended for First-Time Users)

The DMG installer provides a simple, graphical installation experience.

#### Installation Steps

1. **Download the DMG**
   - Visit [ChaseAI Releases](https://github.com/chaseai/chaseai/releases)
   - Download the latest `chase-ai-*.dmg` file for your architecture
   - Verify the checksum (optional but recommended):

   ```bash
   shasum -a 256 chase-ai-*.dmg
   # Compare with the .sha256 file from the release
   ```

2. **Mount the DMG**
   - Double-click the downloaded DMG file
   - The DMG will mount and open a Finder window

3. **Install the Application**
   - Drag the `ChaseAI.app` icon to the `Applications` folder
   - Wait for the copy operation to complete

4. **Eject the DMG**
   - Close the Finder window
   - Eject the DMG by dragging it to the Trash or using `hdiutil eject`

5. **Launch ChaseAI**
   - Open Applications folder
   - Double-click `ChaseAI.app`
   - If you see a security warning, click "Open" to proceed

#### Verification

To verify the installation:

```bash
# Check if the app is installed
ls -la /Applications/ChaseAI.app

# Run the application
/Applications/ChaseAI.app/Contents/MacOS/ChaseAI --version
```

### Method 2: Homebrew (Recommended for Developers)

Homebrew provides easy installation and updates via the command line.

#### Prerequisites

- [Homebrew](https://brew.sh) installed on your system
- Command Line Tools for Xcode (installed automatically by Homebrew if needed)

#### Homebrew Installation Steps

1. **Add the ChaseAI Tap**

   ```bash
   brew tap chaseai/chaseai
   ```

2. **Install ChaseAI**

   ```bash
   brew install chaseai
   ```

3. **Verify Installation**

   ```bash
   chaseai --version
   ```

#### Updating via Homebrew

To update ChaseAI to the latest version:

```bash
brew upgrade chaseai
```

#### Uninstalling via Homebrew

To remove ChaseAI:

```bash
brew uninstall chaseai
```

## Troubleshooting

### "ChaseAI" cannot be opened because the developer cannot be verified

This error occurs on macOS when the application is not notarized or the notarization ticket is missing.

**Solution:**

1. Open **System Preferences** → **Security & Privacy**
2. Look for the ChaseAI error message
3. Click **"Open Anyway"** next to the error
4. Try running ChaseAI again

Alternatively, you can allow the app via command line:

```bash
xattr -d com.apple.quarantine /Applications/ChaseAI.app
```

### "brew: command not found"

Homebrew is not installed on your system.

**Solution:**

Install Homebrew by running:

```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

Then follow the Homebrew installation steps above.

### "Error: No available formula with the name 'chaseai'"

The ChaseAI tap has not been added to your Homebrew installation.

**Solution:**

Add the tap and try again:

```bash
brew tap chaseai/chaseai
brew install chaseai
```

### Application crashes on startup

This may indicate a missing dependency or incompatible system configuration.

**Solution:**

1. Check the system requirements above
2. Try reinstalling:

   ```bash
   # For DMG installation
   rm -rf /Applications/ChaseAI.app
   # Then reinstall using the DMG

   # For Homebrew installation
   brew uninstall chaseai
   brew install chaseai
   ```

3. Check the application logs:

   ```bash
   log stream --predicate 'process == "ChaseAI"' --level debug
   ```

### Homebrew formula not updating

If you have an older version and `brew upgrade` doesn't work:

**Solution:**

```bash
# Remove and reinstall
brew uninstall chaseai
brew tap --repair chaseai/chaseai
brew install chaseai
```

## Uninstallation

### Uninstall via DMG

1. Open the Applications folder
2. Drag `ChaseAI.app` to the Trash
3. Empty the Trash

### Uninstall via Homebrew

```bash
brew uninstall chaseai
```

## Advanced Usage

### Running from Command Line

After installation, you can run ChaseAI from the terminal:

```bash
# Using Homebrew installation
chaseai [OPTIONS] [COMMAND]

# Using DMG installation
/Applications/ChaseAI.app/Contents/MacOS/ChaseAI [OPTIONS] [COMMAND]
```

### Creating an Alias

For easier access, create an alias in your shell configuration file (`~/.zshrc`, `~/.bash_profile`, etc.):

```bash
alias chaseai="/Applications/ChaseAI.app/Contents/MacOS/ChaseAI"
```

Then reload your shell:

```bash
source ~/.zshrc  # or ~/.bash_profile for bash
```

### Checking Application Version

```bash
chaseai --version
```

### Getting Help

```bash
chaseai --help
```

## Support

For issues, questions, or feature requests, please visit:

- [GitHub Issues](https://github.com/chaseai/chaseai/issues)
- [GitHub Discussions](https://github.com/chaseai/chaseai/discussions)
- [Documentation](https://github.com/chaseai/chaseai/wiki)

## Related Documentation

- [Building from Source](./building-from-source.md)
- [Configuration Guide](./configuration.md)
- [User Guide](./user-guide.md)
