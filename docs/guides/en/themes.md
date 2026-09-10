# Theme Development Guide — Oride

**Oride** supports complete visual themes in a declarative **TOML** format, featuring real-time **Live Preview** in the terminal interface and dynamic loading without recompilation.

---

## 1. Where to Place Your Theme File

Oride scans the following directories on startup:

1. **User Global:** `~/.config/oride/themes/<your-theme>.toml`
2. **Project Workspace:** `<workspace>/.oride/themes/<your-theme>.toml`
3. **Built-in Themes:** `default-dark`, `dracula`, `nord`, `one-dark`, `tokyo-night`, `catppuccin-mocha`, `monokai`, `solarized-light`, `solarized-dark`, `github-dark`.

Any `.toml` file placed in these folders immediately becomes selectable in the theme picker!

---

## 2. Structure of `my-theme.toml`

A theme consists of basic metadata (`name`, `is_dark`), a `[ui]` block (interface and editor chrome colors), and a `[syntax]` block (Tree-Sitter and lexical token colors).

### Complete Example: Tokyo Night Dark

```toml
name = "Tokyo Night"
is_dark = true

[ui]
background = "#1a1b26"
foreground = "#c0caf5"
line_number = "#565f89"
cursor_bg = "#c0caf5"
cursor_fg = "#1a1b26"
status_bg = "#16161e"
status_fg = "#7aa2f7"
status_dirty = "#e0af68"
gutter_width = 5

[syntax]
comment = "#565f89"
keyword = "#bb9af7"
string = "#9ece6a"
number = "#ff9e64"
type_name = "#2ac3de"
function = "#7aa2f7"
operator = "#89ddff"
punctuation = "#c0caf5"
variable = "#c0caf5"
constant = "#ff9e64"
property = "#7dcfff"
tag = "#f7768e"
attribute = "#bb9af7"
heading = "#7aa2f7"
emphasis = "#e0af68"
strong = "#ff9e64"
link = "#7dcfff"
code = "#7aa2f7"
list_marker = "#bb9af7"
quote = "#565f89"
```

---

## 3. Token Reference Table

### `[ui]` Block (Editor Chrome & Layout)

| Field | Type | Description |
|---|---|---|
| `background` | Hex / Name | Main editor and window background color |
| `foreground` | Hex / Name | Default text color |
| `line_number` | Hex / Name | Line number gutter text color |
| `cursor_bg` | Hex / Name | Caret block background color |
| `cursor_fg` | Hex / Name | Caret character foreground color |
| `status_bg` | Hex / Name | Status line background color |
| `status_fg` | Hex / Name | Status line text color |
| `status_dirty` | Hex / Name | Dirty buffer indicator color (`*`) |
| `gutter_width` | Integer | Minimum width of line number gutter (e.g. `5`) |

### `[syntax]` Block (Syntax Highlighting)

| Token | Description | Usage Examples |
|---|---|---|
| `keyword` | Language keywords | `fn`, `let`, `if`, `return`, `pub`, `struct` |
| `function` | Function and method identifiers | `main`, `push`, `calculate_total` |
| `type_name` | Primitive types and user structs/classes | `i32`, `String`, `Option`, `User` |
| `string` | Quoted string literals | `"hello world"`, `'c'` |
| `number` | Numeric literals | `42`, `3.14`, `0xFF` |
| `comment` | Single-line and block comments | `// comment`, `/* block */`, `# bash` |
| `operator` | Math and logical operators | `+`, `-`, `*`, `==`, `&&`, `\|\|` |
| `punctuation` | Punctuation and delimiters | `(`, `)`, `{`, `}`, `;`, `,` |
| `variable` | Local variables and arguments | `self`, `index`, `payload` |
| `constant` | Constants and global immutable values | `MAX_SIZE`, `PI` |
| `property` | Struct fields and properties | `user.name`, `config.theme` |
| `tag` | Markup tags (HTML/XML/TSX) | `<div>`, `<span>`, `<main>` |
| `attribute` | Markup attributes and decorators | `class`, `href`, `#[derive]` |
| `heading` | Markdown section headings | `# Title`, `## Subtitle` |
| `emphasis` | Markdown italic text | `*italic*` |
| `strong` | Markdown bold text | `**bold**` |
| `link` | Markdown hyperlinks | `[text](url)` |
| `code` | Inline code spans | `` `code` `` |
| `list_marker` | Markdown list bullet or numbering | `-`, `*`, `1.` |
| `quote` | Blockquote prefix | `> quoted text` |

---

## 4. Supported Color Formats

1. **24-bit TrueColor Hexadecimal:**
   - `#1a1b26`, `#ff79c6`, `#50fa7b`, `#f8f8f2`
2. **Named ANSI Colors (legacy terminal compatibility):**
   - `black`, `red`, `green`, `yellow`, `blue`, `magenta`, `cyan`, `white`
   - `dark_gray`, `light_red`, `light_green`, `light_yellow`, `light_blue`, `light_magenta`, `light_cyan`

---

## 5. Live Preview Testing

1. Save your theme into `~/.config/oride/themes/my-theme.toml`.
2. Launch Oride:
   ```bash
   oride .
   ```
3. Open the Theme Picker:
   - Menu: **View → Color Theme…**
   - Keyboard: `Ctrl+Shift+P` → select **Preferences: Color Theme (Live Preview)**
4. Navigate using `↑` and `↓`:
   - Oride updates the entire UI in real time as you scroll through themes.
5. Press `Enter` to commit the selection (saved automatically to `~/.config/oride/config.toml`), or `Esc` to revert.
