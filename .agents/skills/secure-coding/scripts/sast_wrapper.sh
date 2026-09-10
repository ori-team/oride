#!/usr/bin/env sh
set -e
TARGET="."
echo "Running Static Application Security Testing (SAST) via Atlas Tooling..."
atlas tool scan-secrets ""
atlas tool check-subprocesses ""
atlas tool check-bare-errors ""
