---
name: lang-bash
description: Bash strict mode (set -euo pipefail), ShellCheck clean, explicit parameter quoting, trap handlers for temp cleanup, and zero eval.
---

# Bash Strict Defensive Engineering Contract

## 1. Strict Baseline
- Every script must begin with `#!/usr/bin/env bash` or `#!/bin/sh` and immediately declare `set -euo pipefail`.
- Always quote variable expansions: `"$variable"`.
- Prohibit `eval` on dynamic or external input.

## 2. Temporary Resource Cleanup
- Use `trap` handlers to ensure temporary directories created via `mktemp` are deleted upon exit or termination signals (`EXIT INT TERM`).
