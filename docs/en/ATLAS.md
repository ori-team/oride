# Project Atlas — Oride Terminal IDE (English Intent Router)

This is the canonical intent router for humans and agents. Add links as stable documentation is created; do not create empty documentation solely to populate this map.

---

## Current State & Framework

- [Project State](../../PROJECT_STATE.md) — Active phase, goal status, recent actions
- [`atlas.json`](../../atlas.json) — Canonical project configuration (Project Atlas v0.4.2, Protocol v3)
- [`AGENTS.md`](../../AGENTS.md) — Product invariants, core design rules, and validation commands

## AI Harnesses & Compilation

Project Atlas natively compiles workforce artifacts and skills for multi-agent execution:
- **Google Antigravity**: Compiled to `.agents/` (`atlas compile --target antigravity`)

## I Want to Use the Product

- [User Guide](../guides/en/user-guide.md) — Comprehensive user manual, installation, Vim modal mode, splits, and task runner
- [Configuration Reference](../guides/en/config.md) — TOML configuration options, keymaps, mouse, and terminal defaults
- [Themes Guide & Tutorial](../guides/en/themes.md) — Complete guide to creating and customizing color themes with live preview
- [Syntax & Languages](../guides/en/syntax.md) — Language detection, Tree-Sitter support, and Markdown preview

## I Want to Develop / Contribute

- [Product Design & Architecture](design.md) — Core layers, architecture, non-UI invariant, and roadmap phases
- [Plugin API](plugin-api.md) — Plugin manifest specification, hooks, and external executable tools
- [Workspace Polish & Roadmap](polish.md) — Quality standards, performance metrics, and polish checklist

## I Want to Operate / Support It

- Validation suite: `cargo test --workspace`, `cargo clippy --all-targets -- -D warnings`, `cargo fmt --all -- --check`
- Environment & LSP diagnostics: run `:health` inside Oride or execute `atlas doctor`

## I Am an AI Agent

1. Read `AGENTS.md`, `ENTRYPOINT.md`, `atlas.json`, and `PROJECT_STATE.md`.
2. Use Lean Progressive Context (LPC) — smallest sufficient context.
3. Prefer structural/symbol/document-section pointers over raw full-file dumps.
4. Expand only when evidence is insufficient.
5. Keep output bounded and deterministic.
6. Core without UI: never mix Ratatui / crossterm logic into `oride-core` or `oride-config`.
7. Update only impacted canonical docs, keeping documentation and `CHANGELOG.md` synchronized.

## Architecture / Decisions / Specs

- [Architecture & Design Document](design.md)
- [Configuration Specification](../guides/en/config.md)
- [Plugin Architecture](plugin-api.md)
- [Markdown Engine](../guides/en/markdown.md)

## Goals

Goals live under `.ai/goals/<phase>/` and define measurable completion.
