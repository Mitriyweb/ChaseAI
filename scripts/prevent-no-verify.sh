#!/bin/bash

# This script is a reminder that --no-verify should not be used
# All commits must pass pre-commit hooks

# Note: This is informational. To truly prevent --no-verify,
# you would need to use a server-side hook or organizational policy.
# This script serves as documentation of the requirement.

echo "ℹ️  Reminder: Please do not use 'git commit --no-verify'"
echo "   All commits must pass pre-commit hooks to maintain code quality."
echo ""

exit 0
