//! Configuração TOML do Oride: defaults → user → projeto.

mod editorconfig;
mod load;
mod model;
pub mod theme;

pub use editorconfig::{resolve_indent_for_file, EditorIndent};
pub use load::{
    config_search_roots, discover_language_configs, load_merged, save_user_locale, save_user_theme,
    user_config_path,
};
pub use model::{
    default_key_bindings, Config, EditorConfig, LanguageConfig, LspConfig, MarkdownConfig,
    SyntaxColorsConfig, TerminalConfig, ThemeUiConfig, TreeConfig,
};
pub use theme::{built_in_themes, normalize_theme_name, ThemeDefinition, ThemeRegistry};
