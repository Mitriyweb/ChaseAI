#!/bin/bash

# Publish Release Script
# Builds the app, creates a DMG, and uploads it to GitHub Releases

set -e

# Navigate to project root
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cd "$SCRIPT_DIR/.."

# Check if GH CLI is installed
if ! command -v gh &> /dev/null; then
    echo "❌ Error: GitHub CLI (gh) is not installed."
    echo "   Install it with: brew install gh"
    exit 1
fi

# Check if authenticated
if ! gh auth status &> /dev/null; then
    echo "❌ Error: Not authenticated with GitHub."
    echo "   Run: gh auth login"
    exit 1
fi

# 1. Bump version (optional but recommended)
echo "Current version in Cargo.toml: $(grep '^version =' Cargo.toml | head -1 | cut -d '"' -f2)"
read -p "Enter new version (or press Enter to keep current): " NEW_VERSION

if [ ! -z "$NEW_VERSION" ]; then
    echo "Updating version to $NEW_VERSION..."
    # Update Cargo.toml
    sed -i.bak "s/^version = \".*\"/version = \"$NEW_VERSION\"/" Cargo.toml && rm Cargo.toml.bak
    # Update package.json
    if grep -q "\"version\":" package.json; then
        sed -i.bak "s/\"version\": \".*\"/\"version\": \"$NEW_VERSION\"/" package.json && rm package.json.bak
    else
        # Add version after description
        sed -i.bak "/\"description\":/a \    \"version\": \"$NEW_VERSION\"," package.json && rm package.json.bak
    fi
    # Update .version file
    echo "$NEW_VERSION" > .version

    # Commit changes
    git add Cargo.toml package.json .version
    git commit -m "chore: bump version to $NEW_VERSION"
    git push
    VERSION=$NEW_VERSION
else
    VERSION=$(grep '^version =' Cargo.toml | head -1 | cut -d '"' -f2)
fi

TAG="v$VERSION"

echo "🚀 preparing release $TAG..."

# 2. Build App (Prod)
echo "🔨 Building Production App..."
bun run build:app

# 3. Create DMG
echo "📦 Creating DMG..."
./scripts/macos/create-dmg.sh "$VERSION"

DMG_FILE="target/release/chase-ai-$VERSION-macos.dmg"

if [ ! -f "$DMG_FILE" ]; then
    echo "❌ Error: DMG file not found at $DMG_FILE"
    exit 1
fi

# 4. Create GitHub Release
echo "⬆️  Creating GitHub Release..."

# Check if tag exists locally
if git rev-parse "$TAG" >/dev/null 2>&1; then
    echo "   Tag $TAG exists locally."
else
    echo "   Creating tag $TAG..."
    git tag "$TAG"
    git push origin "$TAG"
fi

# Upload assets
# Only if release doesn't exist, create it. If it does, edit it (or just upload).
if gh release view "$TAG" &> /dev/null; then
    echo "   Release $TAG exists. Uploading assets..."
    gh release upload "$TAG" "$DMG_FILE" "scripts/install.sh" --clobber
else
    echo "   Creating new release $TAG..."
    gh release create "$TAG" "$DMG_FILE" "scripts/install.sh" --title "ChaseAI $VERSION" --notes "Release $VERSION"
fi

echo ""
echo "✅ Release published successfully!"
echo "   Download URL: https://github.com/Mitriyweb/ChaseAI/releases/download/$TAG/$(basename $DMG_FILE)"
echo "   Install Command: curl -sL https://github.com/Mitriyweb/ChaseAI/releases/latest/download/install.sh | bash"
