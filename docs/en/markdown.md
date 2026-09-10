# Markdown in Oride

Native, first-class Markdown and derivative document support.

---

## File Extensions

| Extension | LanguageId |
|-----------|------------|
| `.md`, `.markdown`, `.mdown`, `.mkd`, `.mkdn`, `.mdwn`, `.mdtxt`, `.mdtext` | `markdown` |
| `.rmd`, `.qmd` | `markdown` |
| `.mdx` | `mdx` (highlighted as Markdown) |
| `README`, `CHANGELOG`, `LICENSE`, etc. | `markdown` |

---

## Syntax Highlighting

Powered by a dual-stage **tree-sitter-md** pipeline:

1. **Block Grammar:** Headings, ordered/unordered lists, code fences, tables, and blockquotes.
2. **Inline Grammar:** Bold, italic, hyperlinks, strike-through, and inline `code` spans.
3. **Official queries** with robust fallback by `node.kind()`.
4. **Code Fence Injections:** Code within ` ```lang ` blocks is highlighted using the respective language grammar. Supported aliases include `rust`/`rs`, `c`/`h`, `bash`/`sh`/`shell`, `python`/`py`, `js`/`javascript`, `ts`/`typescript`, `html`, `css`, and more.

Dedicated token theme colors: `heading`, `emphasis`, `strong`, `link`, `code`, `list_marker`, and `quote`.

---

## Editing Features

| Action | Default Shortcut | Behavior |
|--------|------------------|----------|
| Soft Wrap | `Alt+Z` | Toggles visual word wrapping; **enabled by default** for Markdown |
| Toggle Comment | `Ctrl+/` | Injects `<!-- line -->` for Markdown/HTML; `//` for code |
| Enter in List | `Enter` | Continues list prefix (`- `, `* `, `1. `, `- [ ] `, `> `) |
| Enter on Empty Prefix | `Enter` | Cleans prefix and exits list |

Also accessible via Command Palette (`Ctrl+Shift+P`): **Toggle soft wrap**, **Toggle comment**.

---

## In-Terminal Markdown Preview

**Shortcut:** `Ctrl+Shift+V` or `Alt+P`. Opens a synchronized side-by-side read-only preview pane that follows your buffer scroll position.

### Rendered Elements

| Element | Preview Representation |
|---|---|
| Headings (`#`) | Scaled visual hierarchy and line spacing |
| Lists / Tasks | Bullet points (`•`) and checkboxes (`[ ]` / `[x]`) |
| Blockquotes | Indented line prefix `│` with styled text |
| Code Fences | Fenced frame blocks with syntax highlighting |
| Inline Spans | `code`, **bold**, *italic*, ~~strikethrough~~ |
| Hyperlinks | Formatted as `text → url` (clickable via mouse or `Alt+Enter`) |
| Tables | Unicode box-drawing tables with column alignments |
| Frontmatter | Dimmed metadata card at document top |
| Images | Visual placeholder card or terminal graphics protocol |

### Image Handling
An image link alone on a line renders as a visual card:
- Relative paths are resolved against the directory of the open `.md` file.
- Remote URLs (`http://` or `https://`) display a remote reference badge.
- When `terminal_images = true` in config and running on a supported terminal emulator (Kitty, Ghostty, WezTerm), images are rendered directly using terminal graphics protocols.

---

## Verification

```bash
cargo test -p oride-syntax
```
