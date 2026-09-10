# Project Atlas — Oride Terminal IDE

This is the intent router for humans and agents. Add links as stable documentation is created; do not create empty documentation solely to populate this map.

## Current state & framework

- [Project state](../PROJECT_STATE.md) — active phase, goal status, recent actions
- [`atlas.json`](../atlas.json) — canonical project configuration (Project Atlas v0.4.2, Protocol v3)
- [`AGENTS.md`](../AGENTS.md) — product invariants, core design rules, and validation commands

## AI harnesses & compilation

Project Atlas natively compiles workforce artifacts and skills for multi-agent execution:
- **Google Antigravity**: compiled to `.agents/` (`atlas compile --target antigravity`)

## I want to use the product

- [User Guide (Português)](guides/pt/guia-de-uso.md) — Manual do usuário, instalação, modo modal Vim, splits e task runner
- [User Guide (English)](guides/en/user-guide.md) — Complete user manual in English
- [Configuration Reference](guides/pt/config.md) — TOML configuration options, keymaps, mouse, and terminal defaults
- [Themes Guide & Tutorial](guides/pt/themes.md) — Complete guide to creating and customizing color themes with live preview
- [Syntax & Languages](guides/pt/syntax.md) — Language detection, Tree-Sitter support, and Markdown preview

## I want to develop/contribute

- [Product Design & Architecture](design.md) — core layers, architecture, non-UI invariant, and roadmap phases
- [Plugin API](plugin-api.md) — plugin manifest specification, hooks, and external executable tools
- [Workspace Polish & Roadmap](polish.md) — quality standards, performance metrics, and polish checklist

## I want to operate/support it

- Validation suite: `cargo test --workspace`, `cargo clippy --all-targets -- -D warnings`, `cargo fmt --all -- --check`
- Environment & LSP diagnostics: run `:health` inside Oride or execute `atlas doctor`

## I am an AI agent

1. Read `AGENTS.md`, `ENTRYPOINT.md`, `atlas.json`, and `PROJECT_STATE.md`.
2. Use Lean Progressive Context (LPC) — smallest sufficient context.
3. Prefer structural/symbol/document-section pointers over raw full-file dumps.
4. Expand only when evidence is insufficient.
5. Keep output bounded and deterministic.
6. Core without UI: never mix Ratatui / crossterm logic into `oride-core` or `oride-config`.
7. Update only impacted canonical docs, keeping documentation and `CHANGELOG.md` synchronized.

## Architecture / decisions / specs

- [Design Document](design.md)
- [Configuration Specification](guides/pt/config.md)
- [Plugin Architecture](plugin-api.md)

## Goals

Goals live under `.ai/goals/<phase>/` and define measurable completion.

## Durable intelligence

Compact project/task intelligence lives in `.atlas/history/project-intelligence.json`.
