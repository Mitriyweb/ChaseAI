#!/bin/bash

set -e

# Navigate to the project root
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cd "$SCRIPT_DIR/.."

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Get current version
CURRENT_VERSION=$(cat .version | tr -d '[:space:]')
echo -e "${BLUE}Current version: ${GREEN}$CURRENT_VERSION${NC}"

# Parse version components
IFS='.' read -r MAJOR MINOR PATCH <<< "$CURRENT_VERSION"

# Determine bump type
BUMP_TYPE=${1:-patch}

case $BUMP_TYPE in
  major)
    MAJOR=$((MAJOR + 1))
    MINOR=0
    PATCH=0
    ;;
  minor)
    MINOR=$((MINOR + 1))
    PATCH=0
    ;;
  patch)
    PATCH=$((PATCH + 1))
    ;;
  *)
    echo -e "${RED}Invalid bump type: $BUMP_TYPE${NC}"
    echo "Usage: $0 [major|minor|patch]"
    exit 1
    ;;
esac

NEW_VERSION="$MAJOR.$MINOR.$PATCH"
echo -e "${BLUE}New version: ${GREEN}$NEW_VERSION${NC}"

# Get git log since last tag
LAST_TAG=$(git describe --tags --abbrev=0 2>/dev/null || echo "")

if [ -z "$LAST_TAG" ]; then
  echo -e "${YELLOW}No previous tags found, using all commits${NC}"
  COMMITS=$(git log --oneline --pretty=format:"%h %s")
else
  echo -e "${BLUE}Changes since ${GREEN}$LAST_TAG${NC}"
  COMMITS=$(git log "$LAST_TAG"..HEAD --oneline --pretty=format:"%h %s")
fi

# Generate changelog entry
CHANGELOG_ENTRY="# v$NEW_VERSION

## What's New

"

# Parse commits into categories
FEATURES=""
FIXES=""
IMPROVEMENTS=""
OTHER=""

while IFS= read -r line; do
  if [[ -z "$line" ]]; then
    continue
  fi

  HASH=$(echo "$line" | awk '{print $1}')
  MESSAGE=$(echo "$line" | cut -d' ' -f2-)

  if [[ $MESSAGE =~ ^feat ]]; then
    FEATURES+="- $MESSAGE ($HASH)\n"
  elif [[ $MESSAGE =~ ^fix ]]; then
    FIXES+="- $MESSAGE ($HASH)\n"
  elif [[ $MESSAGE =~ ^(perf|refactor|docs) ]]; then
    IMPROVEMENTS+="- $MESSAGE ($HASH)\n"
  else
    OTHER+="- $MESSAGE ($HASH)\n"
  fi
done <<< "$COMMITS"

if [ -n "$FEATURES" ]; then
  CHANGELOG_ENTRY+="### Features\n$FEATURES\n"
fi

if [ -n "$FIXES" ]; then
  CHANGELOG_ENTRY+="### Fixes\n$FIXES\n"
fi

if [ -n "$IMPROVEMENTS" ]; then
  CHANGELOG_ENTRY+="### Improvements\n$IMPROVEMENTS\n"
fi

if [ -n "$OTHER" ]; then
  CHANGELOG_ENTRY+="### Other\n$OTHER\n"
fi

# Create CHANGELOG.md if it doesn't exist
if [ ! -f CHANGELOG.md ]; then
  echo -e "${BLUE}Creating CHANGELOG.md${NC}"
  echo "# Changelog" > CHANGELOG.md
  echo "" >> CHANGELOG.md
fi

# Prepend new entry to CHANGELOG.md
{
  echo -e "$CHANGELOG_ENTRY"
  cat CHANGELOG.md
} > CHANGELOG.md.tmp
mv CHANGELOG.md.tmp CHANGELOG.md

# Update .version file
echo "$NEW_VERSION" > .version

# Update Cargo.toml
sed -i '' "s/^version = \".*\"/version = \"$NEW_VERSION\"/" Cargo.toml

# Update package.json (add version field back temporarily for reference)
# Note: package.json doesn't have version field anymore, but we keep it for reference

echo -e "${GREEN}✓ Version bumped to $NEW_VERSION${NC}"
echo -e "${GREEN}✓ CHANGELOG.md updated${NC}"
echo -e "${GREEN}✓ Cargo.toml updated${NC}"

echo ""
echo -e "${BLUE}Next steps:${NC}"
echo "1. Review CHANGELOG.md"
echo "2. Commit changes: git add .version Cargo.toml CHANGELOG.md && git commit -m 'chore: bump version to $NEW_VERSION'"
echo "3. Create tag: git tag v$NEW_VERSION"
echo "4. Push: git push origin main && git push origin v$NEW_VERSION"
echo ""
echo -e "${YELLOW}Preview of CHANGELOG.md:${NC}"
head -20 CHANGELOG.md

# Auto-commit and tag
echo ""
echo -e "${BLUE}Committing and tagging...${NC}"

git add .version Cargo.toml CHANGELOG.md
git commit -m "chore: bump version to $NEW_VERSION"

git tag "v$NEW_VERSION"
echo -e "${GREEN}✓ Created tag v$NEW_VERSION${NC}"

# Push changes and tag
echo -e "${BLUE}Pushing to remote...${NC}"
git push origin main
git push origin "v$NEW_VERSION"

echo -e "${GREEN}✓ All done! Release v$NEW_VERSION is ready.${NC}"
echo -e "${BLUE}GitHub Actions will automatically create a release.${NC}"
