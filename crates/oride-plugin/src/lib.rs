//! Superfície de extensão **built-in** (P8).
//!
//! Sem carregamento dinâmico (Lua/WASM). Plugins e language providers
//! são registrados no host em tempo de compilação.

mod completion;
mod external;
mod host;
mod language;
mod manifest;
mod plugin;

pub use external::{discover_external_plugins, ExternalPlugin};
pub use host::{builtin_host, PluginHost};
pub use language::{
    BuiltinLang, DynamicLang, LanguageProvider, LANG_CSS, LANG_HTML, LANG_JS, LANG_MD, LANG_MDX,
    LANG_ORIS, LANG_PLAIN,
};
pub use manifest::{
    HookCommandDef, PluginCommandDef, PluginHeader, PluginHooksDef, PluginManifest,
};
pub use plugin::{
    CommandMeta, Plugin, PluginCtx, PluginError, PluginHook, PluginResult, ShowPathPlugin,
    WordCountPlugin,
};
