//! Definições de tipos centrais e estrutura do estado do editor Oride.

use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Instant;

use oride_config::Config;
use oride_core::{ByteOffset, DocumentId, DocumentStore};
use oride_fs::ProjectTree;
use oride_git::GitFileStatus;
use oride_i18n::Locale;
use oride_keymap::{Action, Keymap};
use oride_lsp::{Diagnostic, LspClient};
use oride_plugin::PluginHost;
use oride_search::SearchHit;
use oride_syntax::{HighlightEngine, LanguageId, PreviewLine};
use oride_terminal::EmbeddedTerminal;
use oride_ui::{MenuColumn, UiTheme};
use ratatui::layout::Position;

use crate::browser::PathBrowser;
use crate::disk_watch::DiskWatch;
use crate::find::FindState;
use crate::jump_list::JumpList;
use crate::mouse::HitRegions;
use crate::split::SplitState;

/// Comando aplicado ao documento (inclui inserção de caractere).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyCommand {
    Action(Action),
    InsertChar(char),
}

/// Foco ativo de interação nos painéis da TUI.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Focus {
    Editor,
    Tree,
    Terminal,
    Scm,
}

/// Sobreposição modal ou menu ativo sobre o editor principal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Overlay {
    None,
    CommandPalette {
        query: String,
        selected: usize,
    },
    /// Navegador de pastas (workspace) ou arquivos.
    Browse(PathBrowser),
    Prompt {
        kind: PromptKind,
        buffer: String,
    },
    /// Lista completa de atalhos (filtro + scroll).
    Help {
        query: String,
        selected: usize,
    },
    /// Find/replace compacto (barra no rodapé; estado em `App.find`).
    Find,
    /// Busca no projeto (Ctrl+Shift+F).
    ProjectFind {
        query: String,
        selected: usize,
        case_sensitive: bool,
        use_regex: bool,
        hits: Vec<SearchHit>,
        status: String,
        replace_query: Option<String>,
        file_glob: Option<String>,
        focus_field: u8,
    },
    Diagnostics {
        selected: usize,
    },
    Completion {
        items: Vec<CompletionChoice>,
        selected: usize,
        replace_start: usize,
    },
    Hover {
        text: String,
    },
    /// Arquivo mudou no disco — Enter recarrega, Esc ignora.
    ReloadConfirm {
        path: PathBuf,
    },
    /// Lista de buffers abertos (fuzzy).
    BufferPicker {
        query: String,
        selected: usize,
    },
    /// Git diff somente leitura.
    Diff {
        path: PathBuf,
        lines: Vec<String>,
        scroll: usize,
    },
    /// Surround: aguarda caractere de fechamento.
    SurroundPick,
    /// Busca unificada: arquivos + buffers + comandos.
    MultiPicker {
        query: String,
        selected: usize,
    },
    /// Histórico de desfazer/refazer.
    UndoTree {
        selected: usize,
    },
    /// Seletor e preview interativo de temas de cores.
    ThemePicker {
        query: String,
        selected: usize,
        initial_theme: String,
        themes: Vec<String>,
    },
    /// Seletor interativo de idioma da interface.
    LocalePicker {
        query: String,
        selected: usize,
        locales: Vec<String>,
    },
    /// Linha de comando estilo Vim (:w, :q, :health, etc.).
    VimCommand {
        buffer: String,
    },
    /// Janela modal de diagnóstico de LSP e ambiente (:health).
    HealthCheck {
        scroll: usize,
        report: crate::health::HealthReport,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletionChoice {
    pub display: String,
    pub insert_text: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PromptKind {
    NewFile,
    NewDir,
    Rename,
    DeleteConfirm,
    Commit,
}

/// Estado global da aplicação Oride.
pub struct App {
    pub store: DocumentStore,
    pub scroll_y: usize,
    pub should_quit: bool,
    pub status_message: Option<String>,
    pub(crate) message_expires: Option<Instant>,
    pub(crate) quit_confirm_pending: bool,
    pub(crate) close_tab_confirm: Option<DocumentId>,
    pub theme: UiTheme,
    pub locale: Locale,
    pub show_line_numbers: bool,
    pub keymap: Keymap,
    pub config: Config,
    /// Altura útil do texto no editor (linhas visuais, sem bordas/tabs).
    pub(crate) last_editor_height: usize,
    /// Largura útil do texto (sem gutter) — para soft-wrap no scroll.
    pub(crate) last_editor_text_width: usize,
    pub focus: Focus,
    pub show_tree: bool,
    pub tree: Option<ProjectTree>,
    pub(crate) tree_scroll: usize,
    pub(crate) workspace: PathBuf,
    pub(crate) git_status: HashMap<PathBuf, GitFileStatus>,
    pub git_branch: Option<String>,
    pub git_ahead_behind: Option<(usize, usize)>,
    pub(crate) blame_target: Option<(PathBuf, usize)>,
    pub(crate) blame_cache: Option<String>,
    pub(crate) blame_refresh_at: Option<Instant>,
    pub(crate) use_nerd_icons: bool,
    pub tree_width: u16,
    pub(crate) terminal: Option<EmbeddedTerminal>,
    pub overlay: Overlay,
    pub(crate) file_index: Vec<PathBuf>,
    pub(crate) highlight: HighlightEngine,
    /// Soft wrap (default true em Markdown).
    pub(crate) soft_wrap: bool,
    pub(crate) show_md_preview: bool,
    pub(crate) preview_scroll: usize,
    pub(crate) find: FindState,
    pub(crate) disk_watch: DiskWatch,
    pub(crate) lsp_clients: HashMap<LanguageId, LspClient>,
    pub(crate) lsp_failures: HashMap<LanguageId, String>,
    pub(crate) diagnostics: Vec<(PathBuf, Diagnostic)>,
    pub(crate) show_diagnostics: bool,
    pub(crate) lsp_doc_version: i32,
    pub(crate) pending_reload: Option<PathBuf>,
    pub(crate) plugin_host: PluginHost,
    pub split: SplitState,
    /// Painel SCM à direita.
    pub show_scm: bool,
    pub(crate) scm_selected: usize,
    pub(crate) scm_width: u16,
    pub(crate) scm_cache: Vec<(GitFileStatus, PathBuf)>,
    /// Menu bar: Some((menu_index, item_index)).
    pub(crate) menu_open: Option<(usize, usize)>,
    pub(crate) show_which_key: bool,
    pub(crate) show_welcome: bool,
    pub(crate) jump_list: JumpList,
    pub(crate) menus: Vec<MenuColumn>,
    /// Último layout para hit-test do mouse.
    pub hit_regions: HitRegions,
    pub mouse_enabled: bool,
    /// Âncora de drag (byte) enquanto botão esquerdo pressionado.
    pub(crate) mouse_drag_anchor: Option<ByteOffset>,
    pub(crate) splitter_drag: Option<SplitterDrag>,
    pub(crate) last_click: Option<(Instant, u16, u16)>,
    pub(crate) click_count: u8,
    /// Aguarda tecla de par para surround.
    pub(crate) surround_pending: bool,
    /// Cache de linhas de preview Markdown (doc_id, version, terminal_images, lines).
    pub cached_preview: Option<(DocumentId, u64, bool, Vec<PreviewLine>)>,
    /// Posição do cursor no terminal/área do editor ativo (usada para ancorar popups).
    pub(crate) focused_cursor_pos: Option<Position>,
    /// Estado do modo modal Vim (Normal, Insert, Visual, etc.).
    pub vim: Option<crate::modal::VimState>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SplitterDrag {
    Tree {
        start_x: u16,
        initial_width: u16,
    },
    Editor {
        start_coord: u16,
        initial_ratio: u16,
        total_size: u16,
        orientation: crate::split::SplitOrientation,
    },
}

pub(crate) fn build_default_keymap() -> Keymap {
    let defaults = Config::default();
    Keymap::from_string_map(
        defaults
            .keys
            .iter()
            .map(|(key, value)| (key.as_str(), value.as_str())),
    )
    .expect("atalhos padrão de teclado devem ser válidos")
}

pub fn fuzzy_match(query: &str, candidate: &str) -> bool {
    if query.is_empty() {
        return true;
    }
    let lower_query = query.to_ascii_lowercase();
    let lower_candidate = candidate.to_ascii_lowercase();
    if lower_candidate.contains(&lower_query) {
        return true;
    }
    let mut candidate_chars = lower_candidate.chars();
    for query_char in lower_query.chars() {
        loop {
            match candidate_chars.next() {
                Some(candidate_char) if candidate_char == query_char => break,
                Some(_) => continue,
                None => return false,
            }
        }
    }
    true
}
