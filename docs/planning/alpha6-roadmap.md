# Oride — plano de implementação `0.2.0` e além

**Status:** normativo a partir de 2026-07-13  
**Release atual definida:** **`0.2.0`**  
**Precedência:** este doc > `post-0.1-roadmap.md` (histórico) > notas soltas em `design.md`  
**Produto:** TUI IDE **contida** (tudo no processo Oride + terminal do usuário). Sem bloat.

---

## 1. Princípios (anti-bloat)

| # | Princípio | Implicação |
|---|-----------|------------|
| P1 | **Contido no TUI** | Features vivem em ratatui/crossterm/PTY; não spawna browser de preview MD |
| P2 | **Um conceito por PR** | Fatias pequenas; sem “framework de plugins externos” cedo |
| P3 | **First-class languages finitas** | Só a lista §3; resto = Plain ou highlight genérico depois |
| P4 | **Git via CLI** | Sem libgit2 até dor real; porcelain + blame + diff já existem |
| P5 | **Fail closed / status line** | LSP/git/PTY falham com mensagem, não crash |
| P6 | **Docs no mesmo slice** | Spec/README/CHANGELOG junto da feature user-facing |
| P7 | **Preferir maturar o que existe** | Antes de novo painel, polish do fluxo já shipped |

### Explicitamente **fora de escopo** (não implementar)

- Macros (record/play) — **remover** se ainda no código; não expandir  
- Preview Markdown **HTML/browser** / Typora-like externo  
- Host de plugins Lua/WASM/dynload  
- Full vim modal default  
- Undo tree ramificado visual  
- Telescope multi-source monstro  
- DAP/debugger, collab, cloud  
- Inlay hints densos (salvo se LSP OriScript entregar e for 1 toggle simples)  
- Replace-in-project “IDE monstro” (só se caber em fatia mínima depois de languages)

---

## 2. Baseline `0.1.0-alpha.6` (o que já conta como feito)

Congela e documenta o estado **já no tree** sob a versão **0.1.0-alpha.6**:

| Área | Estado |
|------|--------|
| Core editor | rope, tabs, undo/redo, seleção, soft wrap, comment toggle |
| Árvore | expand, create file/dir, git badges |
| Terminal | PTY interativo, resize, foco, Ctrl no shell |
| Find | buffer (case/accent/word/regex) + project (`rg`/fallback) |
| Git | status tree, SCM panel, blame status, diff read-only |
| MD | highlight + fence inject + preview TUI + image **placeholder** |
| LSP | OriScript (diagnostics, complete, hover, goto, format) |
| Layout | menu, banner, splits (2), multi-cursor, mouse **opt-in** (`mouse=false`) |
| UX | which-key, welcome, buffer picker, jump list, multi-picker MVP, surround MVP |
| Plugins | built-in `LanguageProvider` + 2 commands (sem host externo) |

**Gate de release alpha.6:** `cargo fmt` · `clippy -D warnings` · `cargo test --workspace` · version bump · CHANGELOG com seção **0.1.0-alpha.6** · docs sync (este plano).

---

## 3. Linguagens first-class (alvo)

Ordem de maturidade por linguagem: **detect → highlight → comment/indent → (opcional) LSP**.

| ID | Extensões típicas | Highlight | LanguageProvider | LSP no Oride |
|----|-------------------|-----------|------------------|--------------|
| **OriScript** | `.oris` | tree-sitter (já) | já | `oriscript lsp` (já) |
| **Ori (ori-lang)** | `.orl` | fallback léxico contido (grammar estável indisponível) | sim | se CLI/LSP existir no PATH; senão skip |
| **Markdown** | `.md`, … | já (+ inject) | já | não |
| **HTML** | `.html`, `.htm` | já | já | não no alpha |
| **CSS** | `.css` | já | já | não no alpha |
| **JavaScript** | `.js`, `.mjs`, `.cjs`, `.jsx` | já | já | não no alpha |
| **TypeScript** | `.ts`, `.tsx` | tree-sitter-typescript/TSX | sim | opcional `typescript-language-server` depois |
| **Rust** | `.rs` | tree-sitter-rust | sim | opcional `rust-analyzer` depois |
| **Python** | `.py` | tree-sitter-python | sim | opcional `pylsp`/`pyright` depois |
| **Nim** | `.nim` | fallback léxico contido (sem crate estável) | sim | opcional |
| **Ruby** | `.rb` | tree-sitter-ruby | sim | opcional |

**Regra de contensão:** no ciclo alpha.6→0.2 só **highlight + provider + fence inject**. Multi-LSP genérico = fatia própria (L2), não bloqueia languages.

**Fence inject MD:** aliases para todas as langs first-class (` ```rust `, ` ```python `, ` ```nim `, ` ```ruby `, ` ```oris `, ` ```orl `, …).

---

## 4. Roadmap por fatias (DAG)

```text
R0  Release hygiene alpha.6 ─────────────────────────────┐
R1  Remove macros / anti-bloat cleanup ──────────────────┤
                                                          │
L1  Languages matrix (highlight+provider+fences) ────────┼─► L2 LSP multi (opt-in, 1 server/config)
                                                          │
M1  MD links → system browser (in-TUI hit + open) ───────┤
M2  MD images via terminal graphics (Kitty/Sixel/iterm) ─┤
                                                          │
E1  Editor polish (session layout, replace-project min) ─┤
G1  Git mínimo (stage+commit CLI) ───────────────────────┤
                                                          ▼
                    0.2.0 “languages + MD media + hygiene”
```

### R0 — Release `0.1.0-alpha.6` (este slice de docs/versão)

| Entrega | Gate |
|---------|------|
| `workspace.package.version = 0.1.0-alpha.6` | Cargo |
| CHANGELOG: mover Unreleased → `## 0.1.0-alpha.6` | leitura |
| README status line = alpha.6 | ok |
| Este plano + pointer nos docs antigos | ok |
| `cargo test --workspace` + clippy | CI local |

### R1 — Anti-bloat / higiene

**Status:** ✅ concluído.

| ID | Entrega | Não fazer |
|----|---------|-----------|
| **R1.1** | ✅ Remover actions/UI de **macro** (F9/F10, menu, keymap, estado) | “melhorar macros” |
| **R1.2** | ✅ Remover menções a preview HTML/browser do docs | implementar browser |
| **R1.3** | ✅ Marcar multi-picker/surround como estáveis MVP (sem expandir) | telescope monstro |
| **R1.4** | ✅ Changelog + help keybinds sem macros | — |

### L1 — Languages first-class (prioridade alta)

**Status:** ✅ concluído em Unreleased.

| ID | Entrega | Gate |
|----|---------|------|
| **L1.0** | ✅ `LanguageId` + `detect_language` para rust/python/ts/nim/ruby/ori-lang | testes path |
| **L1.1** | ✅ Deps tree-sitter oficiais para rust, python, typescript e ruby; fallback contido para nim/ori | compile size sanity |
| **L1.2** | ✅ Queries/fallback highlight (keyword/string/comment/function) por lang | asserts spans |
| **L1.3** | ✅ `LanguageProvider` + comment syntax + soft_wrap default | toggle comment em fixture |
| **L1.4** | ✅ Fence inject aliases MD para todas | teste inject |
| **L1.5** | ✅ Docs `syntax.md` + README tabela langs | living-docs |

**Ordem de implementação sugerida (custo/benefício):**  
Rust → Python → TypeScript → Ruby → Nim → Ori-lang (grammar do monorepo).

**Δ binário:** tree-sitter grammars aumentam o binário; aceitar strip release; não embutir 20 langs extras.

### L2 — LSP (depois de L1; contido)

**Status:** ✅ concluído — configuração multi-server, clients preguiçosos sob demanda por linguagem aberta, completion/hover/definition/format unificados com fail-closed.

| ID | Entrega | Gate |
|----|---------|------|
| **L2.1** | ✅ Config `[lsp.servers]` map lang → argv (defaults OriScript + Ori + L1) | TOML |
| **L2.2** | ✅ N clients preguiçosos por linguagem aberta | sem crash se offline |
| **L2.3** | ✅ Unificação completa: completion + sync + matriz diagnostics/hover/goto/format | smoke Ori completion |
| **L2.4** | ✅ Não obrigar servidores externos no CI (skip se binário ausente com fail-closed) | skip se binário ausente |

### M1 — Links clicáveis no preview MD (in-TUI)

**Status:** ✅ concluído.

| ID | Entrega | Gate |
|----|---------|------|
| **M1.1** | ✅ Preview guarda spans de link com URL + rect por linha | unit |
| **M1.2** | ✅ Clique (mouse on) no preview → `xdg-open` / `open` / `cmd start` na URL ou path | manual |
| **M1.3** | ✅ Teclado: Enter com caret na linha do link (ou atalho `Alt+Enter`) no preview | status |
| **M1.4** | ✅ Só `http(s):`, `mailto:`, paths relativos seguros (sem shell injection) | testes URL |

**Não** é preview no browser do MD; só **abre o alvo do link** no sistema.

### M2 — Imagens no terminal (opcional, best-effort)

**Status:** ✅ concluído.

| ID | Entrega | Gate |
|----|---------|------|
| **M2.1** | ✅ Detectar capability: Kitty graphics / iTerm2 inline / Sixel (com inspeção pura de PNG/JPEG/GIF/WebP/SVG) | feature detect |
| **M2.2** | ✅ No preview, se local file image + capability: render inline/card enriquecido com dimensões e peso | manual Kitty |
| **M2.3** | ✅ Fallback: card placeholder legível com dimensões e status do protocolo | não regredir |
| **M2.4** | ✅ Config `markdown.terminal_images = true` default **false** até estável | TOML |
| **M2.5** | ✅ Docs: documentado em `docs/config.md` e `assets/config.example.toml` | ok |

**Não** abrir viewer externo de imagem como feature principal (pode ser ação secundária “Open externally” no mesmo card se trivial).

### E1 — Editor polish contido

**Status:** ✅ concluído.

| ID | Entrega | Gate |
|----|---------|------|
| **E1.1** | ✅ Session: restaurar scroll_y + split panes + secondary doc + show_tree/scm e tree_width | roundtrip |
| **E1.2** | ✅ Project find: glob opcional simples (`*.rs`, `!target/**`) com atalho `Alt+G` | teste |
| **E1.3** | ✅ Replace-in-project **mínimo** (lista hits → replace all com Tab/Enter) | fail closed |
| **E1.4** | ✅ Syntax colors from TOML (map HighlightKind → cor em `SyntaxColorsConfig`) | visual |

### G1 — Git mínimo (CLI)

**Status:** ✅ concluído.

| ID | Entrega | Gate |
|----|---------|------|
| **G1.1** | ✅ SCM: `s` stage path, `u` unstage | status refresh |
| **G1.2** | ✅ Commit message prompt → `git commit -m` | dirty tree clean |
| **G1.3** | ✅ Push/pull sob demanda (`p` pull, `P` push no SCM, palette, menu) + ahead/behind na status bar | status feedback |

---

## 5. Critérios de “first-class” (Definition of Done por linguagem)

Uma linguagem L está **first-class** quando:

1. `detect_language` estável por extensão  
2. Highlight não-vazio em fixture mínima  
3. Toggle comment correto  
4. Aparece em fence inject MD  
5. Listada em README/syntax.md  
6. (Opcional L2) LSP documentado em config, não obrigatório  

---

## 6. SemVer (produto Oride)

| Versão | Conteúdo |
|--------|----------|
| **0.1.0-alpha.6** | Congela baseline anterior + hygiene R0/R1 início |
| **0.2.0** | ✅ L1 completo (D, Lua, Rust, Python, etc.) + M1 (links) + M2 (imagens) + E1.1–E1.2 + G1 completo + R1 anti-bloat + L2 multi-LSP |
| **0.3.0** | Próximas expansões e maturação contínua |
| **≥0.4 / 1.0** | só com API estável e suite de regressão |

**Não** pular para 1.0 enquanto grammars/LSP ainda “best effort”.

---

## 7. Skills / validação por fatia

- Sempre: `clean-code`, `rust`, `living-docs`  
- Linguagens/highlight: + disciplina de `compiler-dev` leve (tests + CHANGELOG)  
- Gate:

```bash
cd /path/to/oride
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

Release build size (observar, não bloquear cedo):

```bash
cargo build --release -p oride && ls -lh target/release/oride
```

---

## 8. Ordem de execução recomendada (um dev)

1. **R0** version + CHANGELOG + sync docs ← **agora**  
2. **R1.1** remover macros  
3. **L1** Rust → Python → TS → Ruby → Nim → Ori-lang ✅
4. **M1** links clicáveis  
5. **M2** Kitty images (flag off default)  
6. **E1.1** session layout  
7. **G1** se ainda couber no 0.2  

---

## 9. Riscos

| Risco | Mitigação |
|-------|-----------|
| Binário cresce com grammars | Só langs da lista; strip release; sem grammars “por precaução” |
| tree-sitter-nim indisponível | Fallback léxico simples, isolado e coberto por testes |
| ori-lang grammar fora do repo | Fallback léxico simples; substituir quando houver grammar estável |
| Protocolos de imagem divergentes | Um backend (Kitty); fallback placeholder |
| Multi-LSP complexidade | L2 só após L1; um server por vez no MVP |

---

## 10. Checklist de sync de docs (R0)

- [x] `docs/planning/alpha6-roadmap.md` (este arquivo)  
- [x] `Cargo.toml` version alpha.6  
- [x] `CHANGELOG.md` seção 0.1.0-alpha.6  
- [x] `README.md` status  
- [x] `docs/planning/post-0.1-roadmap.md` → pointer “superseded”  
- [x] `docs/planning/ux-polish-plan.md` status atual  
- [x] `docs/markdown.md` futuro alinhado (sem browser preview)  
- [x] `docs/syntax.md` tabela langs alvo  
- [x] `docs/plugin-api.md` nota anti-bloat  

---

_Gerado para o ciclo alpha.6; atualizar checkboxes conforme PRs fecharem._
