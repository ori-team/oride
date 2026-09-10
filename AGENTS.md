# AGENTS.md — Oride

Guia para agentes que implementam o editor TUI **Oride**.

**Precedência:** este arquivo > skills Grok > defaults do modelo.

## Produto

| Conceito | Nome |
|----------|------|
| Editor | **Oride** |
| CLI | `oride` |
| Crates | `oride-*` |
| Config | TOML (`~/.config/oride/`, `.oride/`) |
| Design | `docs/design.md` |
| Versão | SemVer a partir de `0.1.0-alpha.x` |

## Skills (obrigatórias neste repo)

| Skill | Quando |
|-------|--------|
| **`rust`** | Todo código (workspace, `Result`, clippy/fmt) |
| **`clean-code`** | Módulos, nomes, KISS, anti-primitivo |
| **`living-docs`** | README / `docs/` / comportamento user-facing |

**Nota:** Inteligência de linguagem vem via **LSP no PATH** (`rust-analyzer`, `clangd`, `ori-lsp`, etc.), mantendo o editor desacoplado de compiladores específicos.

Não misturar lógica de UI (`ratatui`) em `oride-core` / `oride-config` / `oride-keymap`.

## Invariantes

1. **Core sem UI** — `oride-core` testável sem TTY.
2. **Actions como dados** — keymaps e palette disparam ações nomeadas.
3. **Fail closed** — LSP/Git/PTY falham com status, não crash.
4. **LSP via PATH** — spawn desacoplado via binários no PATH, sem dependência estática de compiladores.
5. **Um conceito por PR** — seguir fases em `docs/design.md`.

## Validação

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --workspace
```

## Fases (resumo)

| Fase | Foco |
|------|------|
| P0 | core + TUI + config/keymap/theme |
| P1 | tabs, árvore, terminal, palette (alpha.2) |
| P2 | tree-sitter + Markdown rico (alpha.3) |
| P3 | LSP engine & tooling |
| P4 | **polimento 0.1** (help, find, clipboard, session, …) |

Markdown futuro (preview, fence injections, MDX JSX): `docs/markdown.md` — **não** misturar no P3/P4.
