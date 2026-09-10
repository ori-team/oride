//! Task runner integrado: carrega e resolve tarefas declaradas em `tasks.toml`.

use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskDef {
    pub name: String,
    pub command: String,
    #[serde(default)]
    pub key: Option<String>,
    #[serde(default = "default_run_in")]
    pub run_in: String,
}

fn default_run_in() -> String {
    "terminal".to_string()
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct TasksFile {
    #[serde(default)]
    pub tasks: Vec<TaskDef>,
}

impl TasksFile {
    pub fn from_toml(content: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(content)
    }
}

impl std::str::FromStr for TasksFile {
    type Err = toml::de::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_toml(s)
    }
}

/// Substitui variáveis mágicas no comando da tarefa:
/// - `$FILE`: caminho completo do arquivo ativo
/// - `$FILE_NAME`: nome do arquivo com extensão
/// - `$FILE_STEM`: nome do arquivo sem extensão
/// - `$FILE_DIR`: pasta do arquivo
/// - `$WORKSPACE`: pasta raiz do projeto
/// - `$LINE`: linha atual do cursor (1-based)
/// - `$COL`: coluna atual do cursor (1-based)
#[must_use]
pub fn resolve_task_variables(
    command_template: &str,
    file: Option<&Path>,
    workspace: &Path,
    cursor: (usize, usize),
) -> String {
    let file_str = file.map_or_else(String::new, |p| p.to_string_lossy().to_string());
    let file_name = file
        .and_then(|p| p.file_name())
        .map_or_else(String::new, |n| n.to_string_lossy().to_string());
    let file_stem = file
        .and_then(|p| p.file_stem())
        .map_or_else(String::new, |s| s.to_string_lossy().to_string());
    let file_dir = file
        .and_then(|p| p.parent())
        .map_or_else(String::new, |d| d.to_string_lossy().to_string());
    let workspace_str = workspace.to_string_lossy().to_string();
    let line_str = (cursor.0 + 1).to_string();
    let col_str = (cursor.1 + 1).to_string();

    command_template
        .replace("$FILE_NAME", &file_name)
        .replace("$FILE_STEM", &file_stem)
        .replace("$FILE_DIR", &file_dir)
        .replace("$FILE", &file_str)
        .replace("$WORKSPACE", &workspace_str)
        .replace("$LINE", &line_str)
        .replace("$COL", &col_str)
}

/// Descobre e carrega tarefas do usuário (`~/.config/oride/tasks.toml`)
/// e do projeto (`<workspace>/.oride/tasks.toml`).
#[must_use]
pub fn discover_tasks(workspace_hint: Option<&Path>) -> Vec<TaskDef> {
    let mut tasks = Vec::new();

    // 1. Global do usuário
    if let Some(config_dir) = dirs::config_dir() {
        let global_path = config_dir.join("oride").join("tasks.toml");
        if global_path.is_file() {
            if let Ok(content) = std::fs::read_to_string(&global_path) {
                if let Ok(file) = TasksFile::from_toml(&content) {
                    tasks.extend(file.tasks);
                }
            }
        }
    }

    // 2. Projeto local
    if let Some(ws) = workspace_hint {
        let project_path = ws.join(".oride").join("tasks.toml");
        if project_path.is_file() {
            if let Ok(content) = std::fs::read_to_string(&project_path) {
                if let Ok(file) = TasksFile::from_toml(&content) {
                    tasks.extend(file.tasks);
                }
            }
        }
    }

    tasks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_tasks_toml() {
        let toml = r#"
[[tasks]]
name = "Cargo Run"
command = "cargo run"
key = "F5"
run_in = "terminal"

[[tasks]]
name = "Cargo Test"
command = "cargo test"
key = "F6"
"#;
        let file = TasksFile::from_toml(toml).expect("parse tasks");
        assert_eq!(file.tasks.len(), 2);
        assert_eq!(file.tasks[0].name, "Cargo Run");
        assert_eq!(file.tasks[0].key, Some("F5".into()));
        assert_eq!(file.tasks[1].run_in, "terminal");
    }

    #[test]
    fn resolve_magic_variables() {
        let file = Path::new("/workspace/src/main.rs");
        let ws = Path::new("/workspace");
        let cmd = "gcc -O2 $FILE -o $FILE_DIR/$FILE_STEM -I$WORKSPACE --at $LINE:$COL";
        let resolved = resolve_task_variables(cmd, Some(file), ws, (10, 4));
        assert_eq!(
            resolved,
            "gcc -O2 /workspace/src/main.rs -o /workspace/src/main -I/workspace --at 11:5"
        );
    }
}
