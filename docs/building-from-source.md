# Building ChaseAI from Source

This guide covers how to build ChaseAI from source code for various platforms.

## Prerequisites

- **Rust**: Version 1.93.0 or later (managed via `rust-toolchain.toml`)
- **Bun**: Version 1.0 or later
- **Git**: To clone the repository

### Platform-Specific Requirements

- **macOS**: Xcode Command Line Tools
- **Windows**: Visual Studio Build Tools
- **Linux**: GTK3 development libraries (`libgtk-3-dev`, `libayatana-appindicator3-dev`, `libxdo-dev`)

## Getting the Source

```bash
git clone https://github.com/Mitriyweb/ChaseAI.git
cd ChaseAI
```

## Building

### Standard Build

```bash
# Debug build
cargo build

# Release build
cargo build --release
```

### Beta Build (with extra features)

```bash
# Debug build with beta features
cargo build --features beta

# Release build with beta features
cargo build --release --features beta
```

## macOS Specific Builds

### App Bundle (.app)

```bash
# Production bundle
bun run build:app

# Beta bundle
bun run build:app:beta
```

### DMG Installer (.dmg)

```bash
# Production installer
bun run build:installer

# Beta installer
bun run build:installer:beta
```

### Universal Binary (x86_64 + aarch64)

```bash
bun run build:universal
```

## Troubleshooting

### Build fails on Linux

Ensure you have all system dependencies installed:

```bash
sudo apt-get install libgtk-3-dev libayatana-appindicator3-dev libxdo-dev
```

### Build fails on macOS

Ensure you have the latest Xcode Command Line Tools:

```bash
xcode-select --install
```
