//! Catálogo de temas visuais (presets embutidos + descoberta em disco).

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::model::{SyntaxColorsConfig, ThemeUiConfig};

/// Definição completa de um tema visual serializável em TOML.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ThemeDefinition {
    pub name: String,
    pub is_dark: bool,
    pub ui: ThemeUiConfig,
    pub syntax: SyntaxColorsConfig,
}

impl Default for ThemeDefinition {
    fn default() -> Self {
        Self {
            name: "Default Dark".into(),
            is_dark: true,
            ui: ThemeUiConfig::default(),
            syntax: SyntaxColorsConfig::default(),
        }
    }
}

/// Registro unificado de temas (embutidos + pastas do usuário e workspace).
#[derive(Debug, Clone, Default)]
pub struct ThemeRegistry {
    themes: BTreeMap<String, ThemeDefinition>,
}

impl ThemeRegistry {
    /// Cria registro com todos os temas padrão embutidos.
    #[must_use]
    pub fn new() -> Self {
        let mut reg = Self::default();
        for t in built_in_themes() {
            reg.register(t);
        }
        reg
    }

    /// Cria registro e escaneia automaticamente diretórios do usuário e workspace.
    #[must_use]
    pub fn load_with_paths(workspace_hint: Option<&Path>) -> Self {
        let mut reg = Self::new();
        if let Some(user_dir) = dirs::config_dir().map(|d| d.join("oride").join("themes")) {
            reg.load_from_dir(&user_dir);
        }
        if let Some(ws) = workspace_hint {
            reg.load_from_dir(&ws.join(".oride").join("themes"));
        }
        reg
    }

    pub fn register(&mut self, theme: ThemeDefinition) {
        let key = normalize_theme_name(&theme.name);
        self.themes.insert(key, theme);
    }

    #[must_use]
    pub fn get(&self, name: &str) -> Option<&ThemeDefinition> {
        let key = normalize_theme_name(name);
        if let Some(t) = self.themes.get(&key) {
            return Some(t);
        }
        // Fallback: se buscou "default", aceita "default-dark"
        if key == "default" {
            return self.themes.get("default-dark");
        }
        None
    }

    #[must_use]
    pub fn list_themes(&self) -> Vec<&ThemeDefinition> {
        self.themes.values().collect()
    }

    #[must_use]
    pub fn list_names(&self) -> Vec<String> {
        self.themes.values().map(|t| t.name.clone()).collect()
    }

    pub fn load_from_dir(&mut self, dir: &Path) {
        if !dir.is_dir() {
            return;
        }
        let Ok(entries) = fs::read_dir(dir) else {
            return;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("toml") {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(theme) = toml::from_str::<ThemeDefinition>(&content) {
                        self.register(theme);
                    }
                }
            }
        }
    }
}

pub fn normalize_theme_name(name: &str) -> String {
    name.trim().to_lowercase().replace([' ', '_'], "-")
}

/// Conjunto curado de temas embutidos de alta qualidade.
#[must_use]
pub fn built_in_themes() -> Vec<ThemeDefinition> {
    vec![
        default_dark(),
        dracula(),
        nord(),
        one_dark(),
        tokyo_night(),
        catppuccin_mocha(),
        monokai(),
        solarized_light(),
        solarized_dark(),
        github_dark(),
    ]
}

fn default_dark() -> ThemeDefinition {
    ThemeDefinition {
        name: "Default Dark".into(),
        is_dark: true,
        ui: ThemeUiConfig::default(),
        syntax: SyntaxColorsConfig::default(),
    }
}

fn dracula() -> ThemeDefinition {
    ThemeDefinition {
        name: "Dracula".into(),
        is_dark: true,
        ui: ThemeUiConfig {
            background: "#282a36".into(),
            foreground: "#f8f8f2".into(),
            line_number: "#6272a4".into(),
            status_bg: "#44475a".into(),
            status_fg: "#f8f8f2".into(),
            status_dirty: "#ffb86c".into(),
            cursor_bg: "#f8f8f2".into(),
            cursor_fg: "#282a36".into(),
            gutter_width: 5,
        },
        syntax: SyntaxColorsConfig {
            comment: "#6272a4".into(),
            keyword: "#ff79c6".into(),
            string: "#f1fa8c".into(),
            number: "#bd93f9".into(),
            type_name: "#8be9fd".into(),
            function: "#50fa7b".into(),
            operator: "#ff79c6".into(),
            punctuation: "#f8f8f2".into(),
            variable: "#f8f8f2".into(),
            constant: "#bd93f9".into(),
            property: "#66d9ef".into(),
            tag: "#ff79c6".into(),
            attribute: "#50fa7b".into(),
            heading: "#bd93f9".into(),
            emphasis: "#f1fa8c".into(),
            strong: "#ffb86c".into(),
            link: "#8be9fd".into(),
            code: "#50fa7b".into(),
            list_marker: "#ff79c6".into(),
            quote: "#6272a4".into(),
        },
    }
}

fn nord() -> ThemeDefinition {
    ThemeDefinition {
        name: "Nord".into(),
        is_dark: true,
        ui: ThemeUiConfig {
            background: "#2e3440".into(),
            foreground: "#d8dee9".into(),
            line_number: "#4c566a".into(),
            status_bg: "#3b4252".into(),
            status_fg: "#eceff4".into(),
            status_dirty: "#ebcb8b".into(),
            cursor_bg: "#d8dee9".into(),
            cursor_fg: "#2e3440".into(),
            gutter_width: 5,
        },
        syntax: SyntaxColorsConfig {
            comment: "#4c566a".into(),
            keyword: "#81a1c1".into(),
            string: "#a3be8c".into(),
            number: "#b48ead".into(),
            type_name: "#8fbcbb".into(),
            function: "#88c0d0".into(),
            operator: "#81a1c1".into(),
            punctuation: "#eceff4".into(),
            variable: "#d8dee9".into(),
            constant: "#d08770".into(),
            property: "#8fbcbb".into(),
            tag: "#bf616a".into(),
            attribute: "#8fbcbb".into(),
            heading: "#88c0d0".into(),
            emphasis: "#ebcb8b".into(),
            strong: "#d08770".into(),
            link: "#88c0d0".into(),
            code: "#a3be8c".into(),
            list_marker: "#81a1c1".into(),
            quote: "#4c566a".into(),
        },
    }
}

fn one_dark() -> ThemeDefinition {
    ThemeDefinition {
        name: "One Dark".into(),
        is_dark: true,
        ui: ThemeUiConfig {
            background: "#282c34".into(),
            foreground: "#abb2bf".into(),
            line_number: "#4b5263".into(),
            status_bg: "#21252b".into(),
            status_fg: "#abb2bf".into(),
            status_dirty: "#e5c07b".into(),
            cursor_bg: "#528bff".into(),
            cursor_fg: "#282c34".into(),
            gutter_width: 5,
        },
        syntax: SyntaxColorsConfig {
            comment: "#5c6370".into(),
            keyword: "#c678dd".into(),
            string: "#98c379".into(),
            number: "#d19a66".into(),
            type_name: "#e5c07b".into(),
            function: "#61afef".into(),
            operator: "#56b6c2".into(),
            punctuation: "#abb2bf".into(),
            variable: "#e06c75".into(),
            constant: "#d19a66".into(),
            property: "#e06c75".into(),
            tag: "#e06c75".into(),
            attribute: "#d19a66".into(),
            heading: "#61afef".into(),
            emphasis: "#c678dd".into(),
            strong: "#e5c07b".into(),
            link: "#56b6c2".into(),
            code: "#98c379".into(),
            list_marker: "#c678dd".into(),
            quote: "#5c6370".into(),
        },
    }
}

fn tokyo_night() -> ThemeDefinition {
    ThemeDefinition {
        name: "Tokyo Night".into(),
        is_dark: true,
        ui: ThemeUiConfig {
            background: "#1a1b26".into(),
            foreground: "#a9b1d6".into(),
            line_number: "#363b54".into(),
            status_bg: "#16161e".into(),
            status_fg: "#a9b1d6".into(),
            status_dirty: "#e0af68".into(),
            cursor_bg: "#c0caf5".into(),
            cursor_fg: "#1a1b26".into(),
            gutter_width: 5,
        },
        syntax: SyntaxColorsConfig {
            comment: "#565f89".into(),
            keyword: "#bb9af7".into(),
            string: "#9ece6a".into(),
            number: "#ff9e64".into(),
            type_name: "#2ac3de".into(),
            function: "#7aa2f7".into(),
            operator: "#89ddff".into(),
            punctuation: "#c0caf5".into(),
            variable: "#c0caf5".into(),
            constant: "#ff9e64".into(),
            property: "#73daca".into(),
            tag: "#f7768e".into(),
            attribute: "#bb9af7".into(),
            heading: "#7aa2f7".into(),
            emphasis: "#e0af68".into(),
            strong: "#ff9e64".into(),
            link: "#7dcfff".into(),
            code: "#9ece6a".into(),
            list_marker: "#bb9af7".into(),
            quote: "#565f89".into(),
        },
    }
}

fn catppuccin_mocha() -> ThemeDefinition {
    ThemeDefinition {
        name: "Catppuccin Mocha".into(),
        is_dark: true,
        ui: ThemeUiConfig {
            background: "#1e1e2e".into(),
            foreground: "#cdd6f4".into(),
            line_number: "#585b70".into(),
            status_bg: "#181825".into(),
            status_fg: "#cdd6f4".into(),
            status_dirty: "#f9e2af".into(),
            cursor_bg: "#f5e0dc".into(),
            cursor_fg: "#1e1e2e".into(),
            gutter_width: 5,
        },
        syntax: SyntaxColorsConfig {
            comment: "#6c7086".into(),
            keyword: "#cba6f7".into(),
            string: "#a6e3a1".into(),
            number: "#fab387".into(),
            type_name: "#89dceb".into(),
            function: "#89b4fa".into(),
            operator: "#94e2d5".into(),
            punctuation: "#cdd6f4".into(),
            variable: "#cdd6f4".into(),
            constant: "#fab387".into(),
            property: "#f2cdcd".into(),
            tag: "#f38ba8".into(),
            attribute: "#f9e2af".into(),
            heading: "#89b4fa".into(),
            emphasis: "#f9e2af".into(),
            strong: "#fab387".into(),
            link: "#89dceb".into(),
            code: "#a6e3a1".into(),
            list_marker: "#cba6f7".into(),
            quote: "#6c7086".into(),
        },
    }
}

fn monokai() -> ThemeDefinition {
    ThemeDefinition {
        name: "Monokai".into(),
        is_dark: true,
        ui: ThemeUiConfig {
            background: "#272822".into(),
            foreground: "#f8f8f2".into(),
            line_number: "#75715e".into(),
            status_bg: "#3e3d32".into(),
            status_fg: "#f8f8f2".into(),
            status_dirty: "#fd971f".into(),
            cursor_bg: "#f8f8f0".into(),
            cursor_fg: "#272822".into(),
            gutter_width: 5,
        },
        syntax: SyntaxColorsConfig {
            comment: "#75715e".into(),
            keyword: "#f92672".into(),
            string: "#e6db74".into(),
            number: "#ae81ff".into(),
            type_name: "#66d9ef".into(),
            function: "#a6e22e".into(),
            operator: "#f92672".into(),
            punctuation: "#f8f8f2".into(),
            variable: "#f8f8f2".into(),
            constant: "#ae81ff".into(),
            property: "#a6e22e".into(),
            tag: "#f92672".into(),
            attribute: "#a6e22e".into(),
            heading: "#f92672".into(),
            emphasis: "#e6db74".into(),
            strong: "#fd971f".into(),
            link: "#66d9ef".into(),
            code: "#a6e22e".into(),
            list_marker: "#f92672".into(),
            quote: "#75715e".into(),
        },
    }
}

fn solarized_light() -> ThemeDefinition {
    ThemeDefinition {
        name: "Solarized Light".into(),
        is_dark: false,
        ui: ThemeUiConfig {
            background: "#fdf6e3".into(),
            foreground: "#657b83".into(),
            line_number: "#93a1a1".into(),
            status_bg: "#eee8d5".into(),
            status_fg: "#586e75".into(),
            status_dirty: "#cb4b16".into(),
            cursor_bg: "#586e75".into(),
            cursor_fg: "#fdf6e3".into(),
            gutter_width: 5,
        },
        syntax: SyntaxColorsConfig {
            comment: "#93a1a1".into(),
            keyword: "#859900".into(),
            string: "#2aa198".into(),
            number: "#d33682".into(),
            type_name: "#b58900".into(),
            function: "#268bd2".into(),
            operator: "#859900".into(),
            punctuation: "#657b83".into(),
            variable: "#657b83".into(),
            constant: "#cb4b16".into(),
            property: "#268bd2".into(),
            tag: "#dc322f".into(),
            attribute: "#b58900".into(),
            heading: "#268bd2".into(),
            emphasis: "#b58900".into(),
            strong: "#cb4b16".into(),
            link: "#2aa198".into(),
            code: "#859900".into(),
            list_marker: "#859900".into(),
            quote: "#93a1a1".into(),
        },
    }
}

fn solarized_dark() -> ThemeDefinition {
    ThemeDefinition {
        name: "Solarized Dark".into(),
        is_dark: true,
        ui: ThemeUiConfig {
            background: "#002b36".into(),
            foreground: "#839496".into(),
            line_number: "#586e75".into(),
            status_bg: "#073642".into(),
            status_fg: "#93a1a1".into(),
            status_dirty: "#cb4b16".into(),
            cursor_bg: "#93a1a1".into(),
            cursor_fg: "#002b36".into(),
            gutter_width: 5,
        },
        syntax: SyntaxColorsConfig {
            comment: "#586e75".into(),
            keyword: "#859900".into(),
            string: "#2aa198".into(),
            number: "#d33682".into(),
            type_name: "#b58900".into(),
            function: "#268bd2".into(),
            operator: "#859900".into(),
            punctuation: "#839496".into(),
            variable: "#839496".into(),
            constant: "#cb4b16".into(),
            property: "#268bd2".into(),
            tag: "#dc322f".into(),
            attribute: "#b58900".into(),
            heading: "#268bd2".into(),
            emphasis: "#b58900".into(),
            strong: "#cb4b16".into(),
            link: "#2aa198".into(),
            code: "#859900".into(),
            list_marker: "#859900".into(),
            quote: "#586e75".into(),
        },
    }
}

fn github_dark() -> ThemeDefinition {
    ThemeDefinition {
        name: "GitHub Dark".into(),
        is_dark: true,
        ui: ThemeUiConfig {
            background: "#0d1117".into(),
            foreground: "#c9d1d9".into(),
            line_number: "#484f58".into(),
            status_bg: "#161b22".into(),
            status_fg: "#c9d1d9".into(),
            status_dirty: "#d29922".into(),
            cursor_bg: "#58a6ff".into(),
            cursor_fg: "#0d1117".into(),
            gutter_width: 5,
        },
        syntax: SyntaxColorsConfig {
            comment: "#8b949e".into(),
            keyword: "#ff7b72".into(),
            string: "#a5d6ff".into(),
            number: "#79c0ff".into(),
            type_name: "#ffa657".into(),
            function: "#d2a8ff".into(),
            operator: "#79c0ff".into(),
            punctuation: "#c9d1d9".into(),
            variable: "#c9d1d9".into(),
            constant: "#79c0ff".into(),
            property: "#79c0ff".into(),
            tag: "#7ee787".into(),
            attribute: "#79c0ff".into(),
            heading: "#58a6ff".into(),
            emphasis: "#d2a8ff".into(),
            strong: "#ffa657".into(),
            link: "#58a6ff".into(),
            code: "#a5d6ff".into(),
            list_marker: "#ff7b72".into(),
            quote: "#8b949e".into(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_built_in_themes() {
        let reg = ThemeRegistry::new();
        assert!(reg.get("dracula").is_some());
        assert!(reg.get("nord").is_some());
        assert!(reg.get("one-dark").is_some());
        assert!(reg.get("tokyo-night").is_some());
        assert!(reg.get("catppuccin-mocha").is_some());
        assert!(reg.get("monokai").is_some());
        assert!(reg.get("solarized-light").is_some());
        assert!(reg.get("solarized-dark").is_some());
        assert!(reg.get("github-dark").is_some());
        assert!(reg.get("default").is_some() || reg.get("default-dark").is_some());
    }

    #[test]
    fn custom_theme_registration() {
        let mut reg = ThemeRegistry::new();
        let custom = ThemeDefinition {
            name: "Cyberpunk 2077".into(),
            is_dark: true,
            ui: ThemeUiConfig::default(),
            syntax: SyntaxColorsConfig::default(),
        };
        reg.register(custom);
        assert!(reg.get("cyberpunk-2077").is_some());
    }
}
