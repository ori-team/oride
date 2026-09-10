#!/bin/sh
set -eu
echo "=== Verifying Bash Strict Defensive Engineering ==="
atlas tool check-escape-hatches .
exit 0
