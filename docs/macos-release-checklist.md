# macOS Release Checklist

## Pre-Release Verification

- [ ] All tests pass: `cargo test`
- [ ] Code formatted: `cargo fmt`
- [ ] Clippy passes: `cargo clippy`
- [ ] Version updated in `Cargo.toml`
- [ ] CHANGELOG.md updated with release notes
- [ ] Git repository is clean (no uncommitted changes)

## Build Verification

- [ ] Universal binary builds successfully
- [ ] App bundle creates without errors
- [ ] DMG packaging completes
- [ ] Code signing succeeds (with valid certificate)
- [ ] Notarization completes successfully
- [ ] Checksums generated correctly

## Testing

- [ ] DMG mounts and opens correctly
- [ ] Application launches from DMG
- [ ] Drag-and-drop installation works
- [ ] Installed app runs correctly
- [ ] Homebrew formula installs successfully
- [ ] Installed app via Homebrew runs correctly
- [ ] Test on Intel Mac (if available)
- [ ] Test on Apple Silicon Mac (if available)

## Release Steps

1. Create and push version tag:

   ```bash
   git tag -a v0.1.0 -m "Release v0.1.0"
   git push origin v0.1.0
   ```

2. GitHub Actions workflow runs automatically:
   - Builds universal binary
   - Creates and signs DMG
   - Notarizes DMG
   - Creates GitHub Release
   - Updates Homebrew formula

3. Verify GitHub Release:
   - [ ] DMG file uploaded
   - [ ] SHA256 checksum file uploaded
   - [ ] Release notes generated
   - [ ] Release marked as latest

4. Verify Homebrew formula:
   - [ ] Formula updated in homebrew-chaseai
   - [ ] Formula version matches release
   - [ ] SHA256 checksum matches DMG

## Post-Release

- [ ] Announce release on GitHub Discussions
- [ ] Update website/documentation
- [ ] Monitor for user issues
- [ ] Prepare for next release

## Rollback Procedure

If issues are discovered after release:

1. Delete the GitHub Release
2. Revert the Homebrew formula to previous version
3. Investigate and fix the issue
4. Create a new release with the fix

## Certificate Renewal

- [ ] Certificate expiration date: [DATE]
- [ ] Renewal reminder set for 3 months before expiration
- [ ] Renewal process documented
- [ ] Test renewal before expiration
