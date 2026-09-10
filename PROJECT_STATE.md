# Current Project State

- Project: **Oride Terminal IDE**
- Framework: **Project Atlas v0.4.2**
- Current phase: **P04 — Polimento 0.1**
- Current goal: **Modal Editing, Task Runner & Diagnostics**
- Context methodology: **Lean Progressive Context (LPC)**
- Last updated: `2026-09-10T18:52:00Z`

## Recent Milestones

- ✅ Implemented dynamic external i18n via TOML catalogs (`pt-BR.toml`, `en-US.toml`, user config dirs).
- ✅ Published theme development tutorial and specification in `docs/themes.md`.
- ✅ Implemented Vim-style modal editing engine (`Normal`, `Insert`, `Visual`, `VisualLine`, `:` command bar).
- ✅ Built integrated task runner (`tasks.toml`) with magic variable substitution and PTY execution.
- ✅ Added proactive system & LSP environment diagnostics modal (`:health`).
- ✅ 100% tests passing in workspace with 0 clippy warnings.

## Next action

- Continue polish toward v0.1.0 release.
- Maintain Atlas Framework governance and goal tracking.

## Recovery order

1. `AGENTS.md` (Product rules and invariants).
2. `ENTRYPOINT.md` or the platform adapter (`.agents/`).
3. `atlas.json`.
4. `PROJECT_STATE.md`.
5. `docs/ATLAS.md`.
6. Relevant canonical docs in `docs/` and crates under `crates/`.

Do not load the entire repository by default.
