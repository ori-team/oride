# Syntax highlight

Oride usa **tree-sitter** (e pipeline MD próprio) para colorir o buffer ativo.

## Linguagens first-class

**Normativo:** [`docs/planning/alpha6-roadmap.md`](planning/alpha6-roadmap.md) §3.

| LanguageId | Extensões | Grammar / motor | Estado |
|------------|-----------|-----------------|------------------|
| `oriscript` | `.oris` | `tree-sitter-oriscript` (estático vendored) | **nativo estático** |
| `rust` | `.rs` | `tree-sitter-rust` (estático) | **nativo estático** |
| `c` | `.c`, `.h` | `tree-sitter-c` (estático) | **nativo estático** |
| `bash` | `.sh`, `.bash`, `.zsh` | `tree-sitter-bash` (estático) | **nativo estático** |
| `markdown` | `.md`, … | `tree-sitter-md` + inject (estático) | **nativo estático** |
| `python` | `.py`, `.pyw` | dinâmico (`.so` / plugin) + fallback léxico | **plugin / fallback** |
| `javascript` | `.js`, `.mjs`, `.cjs` | dinâmico (`.so` / plugin) + fallback léxico | **plugin / fallback** |
| `typescript` | `.ts` | dinâmico (`.so` / plugin) + fallback léxico | **plugin / fallback** |
| `typescriptreact` | `.tsx` | dinâmico (`.so` / plugin) + fallback léxico | **plugin / fallback** |
| `ruby` | `.rb`, `.rake`, … | dinâmico (`.so` / plugin) + fallback léxico | **plugin / fallback** |
| `html` | `.html`, `.htm` | dinâmico (`.so` / plugin) + fallback léxico | **plugin / fallback** |
| `css` | `.css` | dinâmico (`.so` / plugin) + fallback léxico | **plugin / fallback** |
| `ori` / ori-lang | `.orl` | fallback léxico contido | **lexical** |
| `nim` | `.nim`, … | fallback léxico contido | **lexical** |
| `d` | `.d`, `.di` | fallback léxico contido | **lexical** |
| `lua` | `.lua` | fallback léxico contido | **lexical** |
| `plain` | outras | — | sem highlight |

A linguagem ativa aparece na status line. Gramáticas dinâmicas são carregadas de `~/.config/oride/grammars/`, `.oride/grammars/` ou plugins instalados.

**Fence inject (MD):** o conteúdo de ` ```lang ` recebe highlight da linguagem.
Aliases disponíveis incluem `oris`/`oriscript`, `rust`/`rs`, `c`/`h`, `bash`/`sh`/`shell`, `python`/`py`, `js`/`javascript`, `ts`/`typescript`, `tsx`, `ruby`/`rb`, `d`/`dlang`, `lua`, `nim`, `html` e `css`.

## Como funciona

1. `detect_language(path)` escolhe o id.
2. `HighlightEngine` reparseia quando o texto muda.
3. Nós do AST → `HighlightKind` → cores em `UiTheme.syntax`.
4. Markdown blocks usam grammar MD; inlines e injects complementam.

## Crates

- `oride-syntax` — engine + kinds + detecção + MD preview lines
- `tree-sitter-oriscript` — binding da grammar OriScript
- grammars externas via crates `tree-sitter-*`

## Limitações

- Reparse completo por edit (não incremental) — ok para arquivos médios
- Cores de syntax no TOML: parcial (E1.4 no roadmap)
- Nim e Ori têm highlight léxico de keywords, tipos, funções, strings, números e
  comentários; recursos sintáticos mais profundos dependem de grammars estáveis
- Semantic tokens / multi-LSP: L2 no roadmap (opt-in)

## Validação

```bash
cargo test -p oride-syntax
```
