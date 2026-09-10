#!/bin/sh
set -eu
echo "=== Verifying Rust Safe-by-Default & Non-Lexical Lifetimes ==="
atlas tool check-escape-hatches .
exit 0
