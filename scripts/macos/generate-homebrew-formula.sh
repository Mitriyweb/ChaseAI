#!/bin/bash

# Generate Homebrew formula for ChaseAI
# This script creates a Homebrew formula that can be used to install ChaseAI
# via `brew install chaseai/chaseai/chaseai`

set -e

echo "📝 Generating Homebrew formula..."

# Configuration
BINARY_NAME="chase-ai"
VERSION="${1:-0.1.0}"
RELEASE_DIR="target/release"
DMG_NAME="${BINARY_NAME}-${VERSION}-macos.dmg"
DMG_PATH="${RELEASE_DIR}/${DMG_NAME}"
FORMULA_DIR="homebrew-chaseai/Formula"
FORMULA_FILE="${FORMULA_DIR}/chaseai.rb"

# Verify DMG exists
if [ ! -f "${DMG_PATH}" ]; then
  echo "❌ Error: DMG not found at ${DMG_PATH}"
  echo "   Please run create-dmg.sh first"
  exit 1
fi

# Calculate SHA256 checksum
echo "🔐 Calculating SHA256 checksum..."
SHA256=$(shasum -a 256 "${DMG_PATH}" | awk '{print $1}')

# Create formula directory if it doesn't exist
mkdir -p "${FORMULA_DIR}"

# Generate formula
cat > "${FORMULA_FILE}" << EOF
# Generated formula for ChaseAI
# This formula installs ChaseAI from a pre-built DMG

class Chaseai < Formula
  desc "Local control and orchestration system for AI agents"
  homepage "https://github.com/chaseai/chaseai"
  url "https://github.com/chaseai/chaseai/releases/download/v#{version}/chase-ai-#{version}-macos.dmg"
  sha256 "${SHA256}"
  version "${VERSION}"

  # Supported architectures
  on_macos do
    if Hardware::CPU.arm?
      # Apple Silicon
    elsif Hardware::CPU.intel?
      # Intel
    end
  end

  def install
    # Mount DMG and extract app
    dmg_mount_point = mount_dmg(cached_download)

    # Copy app bundle to Applications
    app_bundle = File.join(dmg_mount_point, "ChaseAI.app")
    if File.exist?(app_bundle)
      cp_r app_bundle, "/Applications/"
    end

    # Create symlink to binary in bin directory
    bin.install_symlink "/Applications/ChaseAI.app/Contents/MacOS/ChaseAI" => "chaseai"
  end

  def mount_dmg(dmg_path)
    mount_point = "/Volumes/ChaseAI"
    system "hdiutil", "attach", dmg_path, "-mountpoint", mount_point
    mount_point
  end

  def post_install
    puts "ChaseAI has been installed!"
    puts "You can now run: chaseai --help"
  end

  test do
    system "#{bin}/chaseai", "--version"
  end
end
EOF

echo "✅ Formula generated successfully!"
echo "   File: ${FORMULA_FILE}"
echo "   Version: ${VERSION}"
echo "   SHA256: ${SHA256}"
echo ""
echo "To use this formula:"
echo "  1. Commit to homebrew-chaseai repository"
echo "  2. Users can install with: brew tap chaseai/chaseai"
echo "  3. Then: brew install chaseai"
echo ""
