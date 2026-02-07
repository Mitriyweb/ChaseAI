#!/bin/bash

# Notarize the macOS DMG with Apple
# This script submits the DMG to Apple's Notarization Service,
# polls for completion, and staples the notarization ticket.

set -e

# Navigate to the project root
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cd "$SCRIPT_DIR/../.."

echo "🔐 Notarizing macOS DMG..."

# Configuration
BINARY_NAME="chase-ai"
VERSION="${1:-0.1.0}"
RELEASE_DIR="target/release"
DMG_NAME="${BINARY_NAME}-${VERSION}-macos.dmg"
DMG_PATH="${RELEASE_DIR}/${DMG_NAME}"
MAX_WAIT_TIME=1800  # 30 minutes
POLL_INTERVAL=30    # 30 seconds

# Verify DMG exists
if [ ! -f "${DMG_PATH}" ]; then
  echo "❌ Error: DMG not found at ${DMG_PATH}"
  exit 1
fi

# Check if notarization credentials are provided
if [ -z "${NOTARIZATION_USERNAME}" ] || [ -z "${NOTARIZATION_PASSWORD}" ]; then
  echo "⚠️  Warning: NOTARIZATION_USERNAME or NOTARIZATION_PASSWORD not set"
  echo "   Skipping notarization (development mode)"
  echo "   To enable notarization, set these environment variables:"
  echo "   - NOTARIZATION_USERNAME (Apple ID email)"
  echo "   - NOTARIZATION_PASSWORD (app-specific password)"
  exit 0
fi

echo "📤 Submitting DMG for notarization..."

# Submit for notarization
NOTARIZATION_OUTPUT=$(xcrun notarytool submit "${DMG_PATH}" \
  --apple-id "${NOTARIZATION_USERNAME}" \
  --password "${NOTARIZATION_PASSWORD}" \
  --wait 2>&1)

echo "${NOTARIZATION_OUTPUT}"

# Extract request ID from output
REQUEST_ID=$(echo "${NOTARIZATION_OUTPUT}" | grep -i "id:" | head -1 | awk '{print $NF}')

if [ -z "${REQUEST_ID}" ]; then
  echo "❌ Error: Could not extract request ID from notarization response"
  exit 1
fi

echo "   Request ID: ${REQUEST_ID}"

# Poll for notarization status
echo "⏳ Waiting for notarization to complete..."
ELAPSED=0
while [ ${ELAPSED} -lt ${MAX_WAIT_TIME} ]; do
  STATUS_OUTPUT=$(xcrun notarytool info "${REQUEST_ID}" \
    --apple-id "${NOTARIZATION_USERNAME}" \
    --password "${NOTARIZATION_PASSWORD}" 2>&1)

  STATUS=$(echo "${STATUS_OUTPUT}" | grep -i "status:" | head -1 | awk '{print $NF}')

  if [ "${STATUS}" = "Accepted" ]; then
    echo "✅ Notarization accepted!"
    break
  elif [ "${STATUS}" = "Rejected" ]; then
    echo "❌ Notarization rejected!"
    echo "${STATUS_OUTPUT}"
    exit 1
  elif [ "${STATUS}" = "In Progress" ]; then
    echo "   Still processing... (${ELAPSED}s elapsed)"
    sleep ${POLL_INTERVAL}
    ELAPSED=$((ELAPSED + POLL_INTERVAL))
  else
    echo "   Status: ${STATUS}"
    sleep ${POLL_INTERVAL}
    ELAPSED=$((ELAPSED + POLL_INTERVAL))
  fi
done

if [ ${ELAPSED} -ge ${MAX_WAIT_TIME} ]; then
  echo "❌ Error: Notarization timed out after ${MAX_WAIT_TIME} seconds"
  exit 1
fi

# Staple the notarization ticket to the DMG
echo "📎 Stapling notarization ticket..."
xcrun stapler staple "${DMG_PATH}"

# Verify stapling
echo "✅ Verifying stapled ticket..."
xcrun stapler validate "${DMG_PATH}"

echo ""
echo "✅ Notarization completed successfully!"
echo "   DMG: ${DMG_PATH}"
echo "   Request ID: ${REQUEST_ID}"
echo ""
echo "The DMG is now ready for distribution!"
echo ""
