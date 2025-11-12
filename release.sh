#!/bin/bash
set -e

# Flower Password Rust - Release Script
# Usage: ./release.sh [patch|minor|major]

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Parse arguments
BUMP_TYPE=${1:-patch}

if [ "$BUMP_TYPE" != "patch" ] && [ "$BUMP_TYPE" != "minor" ] && [ "$BUMP_TYPE" != "major" ]; then
  echo -e "${RED}Error: Invalid bump type${NC}"
  echo "Usage: ./release.sh [patch|minor|major]"
  echo ""
  echo "  patch - Bug fixes (e.g., 1.0.0 → 1.0.1)"
  echo "  minor - New features (e.g., 1.0.0 → 1.1.0)"
  echo "  major - Breaking changes (e.g., 1.0.0 → 2.0.0)"
  exit 1
fi

echo -e "${BLUE}🚀 Flower Password Rust Release Script${NC}"
echo ""

# Check if we're on main/master branch
CURRENT_BRANCH=$(git rev-parse --abbrev-ref HEAD)
if [ "$CURRENT_BRANCH" != "main" ] && [ "$CURRENT_BRANCH" != "master" ]; then
  echo -e "${YELLOW}⚠️  Warning: Not on main/master branch (current: $CURRENT_BRANCH)${NC}"
  read -p "Continue anyway? (y/N) " -n 1 -r
  echo
  if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    exit 1
  fi
fi

# Check for uncommitted changes
if ! git diff-index --quiet HEAD --; then
  echo -e "${RED}Error: You have uncommitted changes${NC}"
  echo "Please commit or stash your changes before releasing"
  exit 1
fi

# Install cargo-edit if not present
if ! command -v cargo-set-version &> /dev/null; then
  echo -e "${YELLOW}Installing cargo-edit...${NC}"
  cargo install cargo-edit
fi

# Get current version
CURRENT_VERSION=$(grep -m 1 '^version = ' Cargo.toml | cut -d'"' -f2)
echo -e "Current version: ${GREEN}$CURRENT_VERSION${NC}"

# Bump version
echo -e "${BLUE}Bumping version ($BUMP_TYPE)...${NC}"
cargo set-version --bump $BUMP_TYPE

# Get new version
NEW_VERSION=$(grep -m 1 '^version = ' Cargo.toml | cut -d'"' -f2)
echo -e "New version: ${GREEN}$NEW_VERSION${NC}"
echo ""

# Run tests
echo -e "${BLUE}Running tests...${NC}"
cargo test --all-features --verbose

# Run clippy
echo -e "${BLUE}Running clippy...${NC}"
cargo clippy -- -D warnings

# Format code
echo -e "${BLUE}Formatting code...${NC}"
cargo fmt

echo ""
echo -e "${GREEN}✅ All checks passed!${NC}"
echo ""

# Confirm release
echo -e "${YELLOW}Ready to release v$NEW_VERSION${NC}"
read -p "Create commit and tag? (y/N) " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
  echo -e "${RED}Release cancelled${NC}"
  # Revert version change
  git checkout Cargo.toml
  exit 1
fi

# Commit and tag
echo -e "${BLUE}Creating commit and tag...${NC}"
git add Cargo.toml
git commit -m "chore: bump version to $NEW_VERSION"
git tag -a "v$NEW_VERSION" -m "Release v$NEW_VERSION"

echo ""
echo -e "${GREEN}✅ Commit and tag created${NC}"
echo ""

# Confirm push
read -p "Push to remote? (y/N) " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
  echo -e "${YELLOW}Not pushed to remote${NC}"
  echo "To push manually, run:"
  echo "  git push origin $CURRENT_BRANCH --follow-tags"
  exit 0
fi

# Push
echo -e "${BLUE}Pushing to remote...${NC}"
git push origin "$CURRENT_BRANCH" --follow-tags

echo ""
echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}✅ Successfully released v$NEW_VERSION!${NC}"
echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""
echo -e "Previous version: ${BLUE}v$CURRENT_VERSION${NC}"
echo -e "New version:      ${GREEN}v$NEW_VERSION${NC}"
echo -e "Bump type:        ${YELLOW}$BUMP_TYPE${NC}"
echo ""
echo -e "${BLUE}🚀 GitHub Actions will now:${NC}"
echo "   1. Run CI tests"
echo "   2. Publish to crates.io"
echo "   3. Create GitHub Release"
echo ""
echo "View progress at:"
echo "  https://github.com/xlsdg/flowerpassword.rust/actions"
echo ""
