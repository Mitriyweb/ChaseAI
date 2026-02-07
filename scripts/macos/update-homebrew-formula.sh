#!/bin/bash

# Update Homebrew formula on GitHub release
# This script is designed to be run by GitHub Actions when a new release is created.
# It updates the Homebrew formula with the new version and SHA256 checksum.

set -e

echo "🔄 Updating Homebrew formula..."

# Configuration
BINARY_NAME="chase"
VERSION="${1:-}"
GITHUB_TOKEN="${2:-}"
FORMULA_REPO="chaseai/homebrew-chaseai"
FORMULA_FILE="Formula/chaseai.rb"

# Validate inputs
if [ -z "${VERSION}" ]; then
  echo "❌ Error: VERSION not provided"
  echo "Usage: $0 <version> [github-token]"
  exit 1
fi

# Remove 'v' prefix if present
VERSION="${VERSION#v}"

echo "📦 Version: ${VERSION}"

# Get release information from GitHub API
echo "📡 Fetching release information..."
RELEASE_URL="https://api.github.com/repos/chaseai/chaseai/releases/tags/v${VERSION}"
RELEASE_DATA=$(curl -s "${RELEASE_URL}")

# Extract download URL and calculate SHA256
DMG_NAME="${BINARY_NAME}-${VERSION}-macos.dmg"
DMG_URL="https://github.com/chaseai/chaseai/releases/download/v${VERSION}/${DMG_NAME}"

echo "📥 Downloading DMG to calculate checksum..."
TEMP_DMG=$(mktemp)
curl -L -o "${TEMP_DMG}" "${DMG_URL}"

SHA256=$(shasum -a 256 "${TEMP_DMG}" | awk '{print $1}')
rm -f "${TEMP_DMG}"

echo "   SHA256: ${SHA256}"

# Clone or update homebrew-chaseai repository
if [ ! -d "homebrew-chaseai" ]; then
  echo "📂 Cloning homebrew-chaseai repository..."
  git clone "https://github.com/${FORMULA_REPO}.git" homebrew-chaseai
fi

cd homebrew-chaseai

# Update formula
echo "✏️  Updating formula..."
cat > "${FORMULA_FILE}" << EOF
# Generated formula for ChaseAI
# This formula installs ChaseAI from a pre-built DMG

class Chaseai < Formula
  desc "Local control and orchestration system for AI agents"
  homepage "https://github.com/chaseai/chaseai"
  url "https://github.com/chaseai/chaseai/releases/download/v#{version}/chase-#{version}-macos.dmg"
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
    bin.install_symlink "/Applications/ChaseAI.app/Contents/MacOS/ChaseAI" => "chase"
  end

  def mount_dmg(dmg_path)
    mount_point = "/Volumes/ChaseAI"
    system "hdiutil", "attach", dmg_path, "-mountpoint", mount_point
    mount_point
  end

  def post_install
    puts "ChaseAI has been installed!"
    puts "You can now run: chase --help"
  end

  test do
    system "#{bin}/chase", "--version"
  end
end
EOF

# Commit and push changes
echo "📤 Committing and pushing changes..."
git config user.name "ChaseAI Bot"
git config user.email "bot@chaseai.dev"
git add "${FORMULA_FILE}"
git commit -m "Update formula for ChaseAI v${VERSION}"

if [ -n "${GITHUB_TOKEN}" ]; then
  git remote set-url origin "https://x-access-token:${GITHUB_TOKEN}@github.com/${FORMULA_REPO}.git"
fi

git push origin main || git push origin master

cd ..

echo ""
echo "✅ Homebrew formula updated successfully!"
echo "   Version: ${VERSION}"
echo "   SHA256: ${SHA256}"
echo ""
echo "Users can now install with:"
echo "  brew tap chaseai/chaseai"
echo "  brew install chaseai"
echo ""
