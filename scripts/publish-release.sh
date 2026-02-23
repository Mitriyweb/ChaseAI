#!/bin/bash

# Publish Release Script
# Builds the app, creates DMG and tar.gz, and uploads them to GitHub Releases

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

# 1. Get version
VERSION=$(grep '^version =' Cargo.toml | head -1 | cut -d '"' -f2)
echo "Current version in Cargo.toml: $VERSION"
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
fi

TAG="v$VERSION"

echo "🚀 Preparing release $TAG..."

# 2. Build App (Prod)
echo "🔨 Building Production App..."
bun run build:app

# 3. Create Installers
echo "📦 Creating Installers..."
# These scripts handle their own platform checks
./scripts/macos/create-dmg.sh "$VERSION" || true
./scripts/macos/create-archive.sh "$VERSION"

DMG_FILE="target/release/chase-ai-$VERSION-macos.dmg"
ARCHIVE_FILE="target/release/chase-ai-$VERSION-macos.tar.gz"

# 4. Generate Unified Checksums
echo "🔐 Generating unified checksums..."
CHECKSUMS_FILE="target/release/checksums.sha256"
rm -f "$CHECKSUMS_FILE"

# Use a subshell to avoid changing directory in the main script
(
    cd target/release
    if [ -f "chase-ai-$VERSION-macos.dmg" ]; then
        shasum -a 256 "chase-ai-$VERSION-macos.dmg" >> "../../$CHECKSUMS_FILE"
    fi
    if [ -f "chase-ai-$VERSION-macos.tar.gz" ]; then
        shasum -a 256 "chase-ai-$VERSION-macos.tar.gz" >> "../../$CHECKSUMS_FILE"
    fi
)

# 5. Create GitHub Release
echo "⬆️  Creating GitHub Release..."

# Check if tag exists locally
if git rev-parse "$TAG" >/dev/null 2>&1; then
    echo "   Tag $TAG exists locally."
else
    echo "   Creating tag $TAG..."
    git tag "$TAG"
    git push origin "$TAG"
fi

# Prepare list of files to upload
UPLOAD_FILES=("$CHECKSUMS_FILE" "scripts/install.sh")
if [ -f "$DMG_FILE" ]; then UPLOAD_FILES+=("$DMG_FILE"); fi
if [ -f "$ARCHIVE_FILE" ]; then UPLOAD_FILES+=("$ARCHIVE_FILE"); fi

echo "   Uploading assets: ${UPLOAD_FILES[*]}"

# Upload assets
if gh release view "$TAG" &> /dev/null; then
    echo "   Release $TAG exists. Uploading assets..."
    gh release upload "$TAG" "${UPLOAD_FILES[@]}" --clobber
else
    echo "   Creating new release $TAG..."
    gh release create "$TAG" "${UPLOAD_FILES[@]}" --title "ChaseAI $VERSION" --notes "Release $VERSION"
fi

echo ""
echo "✅ Release published successfully!"
echo "   Install Command: curl -sL https://github.com/Mitriyweb/ChaseAI/releases/latest/download/install.sh | bash"
