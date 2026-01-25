# Homebrew ChaseAI Tap

This is a Homebrew tap for installing ChaseAI on macOS.

## Installation

```bash
# Add the tap
brew tap chaseai/chaseai

# Install ChaseAI
brew install chaseai

# Verify installation
chaseai --version
```

## Usage

After installation, you can run ChaseAI from the command line:

```bash
chaseai [OPTIONS] [COMMAND]
```

For more information, see the [ChaseAI documentation](https://github.com/chaseai/chaseai).

## Updating

To update ChaseAI to the latest version:

```bash
brew upgrade chaseai
```

## Uninstalling

To uninstall ChaseAI:

```bash
brew uninstall chaseai
```

## Troubleshooting

### "chaseai" cannot be opened because the developer cannot be verified

This error occurs on macOS when the application is not notarized. To resolve:

1. Open System Preferences → Security & Privacy
2. Click "Open Anyway" next to the ChaseAI error
3. Try running `chaseai` again

### Formula not found

If you get "Error: No available formula with the name 'chaseai'", make sure you've added the tap:

```bash
brew tap chaseai/chaseai
```

## Contributing

To contribute to this tap, please submit a pull request to the [homebrew-chaseai repository](https://github.com/chaseai/homebrew-chaseai).

## License

ChaseAI is licensed under the MIT License. See the [LICENSE](https://github.com/chaseai/chaseai/blob/main/LICENSE) file for details.
