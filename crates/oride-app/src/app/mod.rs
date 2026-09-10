//! Estado da aplicação, ciclo de vida e despacho de comandos.

pub mod actions;
pub mod input;
pub mod lsp;
pub mod mouse_handler;
pub mod render;
pub mod state;

#[cfg(test)]
mod tests;

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use oride_config::{resolve_indent_for_file, Config, EditorIndent};
use oride_core::{DocumentError, DocumentId, DocumentStore};
use oride_fs::{list_files_recursive, ProjectTree};
use oride_git::{ahead_behind, blame_line, current_branch, scm_entries, status_map};
use oride_keymap::Keymap;
use oride_plugin::builtin_host;
use oride_syntax::{detect_language, HighlightEngine};
use oride_terminal::EmbeddedTerminal;
use oride_ui::UiTheme;

#[allow(unused_imports)]
pub use state::{fuzzy_match, App, CompletionChoice, Focus, KeyCommand, Overlay, PromptKind};

use crate::disk_watch::DiskWatch;
use crate::find::FindState;
use crate::jump_list::JumpList;
use crate::mouse::HitRegions;
use crate::session::{Session, SplitSession};
use crate::split::{SplitOrientation, SplitState};

use self::state::build_default_keymap;

impl App {
    pub fn new_empty() -> Self {
        let workspace = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let mut store = DocumentStore::new();
        store.open_empty();
        Self::from_store(store, workspace)
    }

    pub fn open_path(path: PathBuf) -> Result<Self, DocumentError> {
        if path.is_dir() {
            return Self::open_workspace(path);
        }
        let workspace = path
            .parent()
            .map(Path::to_path_buf)
            .unwrap_or_else(|| PathBuf::from("."));
        let workspace = std::fs::canonicalize(&workspace).unwrap_or(workspace);
        let mut store = DocumentStore::new();
        store.open_path(&path)?;
        let mut app = Self::from_store(store, workspace);
        app.apply_language_defaults(detect_language(Some(path.as_path())));
        app.apply_editorconfig_for_active();
        app.lsp_open_active();
        Ok(app)
    }

    pub fn open_workspace(directory: PathBuf) -> Result<Self, DocumentError> {
        let workspace = std::fs::canonicalize(&directory).unwrap_or(directory);
        let mut store = DocumentStore::new();
        store.open_empty();
        Ok(Self::from_store(store, workspace))
    }

    pub fn from_store_with_config(
        store: DocumentStore,
        config: Config,
        workspace: PathBuf,
    ) -> Self {
        let theme = UiTheme::from_config_parts(&config.theme_ui, &config.syntax)
            .unwrap_or_else(|_| UiTheme::default());
        let keymap = Keymap::from_string_map(
            config
                .keys
                .iter()
                .map(|(key, value)| (key.as_str(), value.as_str())),
        )
        .unwrap_or_else(|_| build_default_keymap());

        let show_hidden = config.tree.show_hidden;
        let tree = ProjectTree::open(&workspace, show_hidden).ok();
        let git_status = if config.tree.git_status {
            status_map(&workspace)
        } else {
            HashMap::new()
        };
        let git_branch = current_branch(&workspace);
        let git_ahead_behind = ahead_behind(&workspace);
        let file_index = list_files_recursive(&workspace, show_hidden).unwrap_or_default();

        let terminal_height = config.terminal.default_height.max(3);
        let terminal = EmbeddedTerminal::spawn(
            &workspace,
            80,
            terminal_height,
            Some(&config.terminal.shell),
        )
        .ok()
        .map(|mut terminal_instance| {
            terminal_instance.height_lines = terminal_height;
            terminal_instance
        });

        let disk_watch = DiskWatch::start(&workspace);
        let active_document_id = store.active_id().unwrap_or_else(|| DocumentId::from_raw(0));
        let scm_cache = if config.tree.git_status {
            scm_entries(&workspace)
        } else {
            Vec::new()
        };

        let mut plugin_host = builtin_host();
        for lang in &config.languages {
            let mut dynamic = oride_plugin::DynamicLang::new(&lang.id, lang.extensions.clone());
            if let Some(name) = &lang.name {
                dynamic.name = name.clone();
            }
            dynamic.filenames = lang.filenames.clone();
            dynamic.comment_open = lang.line_comment.clone();
            dynamic.comment_close = lang.block_comment_close.clone();
            dynamic.lsp_command = lang.lsp_command.clone();
            dynamic.tab_size = lang.tab_size;
            dynamic.insert_spaces = lang.insert_spaces;
            dynamic.soft_wrap = lang.soft_wrap;
            dynamic.completion_words = lang.completion_words.clone();
            plugin_host.add_dynamic_language(dynamic);
        }

        let mut plugin_dirs = Vec::new();
        if let Some(user_dir) = dirs::config_dir().map(|d| d.join("oride").join("plugins")) {
            plugin_dirs.push(user_dir);
        }
        plugin_dirs.push(workspace.join(".oride").join("plugins"));
        plugin_dirs.push(workspace.join(".oride"));
        let external_plugins = oride_plugin::discover_external_plugins(&plugin_dirs);
        for ext in external_plugins {
            plugin_host.add_plugin(Box::new(ext));
        }

        oride_i18n::reload_locales(Some(&workspace));
        let locale = oride_i18n::Locale::from_str_loose(&config.locale);
        let menus = crate::menus::menus_for_locale(&locale);
        let vim = if config.editor.modal_mode {
            Some(crate::modal::VimState::default())
        } else {
            None
        };

        Self {
            store,
            scroll_y: 0,
            should_quit: false,
            status_message: None,
            message_expires: None,
            quit_confirm_pending: false,
            close_tab_confirm: None,
            theme,
            locale,
            show_line_numbers: config.show_line_numbers,
            keymap,
            config: config.clone(),
            last_editor_height: 20,
            last_editor_text_width: 80,
            focus: Focus::Editor,
            show_tree: true,
            tree,
            tree_scroll: 0,
            workspace,
            git_status,
            git_branch,
            git_ahead_behind,
            blame_target: None,
            blame_cache: None,
            blame_refresh_at: None,
            use_nerd_icons: true,
            tree_width: config.tree.width.max(8),
            terminal,
            overlay: Overlay::None,
            file_index,
            highlight: HighlightEngine::new(),
            soft_wrap: config.soft_wrap,
            show_md_preview: false,
            preview_scroll: 0,
            find: FindState::default(),
            disk_watch,
            lsp_clients: HashMap::new(),
            lsp_failures: HashMap::new(),
            diagnostics: Vec::new(),
            show_diagnostics: false,
            lsp_doc_version: 1,
            pending_reload: None,
            plugin_host,
            split: SplitState::single(active_document_id),
            show_scm: false,
            scm_selected: 0,
            scm_width: 28,
            scm_cache,
            menu_open: None,
            show_which_key: false,
            show_welcome: false,
            jump_list: JumpList::default(),
            menus,
            hit_regions: HitRegions::default(),
            mouse_enabled: config.mouse,
            mouse_drag_anchor: None,
            splitter_drag: None,
            last_click: None,
            click_count: 0,
            surround_pending: false,
            cached_preview: None,
            focused_cursor_pos: None,
            vim,
        }
    }

    /// Restaura uma instância de `App` a partir de uma `Session`.
    pub fn from_session(session: Session) -> Result<Self, DocumentError> {
        if !session.workspace.is_dir() {
            return Err(DocumentError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "workspace directory does not exist",
            )));
        }
        let mut app = Self::open_workspace(session.workspace.clone())?;
        for file_path in &session.files {
            if file_path.is_file() {
                let _ = app.open_document_path(file_path);
            }
        }
        let open_paths = app.store.open_paths();
        if !open_paths.is_empty() {
            let active_index = session.active_index.min(open_paths.len() - 1);
            let _ = app.open_document_path(&open_paths[active_index]);
        }
        if let Some(tree_width) = session.tree_width {
            app.tree_width = tree_width;
        }
        if let Some(show_tree) = session.show_tree {
            app.show_tree = show_tree;
        }
        if let Some(show_scm) = session.show_scm {
            app.show_scm = show_scm;
        }
        if let Some(split_session) = session.split {
            let orientation = if split_session.orientation == "horizontal" {
                SplitOrientation::Horizontal
            } else {
                SplitOrientation::Vertical
            };
            let secondary_doc_id = if let Some(ref sec_path) = split_session.secondary_file {
                if let Some(existing_id) = app.store.document_id_for_path(sec_path) {
                    Some(existing_id)
                } else if sec_path.is_file() {
                    let _ = app.open_document_path(sec_path);
                    app.store.document_id_for_path(sec_path)
                } else {
                    app.store.active_id()
                }
            } else {
                app.store.active_id()
            };
            if let Some(doc_id) = secondary_doc_id {
                app.split.split(orientation, doc_id);
                app.split.ratio_percent = split_session.ratio_percent;
                app.split.focused = 0;
            }
        }
        app.scroll_y = session.scroll_y;
        app.split.sync_scroll(session.scroll_y);
        app.show_welcome = false;
        app.set_status("sessão restaurada");
        Ok(app)
    }

    /// Restaura sessão salva se existir; senão buffer vazio no CWD.
    pub fn new_empty_or_session() -> Self {
        let current_directory = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let loaded_session = Session::load_for_workspace(&current_directory).or_else(Session::load);
        if let Some(session) = loaded_session {
            if let Ok(app) = Self::from_session(session) {
                return app;
            }
        }
        let mut app = Self::new_empty();
        app.show_welcome = true;
        app
    }

    pub fn persist_session(&self) {
        let open_files = self.store.open_paths();
        let active_index = self
            .store
            .active()
            .ok()
            .and_then(|document| {
                document
                    .path()
                    .map(|doc_path| open_files.iter().position(|file| file == doc_path))
            })
            .flatten()
            .unwrap_or(0);
        let split_session = if self.split.is_split() {
            let orientation = match self.split.orientation {
                SplitOrientation::Vertical => "vertical".to_string(),
                SplitOrientation::Horizontal => "horizontal".to_string(),
            };
            let secondary_file = self.split.panes.get(1).and_then(|pane| {
                self.store
                    .get(pane.doc_id)
                    .and_then(|doc| doc.path().map(Path::to_path_buf))
            });
            Some(SplitSession {
                orientation,
                secondary_file,
                ratio_percent: self.split.ratio_percent,
            })
        } else {
            None
        };

        let session = Session {
            workspace: self.workspace.clone(),
            files: open_files,
            active_index,
            scroll_y: self.scroll_y,
            tree_width: Some(self.tree_width),
            show_tree: Some(self.show_tree),
            show_scm: Some(self.show_scm),
            split: split_session,
        };
        let _ = session.save();
    }

    fn from_store(store: DocumentStore, workspace: PathBuf) -> Self {
        let config = oride_config::load_merged(Some(workspace.as_path())).unwrap_or_default();
        Self::from_store_with_config(store, config, workspace)
    }

    pub fn set_status(&mut self, message: impl Into<String>) {
        self.status_message = Some(message.into());
        self.message_expires = Some(Instant::now() + Duration::from_secs(3));
    }

    pub fn tick(&mut self) {
        self.tick_messages();
        if let Some(terminal) = self.terminal.as_mut() {
            terminal.poll_output();
        }
        self.poll_disk_changes();
        self.poll_lsp_events();
        self.refresh_blame_cache();
    }

    pub(crate) fn poll_disk_changes(&mut self) {
        let changed = self.disk_watch.poll();
        for path in changed {
            let Some(id) = self.store.document_id_for_path(&path) else {
                continue;
            };
            let dirty = self
                .store
                .get(id)
                .is_some_and(|document| document.is_dirty());
            if dirty {
                self.pending_reload = Some(path.clone());
                self.overlay = Overlay::ReloadConfirm { path };
                self.set_status("arquivo mudou no disco — Enter recarrega · Esc ignora");
                break;
            }
            match self.reload_document(id) {
                Ok(()) => self.set_status(format!("recarregado: {}", path.display())),
                Err(error) => self.set_status(format!("reload: {error}")),
            }
        }
    }

    pub(crate) fn reload_document(&mut self, id: DocumentId) -> Result<(), DocumentError> {
        self.store
            .get_mut(id)
            .ok_or(DocumentError::NotFound(id))?
            .reload_from_disk()?;
        self.lsp_sync_document(id);
        Ok(())
    }

    pub(crate) fn notify_saved_path(&mut self, path: &Path) {
        self.disk_watch.mark_saved(path);
        let text = self
            .store
            .document_id_for_path(path)
            .and_then(|id| self.store.get(id))
            .map(|document| document.buffer().as_string());
        let language = detect_language(Some(path));
        if let (Some(client), Some(text)) = (self.lsp_clients.get_mut(&language), text) {
            let _ = client.did_save(path, &text);
        }
    }

    pub(crate) fn apply_editorconfig_for_active(&mut self) {
        if !self.config.editor.use_editorconfig {
            return;
        }
        let Ok(document) = self.store.active() else {
            return;
        };
        let Some(path) = document.path() else {
            return;
        };
        let indent = resolve_indent_for_file(
            path,
            EditorIndent {
                tab_size: self.config.editor.tab_size,
                insert_spaces: self.config.editor.insert_spaces,
            },
        );
        self.config.editor.tab_size = indent.tab_size;
        self.config.editor.insert_spaces = indent.insert_spaces;
    }

    pub fn tick_messages(&mut self) {
        if let Some(expires_at) = self.message_expires {
            if Instant::now() >= expires_at {
                self.status_message = None;
                self.message_expires = None;
            }
        }
    }

    pub(crate) fn refresh_blame_cache(&mut self) {
        let target = self.store.active().ok().and_then(|document| {
            Some((
                document.path()?.to_path_buf(),
                document.caret().ok()?.line + 1,
            ))
        });
        if target != self.blame_target {
            self.blame_target = target;
            self.blame_cache = None;
            self.blame_refresh_at = Some(Instant::now() + Duration::from_millis(150));
            return;
        }
        let Some(refresh_at) = self.blame_refresh_at else {
            return;
        };
        if Instant::now() < refresh_at {
            return;
        }
        self.blame_refresh_at = None;
        self.blame_cache = self
            .blame_target
            .as_ref()
            .and_then(|(path, line)| blame_line(&self.workspace, path, *line));
    }

    pub(crate) fn ensure_tree_visible(&mut self) {
        let Some(tree) = &self.tree else { return };
        let selected = tree.selected_index();
        if selected < self.tree_scroll {
            self.tree_scroll = selected;
        } else if selected >= self.tree_scroll + 20 {
            self.tree_scroll = selected.saturating_sub(19);
        }
    }
}
