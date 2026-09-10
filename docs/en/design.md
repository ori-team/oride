# Architecture & Technical Design: Modular Rust TUI Code Editor & Mini-IDE

## Context

**Oride** is a fast, modular, and extensible **terminal code editor and mini-IDE** inspired by the ergonomics of [Micro](https://micro-editor.github.io/) and [Helix], engineered in **Rust** with a core focus on systems languages (Rust, C, Bash/Shell), technical documentation (Markdown), the Ori ecosystem (`ori-lang`), and modern general-purpose languages via Tree-Sitter, integrated task runners, and standard Language Server Protocol (LSP).

Unlike minimal text viewers, **Oride** includes:

- Navigable project file tree (create/rename/delete files and directories, Git status badges)
- Collapsible embedded PTY terminal (`portable-pty` + VTE parser)
- Native syntax support for **Rust, C, Bash, Markdown, and Ori** (+ dynamic grammars & plugins)
- Dual editing paradigms: standard GUI-like navigation and Vim-style modal editing (`:normal`, `i`, `v`, `V`, `:`)
- Declarative task runner (`tasks.toml`) with variable substitution (`${file}`, `${dir}`, etc.)
- Preventative system & LSP diagnostics engine (`:health`)
- Keybinding configuration, live-reloading TOML themes, and dynamic i18n catalogs
- Nerd Font glyphs with automatic clean ASCII fallback
- Modular Rust workspace with strict separation between domain, core, and Ratatui UI

**Design Decisions:**

| Decision | Selection | Rationale |
|----------|-----------|-----------|
| Surface | **TUI** (Ratatui + Crossterm) | Modern Rust terminal standard (2024–2026) |
| Architecture | **Modular Cargo Workspace** | Strict separation between headless core logic and terminal UI |
| Scope | **Contained Mini-IDE** | Multi-tab, dynamic splits, embedded PTY, Git status, on-demand LSP, themes, task runner |

**Decoupled Language Intelligence:** All advanced language services (diagnostics, completion, hover, goto definition, formatting) operate via stdio-based **LSP on PATH** (`rust-analyzer`, `clangd`, `ori-lsp`), without embedding heavy language compilers directly inside the editor binary.

---

## Technical Stack

| Layer | Crate / Approach | Rationale |
|---|---|---|
| UI / Terminal | `ratatui` + `crossterm` | Idiomatic Rust TUI standard with cross-platform terminal control |
| Text Buffer | `ropey` | Efficient rope data structure, optimized for fast edits and undo/redo |
| Highlighting | `tree-sitter` + embedded queries | High performance AST generation, incremental queries, and Markdown injection |
| Embedded Terminal | `portable-pty` + `vte` | Real PTY allocation and interactive terminal emulator |
| Configuration | **TOML** (`serde` + `toml`) | Human-readable, layered merging, comments support |
| Keymaps | Custom crate + TOML | Multi-chord actions, customizable layers (defaults → user → workspace) |
| LSP Client | Asynchronous JSON-RPC (stdio) | Spawns standard language servers on demand |
| Git SCM | Subprocess `git status --porcelain` | Stable, lightweight, zero libgit2 C-dependency issues |
| File Watching | `notify` | Cross-platform filesystem event monitoring for tree refreshing |
| Clipboard | `arboard` + OSC 52 | System clipboard integration with remote SSH fallback |
| Async Runtime | `tokio` | Non-blocking channels for PTY, LSP, and background tasks |

---

## Modular Workspace Layout

```text
oride/
  Cargo.toml                    # Workspace definition
  README.md                     # Canonical English README
  README.pt-BR.md               # Portuguese README
  docs/
    en/                         # Canonical English documentation
    design.md                   # Architecture & design
    config.md                   # Configuration specification
    themes.md                   # Theme development guide
    guia-de-uso.md              # Portuguese user guide
  crates/
    oride-core/                 # Buffer (Rope), Selection, Documents, Undo history
    oride-config/               # Layered TOML loading & validation
    oride-keymap/               # Chords mapping to named Actions
    oride-fs/                   # File tree, icons, filesystem operations
    oride-git/                  # Git porcelain status integration
    oride-terminal/             # Embedded PTY panel & VTE terminal emulation
    oride-syntax/               # Tree-Sitter highlight engine & lexical fallbacks
    oride-lsp/                  # Multi-server Language Server Protocol client
    oride-search/               # Buffer search & project-wide ripgrep integration
    oride-plugin/               # Plugin traits & command host
    oride-i18n/                 # Dynamic localization catalog loader
    oride-ui/                   # Ratatui widgets & rendering logic
    oride-app/                  # Application composition & event loop
    oride/                      # Main CLI binary entrypoint
```

---

## Core Invariants

1. **Headless Core:** `oride-core` has zero dependencies on `ratatui` or terminal libraries; testable completely in headless unit tests.
2. **Actions as Data:** All keymaps, menus, and palette entries dispatch strongly-typed `Action` enums.
3. **Fail-Closed Principle:** Failures in LSP, Git, or PTY report warnings to the status line without crashing the editor.
4. **Decoupled Tooling:** Language intelligence uses external CLI and LSP tools from `$PATH`.
5. **Clean Architecture:** Domain and state management remain isolated from presentation and terminal drivers.
