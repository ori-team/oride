# Oride

**Oride** (Ori + IDE) is a modular **terminal code editor** focused on
[OriScript](https://github.com/raillen/ori-script), with a navigable project
tree, collapsible embedded terminal, and first-class syntax support for
OriScript, Ori, Markdown, HTML, CSS, JavaScript/TypeScript, Rust, Python, D (dlang),
Lua, Nim, and Ruby.

Status: **`0.2.0`** — mini-IDE TUI contida (editor, tree, terminal, git/SCM com ahead/behind e pull/push, find & replace com globs, multi-LSP sob demanda, MD preview rico com imagens e links, splits dinâmicos, mouse opt-in).
Repo: [ori-team/oride](https://github.com/ori-team/oride).  
Docs: [design](docs/design.md) · [config](docs/config.md) · [markdown](docs/markdown.md) · **[roadmap alpha.6+](docs/planning/alpha6-roadmap.md)**.

## Goals (produto contido)

- Tudo no TUI — **sem** preview HTML/browser, **sem** macros (removidas por anti-bloat), **sem** host de plugins externos
- Multi-tab, tree, terminal PTY, find & replace (buffer + project), git status/SCM, session leve
- **First-class languages:** OriScript, Ori-lang, Markdown, HTML, CSS, JS/TS,
  Rust, Python, D (dlang), Lua, Nim and Ruby — detect, highlight, comment toggle and Markdown
  fence injection ([details](docs/syntax.md))
- Autocomplete local para linguagens first-class + LSP sob demanda para
  **OriScript** (`oriscript lsp`) e **Ori** (`ori-lsp`); outros servidores são configuráveis
- MD preview **no terminal** com tabelas em caixas Unicode, blocos de código com realce sintático e imagens placeholder
- Links no preview → abrir no **navegador do sistema** (clique com mouse ou `Alt+Enter`)
- Mouse **opt-in** (`mouse = false` default) e redimensionamento de divisores por arrasto (drag)

## Build & run

```bash
cargo build --release
./scripts/install.sh                    # → ~/.local/bin/oride
./target/release/oride                  # CWD as workspace + empty buffer
./target/release/oride path/to/file     # open file
./target/release/oride path/to/dir      # open folder as workspace
./target/release/oride --version
```

### Keys (defaults — rebind in TOML)

| Key | Action |
|-----|--------|
| Type / Enter / Backspace / Delete | Edit |
| Arrows, Home, End, PgUp/PgDn | Move |
| `Shift`+arrows/Home/End | Extend selection (multi-line) |
| `Ctrl+Shift+Home` / `End` | Select to doc start/end |
| `Ctrl+A` | Select all |
| `Ctrl+S` | Save |
| `Ctrl+Shift+S` / `F12` / `Alt+Shift+S` | **Save as…** (path browser · Enter salva) |
| `Ctrl+Alt+S` | **Save all** |
| `Ctrl+Z` / `Ctrl+Y` | Undo / redo |
| `Ctrl+N` / `Ctrl+W` | New tab / close tab (2× if dirty) |
| `Ctrl+PgUp` / `Ctrl+PgDn` | Previous / next tab |
| `Alt+←` / `Alt+→` | Previous / next tab |
| (tab bar) | Aba ativa = chip **branco** (fundo sólido) |
| `Ctrl+B` / `Ctrl+E` | Focus tree / editor |
| `Ctrl+O` | **Open folder** (`F2` / `Ctrl+Enter` / `Ctrl+O` confirma) |
| `Ctrl+P` | **Open file** (navigate dirs/files) |
| `Ctrl+"` / `Ctrl+'` / `Ctrl+\`` | Toggle **terminal** (interativo; digite com foco · Esc=editor) |
| `Ctrl+Shift+G` | **SCM panel** (`s` stage · `u` unstage · `c` commit · Enter abre · `d` diff) |
| `Ctrl+Shift+O` | **Buffer picker** (tabs abertas) |
| `Ctrl+Alt+O` / `I` | Jump back / forward |
| `Alt+F/E/V/G/I/H` | **Menu bar** File/Edit/View/Go/Git/Help |
| `Alt+/` | **Which-key** (atalhos essenciais) |
| `F1` / `Ctrl+G` / `Ctrl+Shift+/` | **List all keybindings** (filter · ↑↓ · Esc) |
| `F2` | **Git diff** do arquivo ativo |
| Digitar 2+ caracteres | Sugestões locais automáticas (`↑↓`, `Enter`/`Tab`) |
| `Ctrl+Space` / `Ctrl+K` / `F4` | LSP + fallback complete / hover / goto |
| `Ctrl+Shift+I` / `Ctrl+Shift+M` | LSP format / diagnostics panel |
| `Alt+=` / `Alt+-` | Terminal taller / shorter |
| `Ctrl+R` | Reload file from disk |
| `Ctrl+Shift+F` | **Find & Replace in project** (Tab troca campo · Enter substitui tudo) |
| `Ctrl+Shift+V` / `Alt+P` | **Markdown preview** (segue scroll · Enter/Alt+Enter abre link) |
| `Ctrl+Alt+V` / `H` | Split editor vertical / horizontal |
| `F6` / `Ctrl+Alt+W` | Next pane / close pane |
| `Ctrl+Alt+↑/↓` / `U` | Multi-cursor add / clear |
| Find `Alt+R` | Toggle regex search (buffer e projeto) |
| `Ctrl+F` / `F3` | Find mini-modal / next |
| `Ctrl+H` | Replace (mesmo modal; Tab troca campo) |
| `Alt+C` | Toggle **case sensitive** (no find) |
| `Alt+A` | Toggle **ignorar acentos** (á≈a; no find) |
| `Alt+W` | Toggle **palavra completa** (UI ≠ GUI) |
| `Alt+R` | Toggle regex (buffer e projeto) |
| `Alt+Enter` / `Ctrl+Alt+Enter` | Replace one / replace all |
| `Ctrl+C` / `V` / `X` | Copy / paste / cut |
| `Alt+Z` | Soft wrap |
| `Ctrl+/` | Toggle comment |
| `Esc` or `Ctrl+Q` | Close overlay / quit |

**Browser (`Ctrl+O` / `Ctrl+P` / Save as):** linha ciano = seleção · `↑↓` · `Enter` entra/abre (save as: **Enter salva**) · `F2`/`Ctrl+O` confirma pasta · digite filtra/nome.

**Tree (focused):** `↑↓`/`jk` · `Enter` open/expand · `r` renomear · `d` excluir · `y`/`c` copiar caminho · `←→`/`hl` · `Space` toggle · `Tab`/`Esc` → editor.  
**Terminal:** shell interativo com foco no painel · `Ctrl+C` vai pro shell · `Esc` → editor · `Ctrl+"` fecha.  
**SCM:** lista working tree dirty (`s` stage · `u` unstage · `c` commit prompt · `d` diff).  
**Icons:** Nerd Font glyphs (ASCII fallback exists in code).  
**Mouse (default off):** ativar com `mouse = true` no TOML ou **View → Enable / disable mouse** (palette). Com on: clique = caret · drag = seleção · drag em divisores = redimensiona árvore/splits · duplo = palavra · scroll por painel.

### Extras úteis

| Key | Action |
|-----|--------|
| `F8` | Surround seleção com par `()[]{}…` |
| `Ctrl+Shift+T` | Multi-picker (buffers + cmds + files) |
| `Ctrl+Shift+U` | Histórico de undo |
| View → Enable mouse | Liga captura de mouse (ou `mouse = true`) |

Macros de teclado foram removidas em conformidade com o princípio de produto enxuto (anti-bloat).

### Config

```bash
mkdir -p ~/.config/oride
cp assets/config.example.toml ~/.config/oride/config.toml
# project overlay:
mkdir -p .oride && cp assets/config.example.toml .oride/config.toml
```

See [`docs/config.md`](docs/config.md).

```bash
cargo test --workspace
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
```

## Workspace layout

```text
crates/
  oride-core/     # rope buffer, documents/tabs, undo
  oride-config/   # TOML load/merge
  oride-keymap/   # chords → actions
  oride-fs/       # project tree, create file/dir, icons
  oride-git/      # git status porcelain for tree badges
  oride-terminal/ # embedded PTY panel
  oride-syntax/   # tree-sitter highlight engine
  tree-sitter-oriscript/  # vendored OriScript grammar
  oride-ui/       # ratatui widgets
  oride-app/      # composition + event loop
  oride/          # binary CLI
docs/
  design.md       # architecture & roadmap
  config.md       # TOML reference
```

## Relation to OriScript

Oride is a **separate repository**. It does not vendor the OriScript compiler.
Language intelligence uses the `oriscript` CLI / LSP on `PATH`.

## License

MIT — see [LICENSE](LICENSE).
