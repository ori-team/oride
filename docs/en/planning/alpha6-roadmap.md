# Oride — Implementation Plan for `0.2.0` and Beyond

**Status:** Normative  
**Current Release Version:** **`0.2.0`**  
**Precedence:** This document > `post-0.1-roadmap.md` (historical) > historical design notes  
**Product:** Contained TUI IDE (all features contained in the Oride process and terminal). Anti-bloat.

---

## 1. Core Principles (Anti-Bloat)

| # | Principle | Practical Implication |
|---|---|---|
| P1 | **Contained in TUI** | Features live inside Ratatui / Crossterm / PTY; no external browser spawn for Markdown preview |
| P2 | **One Concept per PR** | Small, focused, verified pull requests; no bloated premature external frameworks |
| P3 | **Defined Supported Languages** | Clear primary language tier; fallback to lexical highlight or plain text for edge cases |
| P4 | **Git via CLI** | Standard subprocess execution (`git status --porcelain`, diff, blame) |
| P5 | **Fail Closed / Status Line** | Failures in LSP, Git, or PTY report clean warnings, never crash the process |
| P6 | **Living Documentation** | Technical specifications, README, and user guides updated in the same slice |
| P7 | **Polish Shipped Flows** | Mature existing features before introducing new interface surfaces |

### Explicitly Out of Scope
- Heavy keystroke macros (removed in favor of clean core editing)
- External HTML/browser previewers
- Complex uncontained cloud dependencies
- Monolithic multi-source telescope monstrosities
- Unbounded visual tree branching

---

## 2. Feature Baseline (`0.2.0`)

| Subsystem | Delivered Capabilities |
|---|---|
| Core Editor | Rope buffer, multi-tab, undo/redo, text selection, soft wrap, comment toggle, modal editing |
| Project File Tree | Recursive expansion, create file/directory, rename, delete, Git status badges, Nerd Fonts |
| Terminal | Interactive PTY, vertical resizing, seamless keyboard focus toggle |
| Search & Replace | Active buffer find & replace (Case/Accent/Word/Regex) + recursive project-wide search |
| Git Integration | Tree badges, dedicated SCM panel, branch status, staged/unstaged tracking, diff view |
| Markdown | Tree-Sitter AST, code fence syntax injections, in-terminal preview, terminal image protocol support |
| Language Intelligence | Multi-LSP on-demand client (`[lsp.servers]`), semantic hover, definition jump, code formatting |
| Layout & Ergonomics | Split panes (vertical & horizontal), multi-cursor, opt-in mouse support, Task Runner (`tasks.toml`), `:health` diagnostics |
