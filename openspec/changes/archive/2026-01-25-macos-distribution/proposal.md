# Proposal: macOS Distribution via DMG Installer and Homebrew

## Summary

Enable ChaseAI distribution on macOS through two complementary channels:
1. **DMG Installer**: Self-contained, signed, and notarized installer for direct downloads
2. **Homebrew Formula**: Package management integration for easy installation via `brew install chaseai`

This proposal covers building, signing, notarizing, and distributing the application on macOS while integrating with the existing CI/CD pipeline.

## Why

**Current State**: ChaseAI lacks native macOS distribution mechanisms. Users cannot easily install or update the application on macOS.

**Problem**:
- No official macOS distribution channel
- Manual builds required for macOS users
- No code signing or notarization (required for Gatekeeper on modern macOS)
- No package manager integration

**Opportunity**:
- Expand user base to macOS developers
- Provide professional, signed distribution
- Enable automatic updates via Homebrew
- Reduce friction for new users

## What Changes

### macos-distribution Spec

#### ADDED
- DMG installer creation and distribution capability
- Code signing and notarization workflow for macOS applications
- Homebrew formula and tap repository integration
- macOS-specific CI/CD build jobs and release automation
- Support for both Intel (x86_64) and Apple Silicon (aarch64) architectures
- Universal binary creation and distribution
- Release artifact management and checksums
- macOS installation documentation

#### MODIFIED
- GitHub Actions CI/CD pipeline (add macOS build job)
- Release workflow (add DMG and Homebrew artifact generation)

#### REMOVED
- None

#### RENAMED
- None

## What

### Scope

1. **DMG Installer Creation**
   - Build and package the ChaseAI application into a DMG file
   - Include code signing with Apple Developer certificate
   - Implement macOS notarization for Gatekeeper compatibility
   - Automate DMG generation in CI/CD pipeline

2. **Homebrew Formula**
   - Create and maintain a Homebrew formula for ChaseAI
   - Host formula in a tap repository or submit to Homebrew core
   - Automate formula updates on releases
   - Support both Intel and Apple Silicon architectures

3. **CI/CD Integration**
   - Add macOS build jobs to GitHub Actions workflow
   - Implement code signing and notarization steps
   - Automate DMG and Homebrew artifact generation
   - Publish artifacts to GitHub Releases

4. **Release Management**
   - Document macOS-specific release procedures
   - Define versioning strategy for DMG and Homebrew
   - Plan for future updates and maintenance

### Out of Scope

- Windows or Linux distribution (separate proposals)
- GUI installer beyond DMG (can be added later)
- Automatic update mechanism beyond Homebrew (future enhancement)
- Custom Homebrew tap repository setup (can be handled separately)

## Impact

### User Experience
- **Before**: Manual compilation or no macOS support
- **After**: One-click DMG installation or `brew install chaseai`

### Development
- Adds macOS-specific CI/CD jobs (~15-20 min per build)
- Requires Apple Developer account for code signing
- Introduces new build artifacts and release procedures

### Maintenance
- Ongoing: Update Homebrew formula on releases
- Ongoing: Maintain code signing certificates
- Ongoing: Monitor notarization status

## Technical Considerations

### Code Signing & Notarization
- Requires Apple Developer account ($99/year)
- Notarization process takes 5-15 minutes per build
- Must handle certificate management securely in CI/CD

### Architecture Support
- Intel (x86_64) and Apple Silicon (aarch64) support
- May require separate builds or universal binaries

### Dependencies
- `create-dmg` or similar tool for DMG creation
- Xcode Command Line Tools for code signing
- Notarization credentials management

## Related Specs

- `specs/app/spec.md` - Core application structure
- `specs/server-lifecycle/spec.md` - Application lifecycle management

## Risks & Mitigations

| Risk | Mitigation |
|------|-----------|
| Certificate expiration | Automated renewal reminders, documented process |
| Notarization failures | Retry logic, detailed logging, fallback procedures |
| Build time increase | Parallel macOS builds, caching strategies |
| Homebrew maintenance burden | Automated formula updates, clear documentation |

## Success Criteria

- [ ] DMG installer builds successfully in CI/CD
- [ ] Application is code-signed and notarized
- [ ] Homebrew formula is created and tested
- [ ] Release artifacts are published to GitHub Releases
- [ ] Documentation covers macOS installation methods
- [ ] Both Intel and Apple Silicon architectures are supported

## Next Steps

1. Review and approve this proposal
2. Create detailed design document (`design.md`)
3. Break down into implementation tasks (`tasks.md`)
4. Create spec deltas for new capabilities
5. Begin implementation phase
