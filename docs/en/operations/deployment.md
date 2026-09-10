# Deployment & Release Guide

## Release Requirements
1. Full test suite passing with 100% success rate across the Cargo workspace.
2. Zero Clippy warnings (`cargo clippy --all-targets -- -D warnings`) and formatted code (`cargo fmt --all -- --check`).
3. Clean SAST security audits and vulnerability checks.
4. Updated version manifest in `Cargo.toml` and release notes registered in `CHANGELOG.md`.

## Build Procedure
- Compilation of optimized release binaries:
  ```bash
  cargo build --release
  ```
- Local user installation:
  ```bash
  ./scripts/install.sh
  ```
- Execution of automated health check verification:
  ```bash
  ./target/release/oride --version
  ```
