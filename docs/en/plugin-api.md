# Plugin Architecture & Extensibility API

Oride maintains a strict **anti-bloat** philosophy ([roadmap alpha.6](planning/alpha6-roadmap.md)). Extensions in 0.2.x are modular Rust traits registered with `PluginHost` during editor initialization.

Crate: **`oride-plugin`**.

---

## LanguageProvider Trait

Provides metadata per language (comment syntax, soft wrap preferences, default LSP command, and offline keyword suggestions). Highlighting queries remain encapsulated in `oride-syntax`.

```rust
pub trait LanguageProvider: Send + Sync {
    fn id(&self) -> &'static str;
    fn language_id(&self) -> LanguageId;
    fn extensions(&self) -> &'static [&'static str];
    fn comment_open(&self) -> Option<&'static str>;
    fn comment_close(&self) -> Option<&'static str>;
    fn lsp_command(&self) -> Option<&'static [&'static str]>;
    fn completion_words(&self) -> &'static [&'static str];
    fn default_soft_wrap(&self) -> bool;
}
```

**Built-in Providers:** `rust`, `c`, `bash`, `markdown`, `ori`, `python`, `javascript`, `typescript`/`tsx`, `html`, `css`, `nim`, `ruby`, `d`, `lua`, and `plain`.

**Usage:** `plugin_host.language(lang)` supplies data for comment toggling, automatic soft wrapping, fallback word completions, and default LSP process commands.

---

## Plugin Trait & PluginCtx

```rust
pub trait PluginCtx {
    fn set_status(&mut self, msg: &str);
    fn workspace_root(&self) -> &Path;
    fn active_path(&self) -> Option<PathBuf>;
    fn active_buffer_text(&self) -> String;
    fn active_is_dirty(&self) -> bool;
}

pub trait Plugin: Send + Sync {
    fn name(&self) -> &'static str;
    fn commands(&self) -> &'static [CommandMeta];
    fn on_hook(&self, hook: PluginHook, ctx: &mut dyn PluginCtx);
    fn run_command(&self, id: &str, ctx: &mut dyn PluginCtx) -> PluginResult;
}
```

### Lifecycle Hooks
- `OnOpen`: Dispatched immediately after a document buffer is loaded into memory.
- `OnSave`: Dispatched immediately after a buffer is successfully written to disk.

### Built-in Plugins

| Plugin | Commands | Hooks | Description |
|---|---|---|---|
| `word-count` | **Plugin: word count** | — | Counts words, characters, and lines in active buffer |
| `show-path` | **Plugin: show file path** | — | Displays the active document's path in status line |
| `lifecycle` | — | `OnOpen`, `OnSave` | Verifies lifecycle hook dispatch in test suites |

---

## Command Palette Integration

The Command Palette (`Ctrl+Shift+P`) dynamically presents built-in editor actions alongside registered plugin commands. Selecting a command executes `host.run_command(id, &mut ctx)`.

---

## Verification

```bash
cargo test -p oride-plugin
```
