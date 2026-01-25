# Design: macOS Distribution via DMG and Homebrew

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                    GitHub Actions CI/CD                      │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  ┌──────────────────┐         ┌──────────────────┐          │
│  │  Build Job       │         │  macOS Build Job │          │
│  │  (Linux/Tests)   │         │  (x86_64 + ARM)  │          │
│  └──────────────────┘         └──────────────────┘          │
│                                        │                     │
│                                        ▼                     │
│                          ┌─────────────────────────┐         │
│                          │  Code Signing & Build   │         │
│                          │  - Compile for x86_64   │         │
│                          │  - Compile for aarch64  │         │
│                          │  - Create universal bin │         │
│                          └─────────────────────────┘         │
│                                        │                     │
│                                        ▼                     │
│                          ┌─────────────────────────┐         │
│                          │  DMG Creation           │         │
│                          │  - Package application  │         │
│                          │  - Add background image │         │
│                          │  - Code sign DMG        │         │
│                          └─────────────────────────┘         │
│                                        │                     │
│                                        ▼                     │
│                          ┌─────────────────────────┐         │
│                          │  Notarization           │         │
│                          │  - Submit to Apple      │         │
│                          │  - Wait for approval    │         │
│                          │  - Staple ticket        │         │
│                          └─────────────────────────┘         │
│                                        │                     │
│                                        ▼                     │
│                          ┌─────────────────────────┐         │
│                          │  Publish Artifacts      │         │
│                          │  - Upload to Releases   │         │
│                          │  - Update Homebrew      │         │
│                          └─────────────────────────┘         │
│                                                               │
└─────────────────────────────────────────────────────────────┘
                              │
                ┌─────────────┴─────────────┐
                ▼                           ▼
        ┌──────────────────┐        ┌──────────────────┐
        │  GitHub Releases │        │  Homebrew Tap    │
        │  - DMG file      │        │  - Formula       │
        │  - Checksums     │        │  - Dependencies  │
        └──────────────────┘        └──────────────────┘
                │                           │
                ▼                           ▼
        ┌──────────────────┐        ┌──────────────────┐
        │  User Downloads  │        │  brew install    │
        │  - Manual DMG    │        │  chaseai         │
        └──────────────────┘        └──────────────────┘
```

## Component Design

### 1. Build System Integration

**Location**: `.github/workflows/main.yml` (new macOS job)

**Responsibilities**:
- Detect macOS runner availability
- Build Rust binary for both x86_64 and aarch64
- Create universal binary (lipo)
- Handle build caching for faster iterations

**Key Decisions**:
- Use GitHub-hosted macOS runners (latest available)
- Build both architectures separately, then combine
- Cache Rust dependencies to reduce build time

### 2. Code Signing & Notarization

**Location**: New workflow file `.github/workflows/macos-sign-notarize.yml` (optional, can be inline)

**Responsibilities**:
- Load code signing certificate from secrets
- Sign the application bundle
- Submit to Apple Notarization Service
- Poll for notarization status
- Staple notarization ticket to DMG

**Key Decisions**:
- Store certificate and password in GitHub Secrets
- Use `xcrun notarytool` for notarization (modern approach)
- Implement retry logic for notarization (can take 5-15 minutes)
- Fail build if notarization fails (security requirement)

**Security Considerations**:
- Certificate stored as base64-encoded secret
- Password stored separately
- Notarization credentials rotated annually
- Audit logging for all signing operations

### 3. DMG Creation

**Location**: New script `scripts/macos/create-dmg.sh`

**Responsibilities**:
- Create DMG container
- Copy application bundle
- Add background image and layout
- Code sign the DMG
- Generate checksums

**Key Decisions**:
- Use `create-dmg` npm package for consistency
- Include background image for professional appearance
- Generate SHA256 checksums for verification
- Store DMG in `target/release/` for CI/CD pickup

### 4. Homebrew Formula

**Location**: Separate tap repository or Homebrew core (future)

**Responsibilities**:
- Define formula with dependencies
- Specify download URL and checksums
- Handle both architectures
- Provide installation instructions

**Key Decisions**:
- Start with custom tap: `chaseai/chaseai/chaseai`
- Auto-update formula on GitHub Releases
- Use GitHub Releases as download source
- Include post-install instructions

**Formula Structure**:
```ruby
class Chaseai < Formula
  desc "Local control and orchestration system for AI agents"
  homepage "https://github.com/chaseai/chaseai"
  url "https://github.com/chaseai/chaseai/releases/download/v#{version}/chaseai-#{version}-macos.tar.gz"
  sha256 "..."

  depends_on "rust" => :build

  def install
    bin.install "chaseai"
  end

  test do
    system "#{bin}/chaseai", "--version"
  end
end
```

### 5. Release Workflow

**Location**: New workflow file `.github/workflows/macos-release.yml` (optional)

**Responsibilities**:
- Trigger on version tags (v*.*.*)
- Build and sign DMG
- Create GitHub Release
- Update Homebrew formula
- Publish release notes

**Key Decisions**:
- Trigger on git tags matching `v*` pattern
- Automatic release creation with artifacts
- Manual approval for Homebrew updates (initially)

## Implementation Phases

### Phase 1: Foundation (Weeks 1-2)
- Set up macOS build job in CI/CD
- Create basic DMG packaging script
- Generate test artifacts

### Phase 2: Signing & Notarization (Weeks 2-3)
- Obtain Apple Developer certificate
- Implement code signing in CI/CD
- Implement notarization workflow
- Test with real certificate

### Phase 3: Homebrew Integration (Weeks 3-4)
- Create Homebrew tap repository
- Write formula
- Test installation via Homebrew
- Document installation methods

### Phase 4: Release & Documentation (Week 4)
- Create release workflow
- Write user documentation
- Test end-to-end installation
- Publish first release

## Technical Decisions

### Universal Binary vs. Separate Builds
**Decision**: Create universal binary (single file supporting both architectures)
**Rationale**:
- Simpler distribution (one DMG file)
- Smaller download size than separate builds
- Better user experience
- Minimal performance overhead

### Notarization Approach
**Decision**: Use `xcrun notarytool` (modern, recommended by Apple)
**Rationale**:
- Replaces deprecated `altool`
- Better error messages
- Faster processing
- Supported on latest macOS versions

### Homebrew Distribution
**Decision**: Start with custom tap, plan for Homebrew core submission
**Rationale**:
- Faster initial release (no review queue)
- Full control over formula
- Can migrate to core later
- Easier to maintain and update

## Dependencies & Requirements

### External Tools
- `create-dmg` (npm package)
- Xcode Command Line Tools
- `xcrun` (included with Xcode)

### Credentials & Certificates
- Apple Developer account ($99/year)
- Code signing certificate (App ID)
- Notarization credentials (app-specific password)

### GitHub Secrets Required
- `MACOS_CERTIFICATE` (base64-encoded .p12)
- `MACOS_CERTIFICATE_PWD` (certificate password)
- `NOTARIZATION_USERNAME` (Apple ID)
- `NOTARIZATION_PASSWORD` (app-specific password)

## Testing Strategy

### Unit Testing
- Verify DMG creation script produces valid DMG
- Test checksum generation and verification

### Integration Testing
- Build and sign DMG in CI/CD
- Verify code signature with `codesign -v`
- Test notarization workflow
- Verify Homebrew formula syntax

### End-to-End Testing
- Download DMG and install manually
- Install via Homebrew and verify
- Test on both Intel and Apple Silicon Macs
- Verify application runs correctly

## Rollback & Maintenance

### Rollback Procedure
- Remove release from GitHub Releases
- Unpublish Homebrew formula (if needed)
- Revert to previous version tag

### Maintenance Tasks
- Annual certificate renewal
- Quarterly Homebrew formula updates
- Monitor notarization service status
- Update build tools as needed

## Security Considerations

### Code Signing
- Certificates stored securely in GitHub Secrets
- Signing happens in isolated CI/CD environment
- All artifacts verified before distribution

### Notarization
- Credentials stored separately from certificate
- Notarization logs retained for audit
- Failed notarizations block release

### Distribution
- Checksums published with releases
- Users can verify download integrity
- Homebrew provides additional verification

## Future Enhancements

1. **Automatic Updates**: Implement Sparkle framework for in-app updates
2. **Homebrew Core**: Submit formula to Homebrew core repository
3. **Signed Releases**: GPG sign release artifacts
4. **Staged Rollout**: Beta releases via separate Homebrew tap
5. **Analytics**: Track installation metrics via download counts
