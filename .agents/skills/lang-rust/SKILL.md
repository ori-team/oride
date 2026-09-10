---
name: lang-rust
description: Safe Rust by default, borrow checker compliance, miri validation of unsafe blocks, clippy -D warnings, and explicit error handling via Result/Option.
---

# Rust Safe-by-Default Contract

## 1. Safe by Default & Unsafe Quarantine
- Production code must be `#![forbid(unsafe_code)]` unless an explicit waiver is registered in `.atlas/escape-hatches.json`.
- Any required `unsafe` block must state its safety invariants with `// SAFETY:` and be tested under `cargo miri`.

## 2. Error Handling & Unwraps
- Prohibit `.unwrap()` and `.expect()` in non-test paths. Propagate errors via `?` operator or return typed errors using `thiserror`.
- Compile with `cargo clippy --all-targets -- -D warnings`.
