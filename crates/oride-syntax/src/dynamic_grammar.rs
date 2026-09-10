//! Carregamento dinâmico de gramáticas Tree-Sitter externas (.so / .dylib / .dll).

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};

use libloading::{Library, Symbol};
use tree_sitter::Language;

static LOADED_LIBRARIES: OnceLock<Mutex<Vec<Library>>> = OnceLock::new();
static GRAMMAR_CACHE: OnceLock<Mutex<HashMap<String, Option<Language>>>> = OnceLock::new();

fn loaded_libraries() -> &'static Mutex<Vec<Library>> {
    LOADED_LIBRARIES.get_or_init(|| Mutex::new(Vec::new()))
}

fn grammar_cache() -> &'static Mutex<HashMap<String, Option<Language>>> {
    GRAMMAR_CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Tenta carregar dinamicamente uma gramática Tree-Sitter externa para o nome da linguagem.
/// Busca em `~/.config/oride/grammars/`, `./.oride/grammars/` e plugins.
#[must_use]
pub fn load_dynamic_grammar(lang_name: &str) -> Option<Language> {
    let mut cache = grammar_cache().lock().ok()?;
    if let Some(cached) = cache.get(lang_name) {
        return cached.clone();
    }

    let loaded = load_grammar_from_disk(lang_name);
    cache.insert(lang_name.to_string(), loaded.clone());
    loaded
}

fn load_grammar_from_disk(lang_name: &str) -> Option<Language> {
    let search_paths = grammar_search_paths();
    let file_candidates = candidate_filenames(lang_name);

    for dir in &search_paths {
        for filename in &file_candidates {
            let path = dir.join(filename);
            if path.is_file() {
                if let Some(lang) = try_load_library(&path, lang_name) {
                    return Some(lang);
                }
            }
        }
    }
    None
}

fn candidate_filenames(lang_name: &str) -> Vec<String> {
    #[cfg(target_os = "linux")]
    let extensions: &[&str] = &["so"];
    #[cfg(target_os = "macos")]
    let extensions: &[&str] = &["dylib", "so"];
    #[cfg(target_os = "windows")]
    let extensions: &[&str] = &["dll"];
    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    let extensions: &[&str] = &["so"];

    let mut candidates = Vec::new();
    for ext in extensions {
        candidates.push(format!("libtree-sitter-{lang_name}.{ext}"));
        candidates.push(format!("tree-sitter-{lang_name}.{ext}"));
        candidates.push(format!("{lang_name}.{ext}"));
        candidates.push(format!("tree_sitter_{lang_name}.{ext}"));
    }
    candidates
}

fn grammar_search_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();
    // 1. Workspace local: ./.oride/grammars/
    paths.push(PathBuf::from(".oride").join("grammars"));

    // 2. User config dir: ~/.config/oride/grammars/
    if let Some(config_dir) = dirs::config_dir() {
        paths.push(config_dir.join("oride").join("grammars"));
    }

    // 3. User plugins dir: ~/.config/oride/plugins/<lang>/
    if let Some(config_dir) = dirs::config_dir() {
        paths.push(config_dir.join("oride").join("plugins"));
    }

    paths
}

fn try_load_library(path: &Path, lang_name: &str) -> Option<Language> {
    unsafe {
        let lib = Library::new(path).ok()?;
        let symbol_name = format!("tree_sitter_{lang_name}");
        let func: Symbol<unsafe extern "C" fn() -> *const ()> =
            lib.get(symbol_name.as_bytes()).ok()?;
        let lang_fn = tree_sitter_language::LanguageFn::from_raw(*func);
        let lang: Language = lang_fn.into();

        if let Ok(mut libs) = loaded_libraries().lock() {
            libs.push(lib);
        }
        Some(lang)
    }
}
