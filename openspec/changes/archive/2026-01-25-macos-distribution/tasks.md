# Tasks: macOS Distribution via DMG and Homebrew

## Overview

Implementation tasks for building and distributing ChaseAI on macOS through DMG installer and Homebrew package manager.

---

## Phase 1: Foundation & Build Setup

### 1.1 Set up macOS build environment
- [x] Add macOS runner configuration to `.github/workflows/main.yml`
- [x] Configure Rust toolchain for macOS (x86_64 and aarch64)
- [x] Set up build caching for Rust dependencies
- [x] Test build succeeds on both architectures
- **Validation**: Build completes without errors on macOS runner

### 1.2 Create universal binary build script
- [x] Create `scripts/macos/build-universal.sh` script
- [x] Build Rust binary for x86_64 architecture
- [x] Build Rust binary for aarch64 architecture
- [x] Combine binaries using `lipo` tool
- [x] Verify universal binary contains both architectures
- **Validation**: `file` command shows "universal binary" with both architectures

### 1.3 Create DMG packaging script
- [x] Create `scripts/macos/create-dmg.sh` script
- [x] Install `create-dmg` npm package as dev dependency
- [x] Create DMG from application bundle
- [x] Add background image to DMG
- [x] Configure DMG layout (icon positioning, etc.)
- [x] Generate SHA256 checksums for DMG
- **Validation**: DMG file is created and can be mounted

### 1.4 Add macOS build job to CI/CD
- [x] Add `build-macos` job to `.github/workflows/main.yml`
- [x] Configure job to run on `macos-latest` runner
- [x] Call universal binary build script
- [x] Call DMG creation script
- [x] Upload DMG as workflow artifact
- **Validation**: Workflow runs successfully and produces DMG artifact

---

## Phase 2: Code Signing & Notarization

### 2.1 Set up Apple Developer credentials
- [x] Obtain Apple Developer account (if not already available)
- [x] Create App ID for ChaseAI
- [x] Generate code signing certificate (Developer ID Application)
- [x] Create app-specific password for notarization
- [x] Document certificate renewal process
- **Validation**: Certificate is valid and can be used for signing

### 2.2 Store credentials in GitHub Secrets
- [x] Export code signing certificate as base64-encoded .p12 file
- [x] Add `MACOS_CERTIFICATE` secret (base64 certificate)
- [x] Add `MACOS_CERTIFICATE_PWD` secret (certificate password)
- [x] Add `NOTARIZATION_USERNAME` secret (Apple ID email)
- [x] Add `NOTARIZATION_PASSWORD` secret (app-specific password)
- **Validation**: Secrets are accessible in CI/CD environment

### 2.3 Create code signing script
- [x] Create `scripts/macos/sign-app.sh` script
- [x] Load certificate from GitHub Secrets
- [x] Import certificate into keychain
- [x] Sign application bundle with certificate
- [x] Verify signature with `codesign -v` command
- [x] Clean up keychain after signing
- **Validation**: Application is signed and signature is valid

### 2.4 Create notarization script
- [x] Create `scripts/macos/notarize-app.sh` script
- [x] Submit DMG to Apple Notarization Service
- [x] Implement polling loop to check notarization status
- [x] Handle notarization success and failure cases
- [x] Staple notarization ticket to DMG
- [x] Implement retry logic for transient failures
- **Validation**: DMG is notarized and ticket is stapled

### 2.5 Integrate signing and notarization into CI/CD
- [x] Update macOS build job to call sign-app.sh
- [x] Update macOS build job to call notarize-app.sh
- [x] Add error handling for signing/notarization failures
- [x] Add logging for debugging
- [x] Test with real Apple Developer certificate
- **Validation**: Workflow completes with signed and notarized DMG

---

## Phase 3: Homebrew Integration

### 3.1 Create Homebrew tap repository
- [x] Create GitHub repository `chaseai/homebrew-chaseai`
- [x] Set up repository structure for Homebrew tap
- [x] Add README with installation instructions
- [x] Configure repository settings (branch protection, etc.)
- **Validation**: Repository is ready for formula submission

### 3.2 Write Homebrew formula
- [x] Create `Formula/chaseai.rb` in tap repository
- [x] Define formula with correct metadata (name, version, homepage)
- [x] Specify download URL pointing to GitHub Releases
- [x] Add SHA256 checksum for DMG
- [x] Define dependencies (if any)
- [x] Add installation instructions
- [x] Add test block to verify installation
- **Validation**: Formula passes `brew audit` checks

### 3.3 Test Homebrew formula locally
- [x] Install formula locally using `brew install --build-from-source`
- [x] Verify application is installed correctly
- [x] Verify application runs without errors
- [x] Test on both Intel and Apple Silicon Macs (if available)
- **Validation**: Application installs and runs via Homebrew

### 3.4 Create formula update automation
- [x] Create GitHub Actions workflow to update formula on releases
- [x] Workflow triggers on GitHub Release creation
- [x] Extract version and download URL from release
- [x] Calculate SHA256 checksum from DMG
- [x] Update formula with new version and checksum
- [x] Commit and push changes to tap repository
- **Validation**: Formula is automatically updated on new releases

### 3.5 Document Homebrew installation
- [x] Add Homebrew installation instructions to README
- [x] Document tap addition: `brew tap chaseai/chaseai`
- [x] Document installation: `brew install chaseai`
- [x] Document update: `brew upgrade chaseai`
- [x] Add troubleshooting section for common issues
- **Validation**: Instructions are clear and complete

---

## Phase 4: Release & Documentation

### 4.1 Create release workflow
- [x] Create `.github/workflows/macos-release.yml` (optional, can be inline)
- [x] Trigger on version tags (v*.*.*)
- [x] Build and sign DMG
- [x] Create GitHub Release with DMG artifact
- [x] Generate release notes
- [x] Publish release
- **Validation**: Release workflow completes successfully

### 4.2 Write user documentation
- [x] Create `docs/installation-macos.md` with installation methods
- [x] Document DMG installation (download, mount, drag-and-drop)
- [x] Document Homebrew installation (tap, install, upgrade)
- [x] Add system requirements (macOS version, architecture)
- [x] Add troubleshooting section
- [x] Add uninstallation instructions
- **Validation**: Documentation is clear and complete

### 4.3 Create release checklist
- [x] Document pre-release verification steps
- [x] Document manual testing procedures
- [x] Create checklist for release manager
- [x] Document rollback procedures
- [x] Document certificate renewal process
- **Validation**: Checklist is comprehensive and actionable

### 4.4 Test end-to-end installation
- [x] Download DMG from GitHub Releases
- [x] Install via DMG (mount, drag-and-drop)
- [x] Verify application runs correctly
- [x] Install via Homebrew (`brew install chaseai`)
- [x] Verify application runs correctly
- [x] Test on both Intel and Apple Silicon Macs
- **Validation**: Both installation methods work correctly

### 4.5 Publish first release
- [x] Create version tag (e.g., v0.1.0)
- [x] Push tag to trigger release workflow
- [x] Verify DMG is built and signed
- [x] Verify notarization completes successfully
- [x] Verify GitHub Release is created with artifacts
- [x] Verify Homebrew formula is updated
- [x] Announce release to users
- **Validation**: Release is published and accessible to users

---

## Phase 5: Maintenance & Future Work

### 5.1 Set up certificate renewal reminders
- [x] Document certificate expiration date
- [x] Set calendar reminders for renewal (3 months before expiration)
- [x] Document renewal process
- [x] Test renewal process before expiration
- **Validation**: Certificate renewal process is documented and tested

### 5.2 Monitor notarization service
- [x] Set up alerts for notarization failures
- [x] Document common notarization errors and solutions
- [x] Monitor Apple's notarization service status
- [x] Plan for service outages
- **Validation**: Monitoring is in place and alerts are configured

### 5.3 Plan future enhancements
- [x] Evaluate Sparkle framework for automatic updates
- [x] Plan Homebrew core submission
- [x] Plan GPG signing for releases
- [x] Plan staged rollout via beta tap
- **Validation**: Enhancement plan is documented

---

## Dependencies & Blockers

### External Dependencies
- Apple Developer account (required for code signing)
- GitHub Secrets configuration (required for CI/CD)
- Homebrew tap repository (required for Homebrew distribution)

### Internal Dependencies
- Existing Rust build system (specs/app/spec.md)
- Existing CI/CD pipeline (.github/workflows/main.yml)

### Parallel Work
- Tasks 1.1-1.4 can be done in parallel
- Tasks 2.1-2.2 must be done before 2.3-2.5
- Tasks 3.1-3.2 can be done in parallel with Phase 2
- Phase 4 depends on completion of Phases 1-3

---

## Success Criteria

- [x] macOS build job is integrated into CI/CD
- [x] DMG is created and packaged correctly
- [x] Application is code-signed with valid certificate
- [x] DMG is notarized and passes Gatekeeper
- [x] Homebrew formula is created and tested
- [x] Installation via DMG works correctly
- [x] Installation via Homebrew works correctly
- [x] Both Intel and Apple Silicon architectures are supported
- [x] Release workflow is automated
- [x] User documentation is complete
- [x] First release is published successfully

---

## Estimated Timeline

- **Phase 1**: 3-4 days
- **Phase 2**: 4-5 days (includes waiting for notarization)
- **Phase 3**: 3-4 days
- **Phase 4**: 2-3 days
- **Total**: 12-16 days (2-3 weeks)

Note: Timeline assumes no blockers and continuous work. Actual timeline may vary based on Apple Developer account setup and notarization service availability.
