# Oride Workspace Crates (`crates/`)

This directory contains the modular crates that compose the **Oride** terminal code editor and mini-IDE.

---

## Crate Overview

| Crate | Responsibility | Dependencies |
|---|---|---|
| [`oride`](oride/) | Binary entrypoint and command-line interface driver | `oride-app`, `oride-config` |
| [`oride-app`](oride-app/) | Top-level application state, event loop, and UI composition | `oride-core`, `oride-ui`, `oride-config`, etc. |
| [`oride-core`](oride-core/) | Headless text buffers (`ropey`), document tabs, selections, undo/redo history | *Headless* (Zero UI / Ratatui dependencies) |
| [`oride-config`](oride-config/) | Layered TOML configuration loading, schema validation, and merging | `serde`, `toml` |
| [`oride-keymap`](oride-keymap/) | Key chord parsing, customizable layers, and `Action` dispatch | `serde`, `crossterm` key types |
| [`oride-fs`](oride-fs/) | Project file tree, filesystem operations, and Nerd Font icons | `walkdir`, `notify` |
| [`oride-git`](oride-git/) | Git porcelain status integration, branch detection, and diffs | CLI `git` subprocess |
| [`oride-terminal`](oride-terminal/) | Embedded PTY interactive terminal emulator panel | `portable-pty`, `vte` |
| [`oride-syntax`](oride-syntax/) | Tree-Sitter AST highlighting engine, queries, and Markdown rendering | `tree-sitter`, language grammars |
| [`oride-lsp`](oride-lsp/) | Asynchronous JSON-RPC Language Server Protocol stdio client | `tokio`, `serde_json` |
| [`oride-search`](oride-search/) | Active buffer and project-wide search (`ripgrep` + fallback) | `regex`, `ignore` |
| [`oride-plugin`](oride-plugin/) | Extensibility traits (`LanguageProvider`, `Plugin`) and command host | Core traits |
| [`oride-i18n`](oride-i18n/) | Dynamic localization catalog loader and string formatting | `serde`, `toml` |
| [`oride-ui`](oride-ui/) | Ratatui widgets, layout rendering, and theme color application | `ratatui`, `crossterm` |
| [`tree-sitter-oriscript`](tree-sitter-oriscript/) | Vendored legacy grammar binding | `tree-sitter` |

---

## Architectural Rules
1. **Headless Core Invariant:** `oride-core`, `oride-config`, and `oride-keymap` must never depend on `ratatui` or terminal graphics.
2. **Action lingua franca:** All user interactions dispatch strongly-typed `Action` variants.
3. **Fail-closed:** Failures in external processes (LSP, Git, PTY) report errors to the status line without panicking.
