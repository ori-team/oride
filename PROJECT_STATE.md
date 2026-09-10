# Current Project State

- Project: **Oride Terminal IDE**
- Framework: **Project Atlas v0.4.2**
- Current phase: **P04 — Polimento 0.2**
- Current goal: **Modal Editing, Task Runner & Diagnostics**
- Context methodology: **Lean Progressive Context (LPC)**
- Last updated: `2026-09-10T20:20:00Z`

## Recent Milestones

- ✅ Implemented dynamic external i18n via TOML catalogs (`pt-BR.toml`, `en-US.toml`, user config dirs).
- ✅ Published theme development tutorial and specification in `docs/guides/pt/themes.md` and `docs/guides/en/themes.md`.
- ✅ Implemented Vim-style modal editing engine (`Normal`, `Insert`, `Visual`, `VisualLine`, `:` command bar).
- ✅ Built integrated task runner (`tasks.toml`) with magic variable substitution and PTY execution.
- ✅ Added proactive system & LSP environment diagnostics modal (`:health`).
- ✅ 100% tests passing in workspace with 0 clippy warnings.

## Next action

- Continue polish and stabilization for v0.2.0 release.
- Maintain Atlas Framework governance and goal tracking.

## Recovery order

1. `AGENTS.md` (Product rules and invariants).
2. `ENTRYPOINT.md` or the platform adapter (`.agents/`).
3. `atlas.json`.
4. `PROJECT_STATE.md`.
5. `docs/ATLAS.md`.
6. Relevant canonical docs in `docs/` and crates under `crates/`.

Do not load the entire repository by default.
