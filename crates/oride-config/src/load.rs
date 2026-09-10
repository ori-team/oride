//! Carrega e mescla camadas de config.

use std::fs;
use std::path::{Path, PathBuf};

use thiserror::Error;

use crate::model::{Config, LanguageConfig, RawConfigFile};

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("read config {path}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("parse config {path}: {source}")]
    Parse {
        path: PathBuf,
        #[source]
        source: toml::de::Error,
    },
}

/// `~/.config/oride/config.toml` (ou equivalente XDG).
#[must_use]
pub fn user_config_path() -> Option<PathBuf> {
    dirs::config_dir().map(|d| d.join("oride").join("config.toml"))
}

/// Caminhos candidatos de projeto: `start` e ancestrais + `.oride/config.toml`.
#[must_use]
pub fn config_search_roots(start: Option<&Path>) -> Vec<PathBuf> {
    let mut roots = Vec::new();
    let mut cur = start
        .map(Path::to_path_buf)
        .or_else(|| std::env::current_dir().ok());

    while let Some(dir) = cur {
        roots.push(dir.join(".oride").join("config.toml"));
        cur = dir.parent().map(Path::to_path_buf);
        if roots.len() > 32 {
            break;
        }
    }
    roots
}

/// Defaults ← user config ← primeiro `.oride/config.toml` encontrado.
pub fn load_merged(workspace_hint: Option<&Path>) -> Result<Config, ConfigError> {
    let mut cfg = Config::default();

    if let Some(user_path) = user_config_path() {
        if user_path.is_file() {
            apply_file(&mut cfg, &user_path)?;
        }
    }

    // Projeto: o mais próximo do hint (primeiro na lista) ganha.
    for path in config_search_roots(workspace_hint) {
        if path.is_file() {
            apply_file(&mut cfg, &path)?;
            break;
        }
    }

    // Aplica tema personalizado caso venha de pasta ~/.config/oride/themes ou .oride/themes
    let registry = crate::theme::ThemeRegistry::load_with_paths(workspace_hint);
    if let Some(theme_def) = registry.get(&cfg.theme) {
        if cfg.theme_ui == crate::model::ThemeUiConfig::default() {
            cfg.theme_ui = theme_def.ui.clone();
        }
        if cfg.syntax == crate::model::SyntaxColorsConfig::default() {
            cfg.syntax = theme_def.syntax.clone();
        }
    }

    // Merge de linguagens descobertas em ~/.config/oride/languages e .oride/languages
    for lang in discover_language_configs(workspace_hint) {
        if !cfg
            .languages
            .iter()
            .any(|l| l.id.eq_ignore_ascii_case(&lang.id))
        {
            cfg.languages.push(lang);
        }
    }

    // Auto-popula lsp.servers a partir de languages configuradas caso não sobrescrito
    for lang in &cfg.languages {
        if !lang.lsp_command.is_empty() && !cfg.lsp.servers.contains_key(&lang.id) {
            cfg.lsp
                .servers
                .insert(lang.id.clone(), lang.lsp_command.clone());
        }
    }

    Ok(cfg)
}

#[derive(Debug, serde::Deserialize)]
struct LanguagesFileWrapper {
    languages: Option<Vec<LanguageConfig>>,
}

/// Descobre definições de linguagens em `~/.config/oride/languages/*.toml` e `<workspace>/.oride/languages/*.toml`.
#[must_use]
pub fn discover_language_configs(workspace_hint: Option<&Path>) -> Vec<LanguageConfig> {
    let mut dirs = Vec::new();
    if let Some(user_dir) = dirs::config_dir().map(|d| d.join("oride").join("languages")) {
        dirs.push(user_dir);
    }
    if let Some(ws) = workspace_hint {
        dirs.push(ws.join(".oride").join("languages"));
    }

    let mut result = Vec::new();
    for dir in dirs {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|e| e.to_str()) == Some("toml") {
                    if let Ok(content) = fs::read_to_string(&path) {
                        if let Ok(wrapper) = toml::from_str::<LanguagesFileWrapper>(&content) {
                            if let Some(langs) = wrapper.languages {
                                for l in langs {
                                    if !result
                                        .iter()
                                        .any(|x: &LanguageConfig| x.id.eq_ignore_ascii_case(&l.id))
                                    {
                                        result.push(l);
                                    }
                                }
                                continue;
                            }
                        }
                        if let Ok(single) = toml::from_str::<LanguageConfig>(&content) {
                            if !single.id.is_empty()
                                && !result
                                    .iter()
                                    .any(|x: &LanguageConfig| x.id.eq_ignore_ascii_case(&single.id))
                            {
                                result.push(single);
                            }
                        }
                    }
                }
            }
        }
    }
    result
}

/// Salva o tema escolhido na configuração de usuário (`~/.config/oride/config.toml`).
pub fn save_user_theme(theme_name: &str) -> Result<(), std::io::Error> {
    let Some(cfg_path) = user_config_path() else {
        return Ok(());
    };
    if let Some(parent) = cfg_path.parent() {
        fs::create_dir_all(parent)?;
    }
    let doc = if cfg_path.is_file() {
        fs::read_to_string(&cfg_path)?
    } else {
        String::new()
    };

    let mut found = false;
    let mut new_lines = Vec::new();
    for line in doc.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("theme =") || trimmed.starts_with("theme=") {
            new_lines.push(format!("theme = \"{theme_name}\""));
            found = true;
        } else {
            new_lines.push(line.to_string());
        }
    }
    if !found {
        new_lines.insert(0, format!("theme = \"{theme_name}\""));
    }
    let mut output = new_lines.join("\n");
    if !output.ends_with('\n') {
        output.push('\n');
    }
    fs::write(&cfg_path, output)?;
    Ok(())
}

/// Salva o idioma escolhido na configuração de usuário (`~/.config/oride/config.toml`).
pub fn save_user_locale(locale: &str) -> Result<(), std::io::Error> {
    let Some(cfg_path) = user_config_path() else {
        return Ok(());
    };
    if let Some(parent) = cfg_path.parent() {
        fs::create_dir_all(parent)?;
    }
    let doc = if cfg_path.is_file() {
        fs::read_to_string(&cfg_path)?
    } else {
        String::new()
    };

    let mut found = false;
    let mut new_lines = Vec::new();
    for line in doc.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("locale =") || trimmed.starts_with("locale=") {
            new_lines.push(format!("locale = \"{locale}\""));
            found = true;
        } else {
            new_lines.push(line.to_string());
        }
    }
    if !found {
        new_lines.insert(0, format!("locale = \"{locale}\""));
    }
    let mut output = new_lines.join("\n");
    if !output.ends_with('\n') {
        output.push('\n');
    }
    fs::write(&cfg_path, output)?;
    Ok(())
}

fn apply_file(cfg: &mut Config, path: &Path) -> Result<(), ConfigError> {
    let text = fs::read_to_string(path).map_err(|source| ConfigError::Io {
        path: path.to_path_buf(),
        source,
    })?;
    let raw: RawConfigFile = toml::from_str(&text).map_err(|source| ConfigError::Parse {
        path: path.to_path_buf(),
        source,
    })?;
    cfg.apply_raw(raw);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn merge_project_overrides_keys() {
        let dir = tempfile::tempdir().unwrap();
        let oride = dir.path().join(".oride");
        fs::create_dir_all(&oride).unwrap();
        let path = oride.join("config.toml");
        let mut f = fs::File::create(&path).unwrap();
        writeln!(
            f,
            r#"
show_line_numbers = false
[editor]
tab_size = 2
completion_auto = false
completion_min_chars = 3
[lsp.servers]
rust = ["rust-analyzer"]
[keys]
"ctrl+s" = "quit"
[ui]
status_dirty = "red"
"#
        )
        .unwrap();

        let mut cfg = Config::default();
        apply_file(&mut cfg, &path).unwrap();
        assert!(!cfg.show_line_numbers);
        assert_eq!(cfg.editor.tab_size, 2);
        assert!(!cfg.editor.completion_auto);
        assert_eq!(cfg.editor.completion_min_chars, 3);
        assert_eq!(
            cfg.lsp.servers.get("rust"),
            Some(&vec!["rust-analyzer".to_string()])
        );
        assert_eq!(cfg.keys.get("ctrl+s").map(String::as_str), Some("quit"));
        assert_eq!(cfg.theme_ui.status_dirty, "red");
        // default key still present
        assert_eq!(cfg.keys.get("ctrl+z").map(String::as_str), Some("undo"));
    }

    #[test]
    fn load_merged_finds_nested_project() {
        let dir = tempfile::tempdir().unwrap();
        let oride = dir.path().join(".oride");
        fs::create_dir_all(&oride).unwrap();
        fs::write(oride.join("config.toml"), "show_line_numbers = false\n").unwrap();
        let nested = dir.path().join("src");
        fs::create_dir_all(&nested).unwrap();

        let cfg = load_merged(Some(&nested)).unwrap();
        assert!(!cfg.show_line_numbers);
    }
}
