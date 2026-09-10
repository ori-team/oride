# Project Atlas entrypoint

1. Read `atlas.json`, `PROJECT_STATE.md` and `docs/ATLAS.md`.
2. Read the active Goal and its dependencies.
3. Start with the minimum sufficient context; do not read the entire repository.
4. Prefer Context Packs/task maps, document sections, symbols and related tests.
5. Expand context only when evidence is insufficient; delegation depth is bounded by `atlas.json`.
6. Never weaken acceptance criteria silently.
7. Keep code, tests and canonical docs synchronized through a Documentation Delta.
8. Keep intermediate output compact and do not persist task-specific context files.
9. Before completion, record evidence/project intelligence and remove temporary context.
