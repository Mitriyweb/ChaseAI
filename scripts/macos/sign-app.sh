#!/bin/bash

# Code sign the macOS application
# This script loads the code signing certificate from GitHub Secrets,
# imports it into the keychain, and signs the application bundle.

set -e

echo "🔐 Code signing macOS application..."

# Configuration
APP_NAME="ChaseAI"
BINARY_NAME="chase-ai"
RELEASE_DIR="target/release"
APP_BUNDLE="${RELEASE_DIR}/${APP_NAME}.app"
KEYCHAIN_PATH="${HOME}/Library/Keychains/build.keychain-db"
KEYCHAIN_PASSWORD="$(openssl rand -base64 32)"

# Verify app bundle exists
if [ ! -d "${APP_BUNDLE}" ]; then
  echo "❌ Error: App bundle not found at ${APP_BUNDLE}"
  exit 1
fi

# Check if certificate and password are provided via environment variables
if [ -z "${MACOS_CERTIFICATE}" ] || [ -z "${MACOS_CERTIFICATE_PWD}" ]; then
  echo "⚠️  Warning: MACOS_CERTIFICATE or MACOS_CERTIFICATE_PWD not set"
  echo "   Skipping code signing (development mode)"
  echo "   To enable signing, set these environment variables:"
  echo "   - MACOS_CERTIFICATE (base64-encoded .p12 file)"
  echo "   - MACOS_CERTIFICATE_PWD (certificate password)"
  exit 0
fi

echo "📋 Setting up keychain..."

# Create a temporary keychain for signing
security create-keychain -p "${KEYCHAIN_PASSWORD}" "${KEYCHAIN_PATH}" || true
security default-keychain -s "${KEYCHAIN_PATH}"
security unlock-keychain -p "${KEYCHAIN_PASSWORD}" "${KEYCHAIN_PATH}"

# Import certificate
echo "📦 Importing certificate..."
CERTIFICATE_PATH="${RUNNER_TEMP}/certificate.p12"
echo "${MACOS_CERTIFICATE}" | base64 --decode > "${CERTIFICATE_PATH}"

security import "${CERTIFICATE_PATH}" \
  -k "${KEYCHAIN_PATH}" \
  -P "${MACOS_CERTIFICATE_PWD}" \
  -T /usr/bin/codesign

# Set keychain settings
security set-key-partition-list -S apple-tool:,apple:,codesign: -s -k "${KEYCHAIN_PASSWORD}" "${KEYCHAIN_PATH}"

# Get the certificate identity
echo "🔍 Finding certificate identity..."
CERTIFICATE_IDENTITY=$(security find-identity -v -p codesigning "${KEYCHAIN_PATH}" | grep "Developer ID Application" | head -1 | awk '{print $2}')

if [ -z "${CERTIFICATE_IDENTITY}" ]; then
  echo "❌ Error: Could not find Developer ID Application certificate"
  exit 1
fi

echo "   Found: ${CERTIFICATE_IDENTITY}"

# Sign the application bundle
echo "✍️  Signing application bundle..."
codesign --force --options runtime \
  --sign "${CERTIFICATE_IDENTITY}" \
  "${APP_BUNDLE}"

# Verify signature
echo "✅ Verifying signature..."
codesign -v "${APP_BUNDLE}"

# Display signature info
echo ""
echo "✅ Code signing completed successfully!"
echo "   Certificate: ${CERTIFICATE_IDENTITY}"
echo "   Bundle: ${APP_BUNDLE}"
echo ""

# Clean up
echo "🧹 Cleaning up..."
rm -f "${CERTIFICATE_PATH}"
security delete-keychain "${KEYCHAIN_PATH}" || true

echo "✅ Done!"
