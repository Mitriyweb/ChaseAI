#!/bin/bash

# Kill any existing instances
pkill -f "ChaseAI.app/Contents/MacOS/ChaseAI" 2>/dev/null

# Run the app directly (not through 'open')
echo "Starting ChaseAI..."
exec target/release/ChaseAI.app/Contents/MacOS/ChaseAI
