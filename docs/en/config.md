# Configuration Reference

Oride loads configuration in a layered hierarchy (later layers override earlier ones):

1. **Built-in defaults** (embedded within the binary).
2. **User global configuration:** `~/.config/oride/config.toml` (XDG standard).
3. **Workspace local configuration:** The first `.oride/config.toml` found walking up from the opened file or CWD.

Full example: [`assets/config.example.toml`](../../assets/config.example.toml).

---

## Top-Level Fields

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `theme` | string | `"default"` | Logical theme name or path to a custom theme |
| `show_line_numbers` | bool | `true` | Display gutter line numbers |
| `mouse` | bool | **`false`** | Mouse capture (click, drag, scroll). Disabled by default; toggle via **View → Enable mouse** or command palette |
| `language` | string | `"en"` | Interface localization locale (e.g. `"en"`, `"pt-BR"`) |
| `modal_editing` | bool | `false` | Enable Vim-style modal editing by default |
| `[editor].tab_size` | u8 | `4` | Tab width in spaces |
| `[editor].insert_spaces` | bool | `true` | Convert Tab presses to spaces |
| `[editor].completion_auto` | bool | `true` | Automatically trigger completion popup while typing |
| `[editor].completion_min_chars` | u8 | `2` | Minimum character prefix required before auto-completion opens |
| `[markdown].terminal_images` | bool | `false` | Enable terminal graphics rendering for Markdown (Kitty, Ghostty, WezTerm) |
| `[ui].*` | color | see defaults | UI theme color overrides |
| `[keys]` | map | default bindings | Keybinding customization |

### Supported Colors

- Named colors: `reset`, `black`, `red`, `green`, `yellow`, `blue`, `magenta`, `cyan`, `gray`, `darkgray`, `white`, `lightred`, `lightgreen`, `lightblue`, etc.
- Hexadecimal notation: `#RGB` or `#RRGGBB`.

---

## Keybinding Actions (`[keys]`)

| Action ID | Description |
|-----------|-------------|
| `quit` | Exit editor (prompt twice if unsaved buffers exist) |
| `save` | Save active file |
| `save_as` | Save active file to a new path |
| `save_all` | Save all modified buffers |
| `undo` / `redo` | Step backwards / forwards in the edit history |
| `insert_newline` / `insert_tab` / `backspace` / `delete` | Text editing primitives |
| `move_left` … `move_line_end` | Caret movement |
| `move_*_extend` | Caret movement with selection extension |
| `page_up` / `page_down` | Scroll viewport by page |
| `toggle_tree` / `toggle_terminal` | Toggle file tree / embedded terminal panels |
| `focus_tree` / `focus_editor` / `focus_terminal` | Direct panel focus shifts |
| `next_tab` / `prev_tab` / `close_tab` / `new_tab` | Tab management |
| `command_palette` / `open_file_fuzzy` | Open command palette or fuzzy file picker |
| `tree_new_file` / `tree_new_dir` / `tree_refresh` | Project file tree management |

Key chords are specified in lowercase with `+` as separator: `ctrl+s`, `shift+left`, `esc`, `pageup`, `ctrl+shift+p`, etc.

---

## Detailed Configuration Sections

### `[editor]`
- `tab_size`: Indentation width.
- `insert_spaces`: Convert tabs to spaces.
- `format_on_save`: Automatically trigger LSP formatting when saving buffers.
- `use_editorconfig`: Read indentation settings from `.editorconfig`.
- `completion_auto`: Auto-trigger completion window.
- `completion_min_chars`: Prefix threshold for automatic completions.

### `[tree]`
- `width`: Sidebar width in character columns (default: 28).
- `show_hidden`: Whether to list dotfiles and hidden directories.
- `git_status`: Render Git status badges next to files and directories.

### `[terminal]`
- `shell`: Shell binary path (defaults to `$SHELL` or `/bin/sh`).
- `default_height`: Height of terminal pane in rows (default: 10).

### `[lsp]`
- `enabled`: Global toggle for language server client.
- `timeout_ms`: Request timeout in milliseconds (default: 10000).
- `[lsp.servers]`: Mapping of `LanguageId → argv` command array. Servers are spawned on-demand when opening files of the corresponding language.
- `oriscript_command`: Maintained solely for backward compatibility with legacy configurations.

```toml
[lsp.servers]
rust = ["rust-analyzer"]
c = ["clangd"]
bash = ["bash-language-server", "start"]
ori = ["ori-lsp"]
```

### `[syntax]`
Syntax highlighting overrides (`keyword`, `string`, `comment`, `function`, `type`, `number`) using color names or `#RRGGBB`.

---

## Validation Commands

```bash
cargo test -p oride-config
cargo test -p oride-keymap
cargo test -p oride-app rebind_ctrl_s
```
