# Polish Milestones & Feature History

Polish history and milestone tracking. Active roadmap: [`planning/alpha6-roadmap.md`](planning/alpha6-roadmap.md).

---

## Completed Features (Up to Alpha.6)

### Phase P4 (Core Usability)

| Feature | Notes / Shortcuts |
|---|---|
| Keybindings Directory | `F1` / `Ctrl+G` / `Ctrl+Shift+/` (interactive filter) |
| Compact Search & Replace | Case toggle (`Alt+C`), accent toggle (`Alt+A`), **Regex (`Alt+R`)**, replace all |
| Multi-line Text Selection | `Shift`+arrows, `Ctrl+A`, styled selection background |
| System Clipboard Integration | `arboard` + **OSC 52** terminal escape support + internal fallback |
| Save As / Save All | Interactive path browser modal (`Ctrl+Shift+S`, `Ctrl+Alt+S`) |
| Embedded PTY Terminal | Toggle (`Ctrl+"` / `Ctrl+\``), height resize (`Alt+=` / `Alt+-`) |
| `.editorconfig` Support | Reads `indent_style` / `indent_size` upon file opening |
| Automatic Disk Reload | `notify` file watching + prompt on unsaved dirty conflicts (`Ctrl+R`) |
| Layered TOML Configuration | Global, workspace, and layered overrides |
| CI Workflows & Installer | GitHub Actions CI + `scripts/install.sh` |
| Workspace Session State | Preserves workspace open files and active tabs |

### Phase P3 (LSP Engine & Tooling)

| Feature | Shortcut / Configuration |
|---|---|
| On-Demand LSP Spawn | `[lsp.servers]` mapping |
| Diagnostics Panel | `Ctrl+Shift+M` |
| Autocomplete | Intelligent local suggestions + `Ctrl+Space` for combined semantic/fallback |
| Hover Documentation | `Ctrl+K` |
| Go to Definition | `F4` |
| Document Formatting | `Ctrl+Shift+I` (+ `format_on_save`) |

---

## Post-0.1 Architecture

See **[`docs/en/planning/post-0.1-roadmap.md`](planning/post-0.1-roadmap.md)** for long-term specifications.

| Phase | Milestone | Notes |
|---|---|---|
| **P5** | Project-wide Search | Completed · `Ctrl+Shift+F` · `ripgrep` + Rust fallback |
| **P6** | Markdown Fence Injections | Completed · syntax highlighting for embedded code snippets |
| **P7** | Terminal Markdown Preview | Completed · `Ctrl+Shift+V` / `Alt+P` · synchronized scroll |
| **P8** | Plugin Engine | Completed · `oride-plugin` · palette actions |
| **P9** | Dynamic Splits & Multi-cursor | Completed · `Ctrl+Alt+V/H`, `F6`, `Ctrl+Alt+↑↓` |
