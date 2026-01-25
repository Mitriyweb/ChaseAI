# macOS Distribution Specification

## Purpose

To provide native macOS distribution channels for ChaseAI through DMG installer and Homebrew package manager, enabling users to easily install and update the application on macOS systems.

## ADDED Requirements

### Requirement: DMG Installer Creation

The system MUST create a signed and notarized DMG installer for macOS distribution.

#### Scenario: DMG Build Process

- **WHEN** a release is triggered in CI/CD
- **THEN** the system MUST:
  - Build a universal binary supporting both x86_64 and aarch64 architectures
  - Create a DMG file containing the application bundle
  - Code-sign the DMG with a valid Apple Developer ID certificate
  - Submit the DMG to Apple Notarization Service
  - Staple the notarization ticket to the DMG
  - Generate SHA256 checksums for integrity verification

#### Scenario: DMG Distribution

- **WHEN** a user downloads the DMG from GitHub Releases
- **THEN** the system MUST:
  - Allow the user to mount the DMG on macOS
  - Display the application bundle in a Finder window
  - Allow drag-and-drop installation to Applications folder
  - Verify code signature and notarization status
  - Allow the application to run without Gatekeeper warnings

### Requirement: Homebrew Package Distribution

The system MUST provide a Homebrew formula for easy installation via package manager.

#### Scenario: Homebrew Formula Creation

- **WHEN** a new release is published
- **THEN** the system MUST:
  - Create or update a Homebrew formula in the tap repository
  - Specify the correct download URL pointing to GitHub Releases
  - Include SHA256 checksum for the DMG
  - Define any required dependencies
  - Pass Homebrew audit checks

#### Scenario: Homebrew Installation

- **WHEN** a user runs `brew install chaseai`
- **THEN** the system MUST:
  - Download the DMG from GitHub Releases
  - Verify the SHA256 checksum
  - Extract and install the application
  - Make the application available in the user's PATH
  - Allow the application to run without errors

#### Scenario: Homebrew Updates

- **WHEN** a new version is released
- **THEN** the system MUST:
  - Automatically update the Homebrew formula
  - Allow users to run `brew upgrade chaseai` to update
  - Maintain version history in the tap repository

### Requirement: Code Signing and Notarization

The system MUST implement proper code signing and notarization for macOS Gatekeeper compatibility.

#### Scenario: Code Signing

- **WHEN** the application is built for macOS
- **THEN** the system MUST:
  - Load the Apple Developer ID certificate from secure storage
  - Sign the application bundle with the certificate
  - Verify the signature is valid using `codesign -v`
  - Fail the build if signing fails

#### Scenario: Notarization

- **WHEN** the DMG is created
- **THEN** the system MUST:
  - Submit the DMG to Apple Notarization Service
  - Poll for notarization status with exponential backoff
  - Staple the notarization ticket to the DMG upon success
  - Fail the build if notarization fails or times out

#### Scenario: Gatekeeper Compatibility

- **WHEN** a user opens the application from the DMG
- **THEN** the system MUST:
  - Pass Gatekeeper verification
  - Not display security warnings
  - Allow the application to run immediately

### Requirement: CI/CD Integration

The system MUST integrate macOS distribution into the existing CI/CD pipeline.

#### Scenario: macOS Build Job

- **WHEN** code is pushed to the main branch
- **THEN** the system MUST:
  - Trigger a macOS build job on GitHub Actions
  - Build the application for both x86_64 and aarch64
  - Create a universal binary
  - Generate a DMG installer
  - Upload the DMG as a workflow artifact

#### Scenario: Release Workflow

- **WHEN** a version tag is pushed (v*.*.*)
- **THEN** the system MUST:
  - Trigger the release workflow
  - Build and sign the DMG
  - Create a GitHub Release with the DMG artifact
  - Update the Homebrew formula
  - Publish the release

### Requirement: Architecture Support

The system MUST support both Intel and Apple Silicon architectures.

#### Scenario: Universal Binary Creation

- **WHEN** the application is built for macOS
- **THEN** the system MUST:
  - Build the Rust binary for x86_64 architecture
  - Build the Rust binary for aarch64 architecture
  - Combine both binaries into a universal binary using `lipo`
  - Verify the universal binary contains both architectures

#### Scenario: Architecture Detection

- **WHEN** the application is installed and run
- **THEN** the system MUST:
  - Automatically detect the host architecture
  - Execute the appropriate binary slice
  - Provide equivalent functionality on both architectures

### Requirement: Release Artifact Management

The system MUST manage and publish release artifacts correctly.

#### Scenario: Artifact Generation

- **WHEN** a release is built
- **THEN** the system MUST:
  - Generate the DMG file
  - Generate SHA256 checksums
  - Create a checksum file for verification
  - Store artifacts in a consistent location

#### Scenario: Artifact Publication

- **WHEN** a release is ready
- **THEN** the system MUST:
  - Upload the DMG to GitHub Releases
  - Upload the checksum file to GitHub Releases
  - Make artifacts publicly accessible
  - Retain artifacts for at least 1 year

#### Scenario: Checksum Verification

- **WHEN** a user downloads the DMG
- **THEN** the system MUST:
  - Provide SHA256 checksums for verification
  - Document how to verify checksums
  - Allow users to confirm download integrity

## Related Capabilities

- `specs/app/spec.md` - Core application structure and build
- `specs/server-lifecycle/spec.md` - Application lifecycle management

## Implementation Notes

- Code signing requires an Apple Developer account ($99/year)
- Notarization is mandatory for Gatekeeper compatibility on modern macOS
- Universal binaries provide better user experience than separate builds
- Homebrew formula should be maintained in a separate tap repository
- Certificate management must be handled securely in CI/CD environment
