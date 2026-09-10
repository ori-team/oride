//! Definição de manifesto TOML para plugins externos (`plugin.toml`).

use serde::{Deserialize, Serialize};
use std::path::Path;

/// Manifesto completo de um plugin em `plugin.toml`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PluginManifest {
    pub plugin: PluginHeader,
    #[serde(default)]
    pub commands: Vec<PluginCommandDef>,
    #[serde(default)]
    pub hooks: Option<PluginHooksDef>,
}

/// Cabeçalho com identificação do plugin.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PluginHeader {
    pub name: String,
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub description: String,
}

/// Definição de comando executável via command palette ou atalho.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PluginCommandDef {
    pub id: String,
    pub label: String,
    #[serde(default)]
    pub description: String,
    pub executable: String,
    #[serde(default)]
    pub args: Vec<String>,
}

/// Hooks de ciclo de vida configuráveis.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct PluginHooksDef {
    #[serde(default)]
    pub on_open: Option<HookCommandDef>,
    #[serde(default)]
    pub on_save: Option<HookCommandDef>,
}

/// Definição de comando executado em um hook.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HookCommandDef {
    pub executable: String,
    #[serde(default)]
    pub args: Vec<String>,
}

impl PluginManifest {
    /// Faz o parse de uma string TOML contendo a especificação do plugin.
    pub fn parse(content: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(content)
    }

    /// Lê e faz o parse de um arquivo `plugin.toml`.
    pub fn from_file(path: &Path) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let content = std::fs::read_to_string(path)?;
        let manifest = Self::parse(&content)?;
        Ok(manifest)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_plugin_manifest_roundtrip() {
        let sample = r#"
[plugin]
name = "formatter"
version = "0.1.0"
description = "Código formatado automaticamente"

[[commands]]
id = "fmt"
label = "Plugin: Formatar Arquivo"
description = "Formata o buffer com prettier"
executable = "npx"
args = ["prettier", "--write", "$FILE"]

[hooks.on_save]
executable = "echo"
args = ["salvo:", "$FILE"]
"#;

        let manifest = PluginManifest::parse(sample).expect("falha ao fazer parse do manifest");
        assert_eq!(manifest.plugin.name, "formatter");
        assert_eq!(manifest.commands.len(), 1);
        assert_eq!(manifest.commands[0].id, "fmt");
        assert_eq!(manifest.commands[0].executable, "npx");
        assert!(manifest.hooks.is_some());
        assert!(manifest.hooks.as_ref().unwrap().on_save.is_some());
    }
}
