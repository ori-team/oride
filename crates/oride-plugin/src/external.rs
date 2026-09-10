//! Suporte e execução para plugins externos baseados em manifesto (`plugin.toml`).

use std::path::{Path, PathBuf};

use crate::manifest::{HookCommandDef, PluginCommandDef, PluginHooksDef, PluginManifest};
use crate::plugin::{CommandMeta, Plugin, PluginCtx, PluginError, PluginHook, PluginResult};

/// Plugin externo carregado a partir de um diretório com `plugin.toml`.
pub struct ExternalPlugin {
    name: &'static str,
    root_dir: PathBuf,
    commands: &'static [CommandMeta],
    command_defs: Vec<PluginCommandDef>,
    hooks: Option<PluginHooksDef>,
}

impl ExternalPlugin {
    /// Constrói um `ExternalPlugin` a partir de um manifesto parseado e seu diretório raiz.
    #[must_use]
    pub fn from_manifest(manifest: PluginManifest, root_dir: PathBuf) -> Self {
        let name: &'static str = Box::leak(manifest.plugin.name.into_boxed_str());
        let meta_vec: Vec<CommandMeta> = manifest
            .commands
            .iter()
            .map(|cmd| CommandMeta {
                id: Box::leak(cmd.id.clone().into_boxed_str()),
                label: Box::leak(cmd.label.clone().into_boxed_str()),
                description: Box::leak(cmd.description.clone().into_boxed_str()),
            })
            .collect();
        let commands: &'static [CommandMeta] = Box::leak(meta_vec.into_boxed_slice());

        Self {
            name,
            root_dir,
            commands,
            command_defs: manifest.commands,
            hooks: manifest.hooks,
        }
    }

    /// Carrega um plugin externo diretamente de um arquivo `plugin.toml`.
    pub fn load_file(
        manifest_path: &Path,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let manifest = PluginManifest::from_file(manifest_path)?;
        let root_dir = manifest_path
            .parent()
            .map(Path::to_path_buf)
            .unwrap_or_else(|| PathBuf::from("."));
        Ok(Self::from_manifest(manifest, root_dir))
    }

    /// Diretório raiz onde o plugin reside.
    #[must_use]
    pub fn root_dir(&self) -> &Path {
        &self.root_dir
    }
}

impl Plugin for ExternalPlugin {
    fn name(&self) -> &'static str {
        self.name
    }

    fn commands(&self) -> &'static [CommandMeta] {
        self.commands
    }

    fn on_hook(&self, hook: PluginHook, ctx: &mut dyn PluginCtx) {
        let Some(hooks) = &self.hooks else {
            return;
        };
        let hook_cmd = match hook {
            PluginHook::OnOpen => hooks.on_open.as_ref(),
            PluginHook::OnSave => hooks.on_save.as_ref(),
        };

        if let Some(cmd) = hook_cmd {
            let working_dir = if self.root_dir.is_dir() {
                self.root_dir.clone()
            } else {
                ctx.workspace_root().to_path_buf()
            };
            if let Err(err) = execute_hook_command(cmd, &working_dir, ctx) {
                // Fail closed: reporta status sem quebrar a execução do editor
                ctx.set_status(&format!("{}: hook error: {err}", self.name));
            }
        }
    }

    fn run_command(&self, id: &str, ctx: &mut dyn PluginCtx) -> PluginResult {
        let cmd_def = self
            .command_defs
            .iter()
            .find(|c| c.id == id || c.label == id)
            .ok_or_else(|| PluginError::UnknownCommand(id.to_string()))?;

        let working_dir = if self.root_dir.is_dir() {
            self.root_dir.clone()
        } else {
            ctx.workspace_root().to_path_buf()
        };

        match execute_plugin_command(cmd_def, &working_dir, ctx) {
            Ok(output) => {
                if !output.is_empty() {
                    let first_line = output.lines().next().unwrap_or("").trim();
                    if !first_line.is_empty() {
                        ctx.set_status(&format!("{}: {first_line}", self.name));
                    } else {
                        ctx.set_status(&format!("{}: concluído com sucesso", self.name));
                    }
                } else {
                    ctx.set_status(&format!("{}: concluído com sucesso", self.name));
                }
                Ok(())
            }
            Err(err) => Err(PluginError::Message(format!("{}: {err}", self.name))),
        }
    }
}

/// Expande variáveis de ambiente no formato `$VAR`:
/// `$FILE`, `$DIR`, `$WORKSPACE`.
fn expand_vars(arg: &str, ctx: &dyn PluginCtx) -> String {
    let file_path = ctx
        .active_path()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_default();
    let dir_path = ctx
        .active_path()
        .and_then(|p| p.parent().map(|d| d.to_string_lossy().to_string()))
        .unwrap_or_default();
    let workspace_path = ctx.workspace_root().to_string_lossy().to_string();

    arg.replace("$FILE", &file_path)
        .replace("$DIR", &dir_path)
        .replace("$WORKSPACE", &workspace_path)
}

fn execute_plugin_command(
    cmd_def: &PluginCommandDef,
    working_dir: &Path,
    ctx: &mut dyn PluginCtx,
) -> Result<String, String> {
    let mut cmd = std::process::Command::new(&cmd_def.executable);
    for arg in &cmd_def.args {
        cmd.arg(expand_vars(arg, ctx));
    }
    cmd.current_dir(working_dir);

    match cmd.output() {
        Ok(output) => {
            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).to_string())
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
                if !stderr.is_empty() {
                    Err(stderr)
                } else {
                    Err(format!("status {}", output.status))
                }
            }
        }
        Err(err) => Err(format!("executável '{}': {err}", cmd_def.executable)),
    }
}

fn execute_hook_command(
    hook_cmd: &HookCommandDef,
    working_dir: &Path,
    ctx: &dyn PluginCtx,
) -> Result<(), String> {
    let mut cmd = std::process::Command::new(&hook_cmd.executable);
    for arg in &hook_cmd.args {
        cmd.arg(expand_vars(arg, ctx));
    }
    cmd.current_dir(working_dir);

    match cmd.status() {
        Ok(status) => {
            if status.success() {
                Ok(())
            } else {
                Err(format!("status {status}"))
            }
        }
        Err(err) => Err(format!("executável '{}': {err}", hook_cmd.executable)),
    }
}

/// Descobre e carrega plugins externos em subpastas dos caminhos de busca fornecidos.
/// Exemplo de estrutura esperada:
/// `<search_dir>/plugins/<plugin-name>/plugin.toml` ou `<search_dir>/<plugin-name>/plugin.toml`.
#[must_use]
pub fn discover_external_plugins(search_dirs: &[PathBuf]) -> Vec<ExternalPlugin> {
    let mut loaded = Vec::new();
    for base in search_dirs {
        if !base.is_dir() {
            continue;
        }
        // Verifica se a base já é a pasta `plugins` ou se tem subpasta `plugins`
        let candidates = if base.ends_with("plugins") {
            vec![base.clone()]
        } else {
            vec![base.join("plugins"), base.clone()]
        };

        for dir in candidates {
            let Ok(entries) = std::fs::read_dir(&dir) else {
                continue;
            };
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    let manifest_file = path.join("plugin.toml");
                    if manifest_file.is_file() {
                        if let Ok(plugin) = ExternalPlugin::load_file(&manifest_file) {
                            loaded.push(plugin);
                        }
                    }
                }
            }
        }
    }
    loaded
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manifest::PluginHeader;

    struct DummyCtx {
        status: String,
        workspace: PathBuf,
        active_file: Option<PathBuf>,
        buffer_text: String,
    }

    impl PluginCtx for DummyCtx {
        fn set_status(&mut self, msg: &str) {
            self.status = msg.to_string();
        }
        fn workspace_root(&self) -> &Path {
            &self.workspace
        }
        fn active_path(&self) -> Option<PathBuf> {
            self.active_file.clone()
        }
        fn active_buffer_text(&self) -> String {
            self.buffer_text.clone()
        }
        fn active_is_dirty(&self) -> bool {
            false
        }
    }

    #[test]
    fn external_plugin_execution_echo() {
        let manifest = PluginManifest {
            plugin: PluginHeader {
                name: "echo-plugin".to_string(),
                version: "1.0".to_string(),
                description: "echo test".to_string(),
            },
            commands: vec![PluginCommandDef {
                id: "echo_cmd".to_string(),
                label: "Plugin: Echo Test".to_string(),
                description: "Runs echo".to_string(),
                executable: "echo".to_string(),
                args: vec!["Hello".to_string(), "$FILE".to_string()],
            }],
            hooks: Some(PluginHooksDef {
                on_save: Some(HookCommandDef {
                    executable: "echo".to_string(),
                    args: vec!["Saved".to_string()],
                }),
                on_open: None,
            }),
        };

        let plugin = ExternalPlugin::from_manifest(manifest, PathBuf::from("."));
        assert_eq!(plugin.name(), "echo-plugin");
        assert_eq!(plugin.commands().len(), 1);

        let mut ctx = DummyCtx {
            status: String::new(),
            workspace: PathBuf::from("."),
            active_file: Some(PathBuf::from("test.rs")),
            buffer_text: "fn main() {}".to_string(),
        };

        // Test running command
        let res = plugin.run_command("echo_cmd", &mut ctx);
        assert!(res.is_ok());
        assert!(ctx.status.contains("Hello test.rs"));

        // Test running hook
        plugin.on_hook(PluginHook::OnSave, &mut ctx);
        // Hook was executed cleanly without error
    }

    #[test]
    fn external_plugin_fail_closed() {
        let manifest = PluginManifest {
            plugin: PluginHeader {
                name: "nonexistent-cmd".to_string(),
                version: "1.0".to_string(),
                description: "nonexistent executable".to_string(),
            },
            commands: vec![PluginCommandDef {
                id: "bad_cmd".to_string(),
                label: "Plugin: Bad".to_string(),
                description: "".to_string(),
                executable: "nonexistent_executable_12345_xyz".to_string(),
                args: vec![],
            }],
            hooks: None,
        };

        let plugin = ExternalPlugin::from_manifest(manifest, PathBuf::from("."));
        let mut ctx = DummyCtx {
            status: String::new(),
            workspace: PathBuf::from("."),
            active_file: None,
            buffer_text: String::new(),
        };

        let res = plugin.run_command("bad_cmd", &mut ctx);
        assert!(res.is_err());
    }
}
