# Syntax Highlighting Engine

Oride utilizes **Tree-Sitter** (along with a custom Markdown rendering pipeline) to highlight active buffers with low latency and high accuracy.

---

## Supported Languages

| LanguageId | Extensions | Grammar / Engine | Tier |
|------------|------------|------------------|------|
| `rust` | `.rs` | `tree-sitter-rust` (static) | **Native Static** |
| `c` | `.c`, `.h` | `tree-sitter-c` (static) | **Native Static** |
| `bash` | `.sh`, `.bash`, `.zsh` | `tree-sitter-bash` (static) | **Native Static** |
| `markdown` | `.md`, … | `tree-sitter-md` + injections (static) | **Native Static** |
| `ori` / ori-lang | `.orl` | built-in lexical fallback | **Lexical Engine** |
| `python` | `.py`, `.pyw` | dynamic (`.so` / plugin) + lexical fallback | **Plugin / Fallback** |
| `javascript` | `.js`, `.mjs`, `.cjs` | dynamic (`.so` / plugin) + lexical fallback | **Plugin / Fallback** |
| `typescript` | `.ts` | dynamic (`.so` / plugin) + lexical fallback | **Plugin / Fallback** |
| `typescriptreact` | `.tsx` | dynamic (`.so` / plugin) + lexical fallback | **Plugin / Fallback** |
| `ruby` | `.rb`, `.rake`, … | dynamic (`.so` / plugin) + lexical fallback | **Plugin / Fallback** |
| `html` | `.html`, `.htm` | dynamic (`.so` / plugin) + lexical fallback | **Plugin / Fallback** |
| `css` | `.css` | dynamic (`.so` / plugin) + lexical fallback | **Plugin / Fallback** |
| `nim` | `.nim`, … | built-in lexical fallback | **Lexical Engine** |
| `d` | `.d`, `.di` | built-in lexical fallback | **Lexical Engine** |
| `lua` | `.lua` | built-in lexical fallback | **Lexical Engine** |
| `oriscript` | `.oris` | `tree-sitter-oriscript` (static legacy) | **Legacy Static** |
| `plain` | others | — | Plain text |

The detected language appears in the status bar. Dynamic grammars can be loaded from `~/.config/oride/grammars/`, `.oride/grammars/`, or installed plugins.

**Markdown Fence Injections:** Code inside triple backticks ` ```lang ` receives syntax highlighting for that specific language. Supported aliases include `rust`/`rs`, `c`/`h`, `bash`/`sh`/`shell`, `python`/`py`, `js`/`javascript`, `ts`/`typescript`, `tsx`, `ruby`/`rb`, `d`/`dlang`, `lua`, `nim`, `html`, `css`, and `oris`/`oriscript`.

---

## How It Works

1. `detect_language(path)` determines the language identifier based on file extensions or shebangs.
2. `HighlightEngine` parses the buffer and generates an abstract syntax tree.
3. AST nodes are queried and mapped to `HighlightKind` tokens, which are rendered using colors defined in `UiTheme.syntax`.
4. Markdown files use a hybrid approach: structural blocks are processed via `tree-sitter-md`, and code blocks are highlighted via nested syntax injection.

---

## Workspace Crates

- `oride-syntax` — Parser engine, token kinds, language detection, and Markdown preview lines.
- `tree-sitter-oriscript` — Vendored legacy OriScript grammar binding (maintained for backward compatibility).
- External grammars via `tree-sitter-*` crates.

---

## Verification

```bash
cargo test -p oride-syntax
```
