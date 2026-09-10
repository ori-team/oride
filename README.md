# Oride

**English** · [Português](README.pt-BR.md)

**Oride** (Ori + IDE) is a modular, lightweight, and extensible **terminal code editor / mini-IDE** built in Rust. It features a navigable project tree, collapsible embedded terminal, Vim-style modal editing, integrated declarative task runner (`tasks.toml`), environment health diagnostics (`:health`), and syntax highlighting for multiple languages (Rust, C, Bash, Markdown, Ori, HTML, CSS, JavaScript/TypeScript, Python, D, Lua, Nim, and Ruby).

Status: **`0.2.0`** — self-contained TUI mini-IDE (editor, project file tree, PTY terminal, Git/SCM with ahead/behind and pull/push, project search & replace with globs/regex, on-demand multi-LSP, rich Markdown preview with images and links, dynamic splits, opt-in mouse support).  
Repository: [ori-team/oride](https://github.com/ori-team/oride).  
Documentation: [User Guide](docs/en/user-guide.md) · [Architecture & Design](docs/en/design.md) · [Configuration](docs/en/config.md) · [Themes](docs/en/themes.md) · [Roadmap](docs/en/planning/alpha6-roadmap.md).

## Goals (Lean & Contained Product)

- **Everything in the TUI:** Zero browser preview dependency, no slow scripting bloat, no heavy macro engines.
- **Full-featured Layout:** Multiple buffer tabs, project tree with Nerd Font glyphs, interactive PTY terminal, buffer & project-wide find and replace, Git/SCM status, lightweight workspace sessions.
- **Supported Languages:** Rust, C, Bash, Markdown, Ori, HTML, CSS, JS/TS, Python, D (dlang), Lua, Nim, and Ruby — auto-detection, syntax highlight, comment toggling, and Markdown fence injections ([details](docs/en/syntax.md)).
- **On-demand LSP:** Local intelligent autocomplete + language servers spawned on-demand for **Ori** (`ori-lsp`), Rust (`rust-analyzer`), C/C++ (`clangd`), Bash (`bash-language-server`), etc. (fully configurable in `config.toml`).
- **Rich Terminal Markdown Preview:** Unicode box-drawing tables, fenced code blocks with syntax highlighting, and terminal graphics protocol rendering on supported terminals (Kitty/Ghostty/WezTerm).
- **Preview Links:** Open links in the system web browser via mouse click or `Alt+Enter`.
- **Opt-in Mouse:** Disabled by default (`mouse = false`); when enabled, supports caret positioning, text selection by dragging, and split divider resizing.

## Build & Run

```bash
cargo build --release
./scripts/install.sh                    # Installs to ~/.local/bin/oride
./target/release/oride                  # CWD as workspace + empty buffer
./target/release/oride path/to/file     # Open specific file
./target/release/oride path/to/dir      # Open directory as workspace
./target/release/oride --version
```

### Essential Keybindings (Rebindable via TOML)

| Key | Action |
|---|---|
| Typing / Enter / Backspace / Delete | Direct text editing |
| Arrow keys, Home, End, PgUp, PgDn | Cursor navigation |
| `Shift` + Arrow keys / Home / End | Extend selection |
| `Ctrl+Shift+Home` / `End` | Select to document start / end |
| `Ctrl+A` | Select all |
| `Ctrl+S` | Save active file |
| `Ctrl+Shift+S` / `F12` / `Alt+Shift+S` | **Save as…** (integrated file browser) |
| `Ctrl+Alt+S` | Save all open buffers |
| `Ctrl+Z` / `Ctrl+Y` | Undo / Redo |
| `Ctrl+N` / `Ctrl+W` | New tab / Close tab |
| `Ctrl+PgUp` / `Ctrl+PgDn` / `Alt+←` / `Alt+→` | Previous tab / Next tab |
| `Ctrl+B` / `Ctrl+E` | Focus project tree / Focus editor |
| `Ctrl+O` | **Open folder** as workspace (`F2` confirms) |
| `Ctrl+P` | **Open file** |
| `Ctrl+"` / `Ctrl+'` / `Ctrl+\`` | Toggle **embedded PTY terminal** |
| `Ctrl+Shift+G` | **SCM / Git panel** (`s` stage · `u` unstage · `c` commit · `d` diff) |
| `Ctrl+Shift+O` | **Buffer picker** (switch open tabs) |
| `Alt+F/E/V/G/I/H` | Menu bar (*File*, *Edit*, *View*, *Go*, *Git*, *Help*) |
| `Alt+/` | Which-key (essential shortcuts reminder) |
| `F1` / `Ctrl+G` / `Ctrl+Shift+/` | List all keybindings with search filter |
| `F2` | Git diff for active file |
| Type 2+ characters | Automatic local word autocomplete |
| `Ctrl+Space` / `Ctrl+K` / `F4` | LSP autocomplete / hover info / goto definition |
| `Ctrl+Shift+I` / `Ctrl+Shift+M` | LSP format document / diagnostics panel |
| `Alt+=` / `Alt+-` | Increase / decrease terminal panel height |
| `Ctrl+R` | Reload file from disk |
| `Ctrl+Shift+F` | **Find & Replace in Project** (recursive fast search) |
| `Ctrl+Shift+V` / `Alt+P` | Real-time **Markdown preview** |
| `Ctrl+Alt+V` / `Ctrl+Alt+H` | Split editor vertically / horizontally |
| `F6` / `Ctrl+Alt+W` | Cycle next split pane / Close pane |
| `Ctrl+Alt+↑/↓` / `U` | Add multi-cursor / Clear multi-cursors |
| `Ctrl+F` / `F3` | Find in buffer / Next occurrence |
| `Ctrl+H` | Replace in buffer |
| `Alt+C` / `Alt+A` / `Alt+W` / `Alt+R` | Toggle Case Sensitive / Ignore Accents / Whole Word / Regex |
| `Alt+Enter` / `Ctrl+Alt+Enter` | Replace current match / Replace all matches |
| `Ctrl+C` / `Ctrl+V` / `Ctrl+X` | Copy / Paste / Cut |
| `Alt+Z` | Toggle soft word wrap |
| `Ctrl+/` | Toggle line comment |
| `Esc` or `Ctrl+Q` | Close overlay / Quit |

### Navigation & Panels
- **File Browser (`Ctrl+O` / `Ctrl+P`):** Cyan highlight indicates selection · `↑↓` navigate · `Enter` opens/enters · typing filters items.
- **Project Tree:** `↑↓` or `jk` navigate · `Enter` opens file or expands folder · `r` rename · `d` delete · `y`/`c` copy path · `Tab`/`Esc` returns focus to editor.
- **PTY Terminal:** Full interactive shell; `Ctrl+C` sends interrupt to shell when terminal is focused; `Esc` returns to editor.
- **Vim Modal Mode:** Type `:normal` in command prompt to enable modal navigation (`h`, `j`, `k`, `l`, `w`, `b`, `gg`, `G`, `x`, `u`, `:w`, `:q`, `:tasks`, `:health`).

### Configuration

```bash
mkdir -p ~/.config/oride
cp assets/config.example.toml ~/.config/oride/config.toml

# Local project configuration:
mkdir -p .oride && cp assets/config.example.toml .oride/config.toml
```

See [`docs/en/config.md`](docs/en/config.md) for full configuration options.

## Workspace Layout

```text
crates/
  oride-core/     # rope buffers, document tabs, undo history
  oride-config/   # TOML loading & layered merging
  oride-keymap/   # chord mapping & action dispatch
  oride-fs/       # project tree, file management, icons
  oride-git/      # git status porcelain integration
  oride-terminal/ # embedded PTY panel
  oride-syntax/   # Tree-Sitter highlighting & lexical engine
  tree-sitter-oriscript/  # vendored legacy grammar
  oride-ui/       # Ratatui widgets & rendering
  oride-app/      # application orchestration & event loop
  oride/          # CLI binary
docs/
  en/             # canonical English documentation
  guia-de-uso.md  # Portuguese user guide
  design.md       # architecture & design
  config.md       # configuration reference
  themes.md       # theme development guide
```

## License

MIT — see [LICENSE](LICENSE).
