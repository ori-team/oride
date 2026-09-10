//! Renderização da interface TUI (Ratatui), cálculo de viewport e layouts.

use std::path::Path;

use oride_search::format_hit_label;
use oride_syntax::{detect_language, render_preview_lines_with_config};
use oride_ui::{
    menu_dropdown_rect, render_completion_popup, render_context_banner, render_editor,
    render_find_modal, render_md_preview, render_menu_bar, render_menu_dropdown, render_mini_modal,
    render_palette, render_project_find, render_scm_panel, render_status, render_tabs,
    render_terminal_panel, render_tree, render_which_key, CompletionPopupView, EditorView,
    FindModalView, MdPreviewView, MiniModalView, PaletteView, ProjectFindView, ScmItem,
    StatusModel, TreeView,
};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::Frame;

use crate::split::SplitOrientation;

use super::state::{fuzzy_match, App, Focus, Overlay, PromptKind};

impl App {
    pub fn draw(&mut self, frame: &mut Frame) {
        self.tick();
        let area = frame.area();

        let terminal_height = self
            .terminal
            .as_ref()
            .filter(|term| term.visible)
            .map(|term| term.height_lines.min(area.height.saturating_sub(5)).max(3))
            .unwrap_or(0);

        // menu(1) + banner(1) + body + term + status(1)
        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1), // menu bar
                Constraint::Length(1), // context banner
                Constraint::Min(1),
                Constraint::Length(terminal_height),
                Constraint::Length(1), // status
            ])
            .split(area);

        let menu_area = main_chunks[0];
        let banner_area = main_chunks[1];
        let body_area = main_chunks[2];
        let terminal_area = main_chunks[3];
        let status_area = main_chunks[4];

        let open_menu = self.menu_open.map(|(index, _)| index);
        render_menu_bar(frame, menu_area, &self.menus, open_menu);
        self.hit_regions.menu = menu_area;

        let banner_hint = match self.focus {
            Focus::Editor => "F1 help · Ctrl+Shift+P cmds · Alt+F menu",
            Focus::Tree => "↑↓ Enter · Ctrl+E editor",
            Focus::Terminal => "digite · Esc=editor · Ctrl+C shell",
            Focus::Scm => "↑↓ Enter · s=stage · u=unstage · c=commit · p=pull · P=push · Esc",
        };
        render_context_banner(frame, banner_area, self.focus_label(), banner_hint);

        let tree_width = if self.show_tree {
            self.tree_width.min(body_area.width / 2).max(12)
        } else {
            0
        };
        let scm_width = if self.show_scm {
            self.scm_width.min(body_area.width / 3).max(16)
        } else {
            0
        };

        let body_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(tree_width),
                Constraint::Min(1),
                Constraint::Length(scm_width),
            ])
            .split(body_area);

        if tree_width > 0 {
            self.hit_regions.tree = Some(body_chunks[0]);
            let splitter_x = body_chunks[0]
                .x
                .saturating_add(body_chunks[0].width)
                .saturating_sub(1);
            self.hit_regions.tree_splitter = Some(Rect::new(
                splitter_x,
                body_chunks[0].y,
                2,
                body_chunks[0].height,
            ));
            self.draw_tree(frame, body_chunks[0]);
        } else {
            self.hit_regions.tree = None;
            self.hit_regions.tree_splitter = None;
        }
        self.draw_editor_column(frame, body_chunks[1]);
        if scm_width > 0 {
            self.hit_regions.scm = Some(body_chunks[2]);
            self.draw_scm(frame, body_chunks[2]);
        } else {
            self.hit_regions.scm = None;
        }

        if terminal_height > 0 {
            self.hit_regions.terminal = Some(terminal_area);
            if let Some(term) = self.terminal.as_mut() {
                term.poll_output();
                let columns = terminal_area.width.max(20);
                let rows = terminal_area.height.max(2);
                term.resize(columns, rows);
                let lines = term.visible_lines(rows as usize);
                let last_error = term.last_error.clone();
                render_terminal_panel(
                    frame,
                    terminal_area,
                    &lines,
                    self.focus == Focus::Terminal,
                    &self.theme,
                    last_error.as_deref(),
                    Some(term.cursor_col()),
                );
            }
        } else {
            self.hit_regions.terminal = None;
        }

        self.draw_status(frame, status_area);

        // Dropdown menu por cima
        if let Some((menu_index, item_index)) = self.menu_open {
            self.hit_regions.menu_dropdown = menu_dropdown_rect(area, &self.menus, menu_index);
            render_menu_dropdown(frame, area, &self.menus, menu_index, item_index);
        } else {
            self.hit_regions.menu_dropdown = None;
        }

        // Modais e sobreposições
        self.draw_overlays(frame, area);
    }

    pub(crate) fn draw_overlays(&self, frame: &mut Frame, area: Rect) {
        match &self.overlay {
            Overlay::None => {}
            Overlay::CommandPalette { query, selected } => {
                let items = self.command_palette_items(query);
                let view = PaletteView {
                    title: "commands",
                    query,
                    items: &items,
                    selected: *selected,
                    hint: "↑↓ · Enter executa · Esc",
                };
                render_palette(frame, area, &view, &self.theme);
            }
            Overlay::Browse(browser) => {
                let items = browser.list_labels();
                let query_display = browser.query_display();
                let title = browser.title();
                let view = PaletteView {
                    title: &title,
                    query: &query_display,
                    items: &items,
                    selected: browser.selected_index_for_display(),
                    hint: browser.hint(),
                };
                render_palette(frame, area, &view, &self.theme);
            }
            Overlay::Prompt { kind, buffer } => {
                let title = match kind {
                    PromptKind::NewFile => "novo arquivo",
                    PromptKind::NewDir => "nova pasta",
                    PromptKind::Rename => "renomear item",
                    PromptKind::DeleteConfirm => "confirmar exclusão",
                    PromptKind::Commit => "git commit (mensagem)",
                };
                let items: &[String] = &[];
                let view = PaletteView {
                    title,
                    query: buffer,
                    items,
                    selected: 0,
                    hint: "Enter confirma · Esc cancela",
                };
                render_palette(frame, area, &view, &self.theme);
            }
            Overlay::Help { query, selected } => {
                let items = self.keybind_list_items(query);
                let title = format!(
                    "atalhos ({}/{}) — F1 · Ctrl+G · Ctrl+Shift+/",
                    items.len(),
                    self.keymap.len()
                );
                let view = PaletteView {
                    title: &title,
                    query,
                    items: &items,
                    selected: *selected,
                    hint: "↑↓ navegar · digite filtra · Enter/Esc/q fecha",
                };
                render_palette(frame, area, &view, &self.theme);
            }
            Overlay::Find => {
                let match_label = self.find.match_label();
                let view = FindModalView {
                    query: &self.find.query,
                    replace: &self.find.replace,
                    show_replace: self.find.show_replace,
                    focus_replace: self.find.focus_replace,
                    match_label: &match_label,
                    case_sensitive: self.find.case_sensitive,
                    ignore_accents: self.find.ignore_accents,
                    whole_word: self.find.whole_word,
                    use_regex: self.find.use_regex,
                    error: self.find.regex_error.as_deref(),
                };
                render_find_modal(frame, area, &view);
            }
            Overlay::ProjectFind {
                query,
                selected,
                hits,
                status,
                replace_query,
                file_glob,
                focus_field,
                ..
            } => {
                let items: Vec<String> = hits
                    .iter()
                    .map(|hit| format_hit_label(hit, &self.workspace))
                    .collect();
                let title = if replace_query.is_some() {
                    format!("find & replace in project ({status})")
                } else {
                    format!("find in project ({status})")
                };
                let hint = if replace_query.is_some() {
                    "↑↓ · Tab campo · Alt+G glob · Enter/Alt+Enter substitui tudo · Alt+C case · Alt+R regex · Esc"
                } else {
                    "↑↓ · Enter abre · Tab Replace · Alt+G glob · Alt+C case · Alt+R regex · Esc"
                };
                let view = ProjectFindView {
                    title: &title,
                    query,
                    replace_query: replace_query.as_deref(),
                    file_glob: file_glob.as_deref(),
                    focus_field: *focus_field,
                    items: &items,
                    selected: *selected,
                    hint,
                };
                render_project_find(frame, area, &view, &self.theme);
            }
            Overlay::Diagnostics { selected } => {
                let items: Vec<String> = self
                    .diagnostics
                    .iter()
                    .map(|(path, diagnostic)| {
                        format!(
                            "L{}:{}  {}  {}",
                            diagnostic.range.start.line + 1,
                            diagnostic.range.start.character + 1,
                            path.file_name()
                                .and_then(|name| name.to_str())
                                .unwrap_or("?"),
                            diagnostic.message
                        )
                    })
                    .collect();
                let view = PaletteView {
                    title: "diagnostics (Enter jump · Esc)",
                    query: "",
                    items: &items,
                    selected: *selected,
                    hint: "↑↓ · Enter salta · Esc",
                };
                render_palette(frame, area, &view, &self.theme);
            }
            Overlay::Completion {
                items, selected, ..
            } => {
                let labels: Vec<String> = items.iter().map(|item| item.display.clone()).collect();
                let view = CompletionPopupView {
                    items: &labels,
                    selected: *selected,
                    cursor_pos: self.focused_cursor_pos,
                };
                render_completion_popup(frame, area, &view, &self.theme);
            }
            Overlay::Hover { text } => {
                let items: Vec<String> = text.lines().map(|line| line.to_string()).collect();
                let view = PaletteView {
                    title: "hover",
                    query: "",
                    items: &items,
                    selected: 0,
                    hint: "Esc fecha",
                };
                render_palette(frame, area, &view, &self.theme);
            }
            Overlay::ReloadConfirm { path } => {
                let items = [
                    format!("arquivo: {}", path.display()),
                    "Enter = recarregar do disco (descarta edições)".into(),
                    "Esc = manter buffer".into(),
                ];
                let view = PaletteView {
                    title: "arquivo mudou no disco",
                    query: "",
                    items: &items,
                    selected: 0,
                    hint: "Enter / Esc",
                };
                render_palette(frame, area, &view, &self.theme);
            }
            Overlay::BufferPicker { query, selected } => {
                let items: Vec<String> = self
                    .buffer_picker_items(query)
                    .into_iter()
                    .map(|(_, label)| label)
                    .collect();
                let view = PaletteView {
                    title: "buffers",
                    query,
                    items: &items,
                    selected: *selected,
                    hint: "↑↓ · Enter abre · digite filtra · Esc",
                };
                render_palette(frame, area, &view, &self.theme);
            }
            Overlay::Diff {
                path,
                lines,
                scroll,
            } => {
                let start = *scroll;
                let end = (start + 40).min(lines.len());
                let slice: Vec<String> = lines[start..end].to_vec();
                let title = format!("diff · {} · j/k scroll · Esc", path.display());
                let view = PaletteView {
                    title: &title,
                    query: "",
                    items: &slice,
                    selected: 0,
                    hint: "↑↓/jk · PgUp/PgDn · Esc fecha",
                };
                render_palette(frame, area, &view, &self.theme);
            }
            Overlay::SurroundPick => {
                let lines = vec![
                    "Selecione o delimitador:".into(),
                    "  (  [  {  <  \"  '  `".into(),
                    "Esc cancela".into(),
                ];
                let view = MiniModalView {
                    title: "Surround",
                    lines: &lines,
                    selected: 0,
                };
                render_mini_modal(frame, area, &view, &self.theme);
            }
            Overlay::MultiPicker { query, selected } => {
                let items = self.multi_picker_items(query);
                let view = PaletteView {
                    title: "multi-picker · buf | cmd | file",
                    query,
                    items: &items,
                    selected: *selected,
                    hint: "↑↓ · Enter · digite filtra · Esc",
                };
                render_palette(frame, area, &view, &self.theme);
            }
            Overlay::UndoTree { selected } => {
                let items = self
                    .store
                    .active()
                    .map(|document| document.undo_history_labels())
                    .unwrap_or_default();
                let view = PaletteView {
                    title: "undo history · Enter desfaz até o item",
                    query: "",
                    items: &items,
                    selected: *selected,
                    hint: "↑↓ · Enter · Esc",
                };
                render_palette(frame, area, &view, &self.theme);
            }
            Overlay::ThemePicker {
                query,
                selected,
                themes,
                ..
            } => {
                let filtered: Vec<String> = themes
                    .iter()
                    .filter(|t| fuzzy_match(query, t))
                    .cloned()
                    .collect();
                let msg = self.locale.messages();
                let view = PaletteView {
                    title: msg.theme_picker_title(),
                    query,
                    items: &filtered,
                    selected: *selected,
                    hint: msg.theme_picker_hint(),
                };
                render_palette(frame, area, &view, &self.theme);
            }
            Overlay::LocalePicker {
                query,
                selected,
                locales,
            } => {
                let filtered: Vec<String> = locales
                    .iter()
                    .filter(|l| fuzzy_match(query, l))
                    .cloned()
                    .collect();
                let msg = self.locale.messages();
                let view = PaletteView {
                    title: msg.locale_picker_title(),
                    query,
                    items: &filtered,
                    selected: *selected,
                    hint: msg.locale_picker_hint(),
                };
                render_palette(frame, area, &view, &self.theme);
            }
            Overlay::VimCommand { buffer } => {
                let bottom_row = Rect {
                    x: area.x,
                    y: area.y + area.height.saturating_sub(1),
                    width: area.width,
                    height: 1,
                };
                frame.render_widget(ratatui::widgets::Clear, bottom_row);
                let text = format!(":{}█", buffer);
                let p = ratatui::widgets::Paragraph::new(text).style(self.theme.status_style());
                frame.render_widget(p, bottom_row);
            }
            Overlay::HealthCheck { scroll, report } => {
                let mut lines = Vec::new();
                lines.push(format!(
                    "● Oride v0.2.0 | Tema: {} | Idioma: {} | Modo: {}",
                    report.active_theme,
                    report.active_locale,
                    if report.modal_mode {
                        "Modal (Vim)"
                    } else {
                        "Padrão (CUA)"
                    }
                ));
                lines.push(format!(
                    "● Gramáticas estáticas ({}): {}",
                    report.static_grammars.len(),
                    report.static_grammars.join(", ")
                ));
                lines.push(format!(
                    "● Gramáticas dinâmicas: {} ({})",
                    report.dynamic_grammar_count,
                    report
                        .dynamic_grammars_dir
                        .as_ref()
                        .map(|p| p.display().to_string())
                        .unwrap_or_else(|| "nenhum".into())
                ));
                lines.push("─".repeat(area.width.min(64) as usize));
                lines.push(format!(
                    "{:<12} {:<12} {:<10} {}",
                    "Linguagem", "Comando", "Status", "Caminho / Dica"
                ));
                lines.push("─".repeat(area.width.min(64) as usize));
                for item in &report.lsp_items {
                    let status = if item.installed {
                        "✓ OK"
                    } else {
                        "✗ Ausente"
                    };
                    let info = item
                        .path
                        .as_ref()
                        .map(|p| p.display().to_string())
                        .unwrap_or_else(|| item.install_hint.to_string());
                    lines.push(format!(
                        "{:<12} {:<12} {:<10} {}",
                        item.language, item.command, status, info
                    ));
                }
                let view = PaletteView {
                    title: "system & lsp health (:health)",
                    query: "",
                    items: &lines,
                    selected: *scroll,
                    hint: "↑↓/jk navega · Esc fecha",
                };
                render_palette(frame, area, &view, &self.theme);
            }
        }

        if self.show_which_key {
            let rows = self.which_key_rows();
            render_which_key(frame, area, "which-key", &rows);
        }
        if self.show_welcome {
            let lines = self.welcome_lines();
            let view = MiniModalView {
                title: "Oride · essentials",
                lines: &lines,
                selected: 0,
            };
            render_mini_modal(frame, area, &view, &self.theme);
        }
    }

    pub(crate) fn draw_tree(&mut self, frame: &mut Frame, area: Rect) {
        let Some(tree) = &self.tree else {
            return;
        };
        let rows = tree.flat_rows();
        let visible_rows = area.height.saturating_sub(2) as usize;
        let selected_index = tree.selected_index();
        if selected_index < self.tree_scroll {
            self.tree_scroll = selected_index;
        } else if visible_rows > 0 && selected_index >= self.tree_scroll + visible_rows {
            self.tree_scroll = selected_index + 1 - visible_rows;
        }
        let view = TreeView {
            title: tree.root_name(),
            rows: &rows,
            selected: tree.selected_index(),
            scroll: self.tree_scroll,
            use_nerd_icons: self.use_nerd_icons,
            git: &self.git_status,
            workspace_root: &self.workspace,
            focused: self.focus == Focus::Tree,
        };
        render_tree(frame, area, &view, &self.theme);
    }

    pub(crate) fn draw_editor_column(&mut self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Min(1)])
            .split(area);

        let tabs = self.store.tab_summaries();
        self.hit_regions.tabs = Some(chunks[0]);
        render_tabs(frame, chunks[0], &tabs, &self.theme);

        let (language, source) = match self.store.active() {
            Ok(document) => (
                detect_language(document.path()),
                document.buffer().as_string(),
            ),
            Err(_) => return,
        };
        self.highlight.update(language, &source);

        let show_preview = self.show_md_preview && language.is_markdown_family();
        let body_area = chunks[1];

        // Área do editor (pode ser split em 2 panes) + preview MD opcional
        let (editor_zone, preview_area) = if show_preview {
            let split = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(55), Constraint::Percentage(45)])
                .split(body_area);
            (split[0], Some(split[1]))
        } else {
            (body_area, None)
        };

        self.split.sync_scroll(self.scroll_y);
        let pane_areas: Vec<Rect> = if self.split.is_split() {
            let orientation = match self.split.orientation {
                SplitOrientation::Vertical => Direction::Horizontal,
                SplitOrientation::Horizontal => Direction::Vertical,
            };
            let first_percent = self.split.ratio_percent.clamp(15, 85);
            let second_percent = 100 - first_percent;
            let chunks = Layout::default()
                .direction(orientation)
                .constraints([
                    Constraint::Percentage(first_percent),
                    Constraint::Percentage(second_percent),
                ])
                .split(editor_zone);

            match self.split.orientation {
                SplitOrientation::Vertical => {
                    let splitter_x = chunks[0]
                        .x
                        .saturating_add(chunks[0].width)
                        .saturating_sub(1);
                    self.hit_regions.editor_splitter =
                        Some(Rect::new(splitter_x, chunks[0].y, 2, chunks[0].height));
                }
                SplitOrientation::Horizontal => {
                    let splitter_y = chunks[0]
                        .y
                        .saturating_add(chunks[0].height)
                        .saturating_sub(1);
                    self.hit_regions.editor_splitter =
                        Some(Rect::new(chunks[0].x, splitter_y, chunks[0].width, 2));
                }
            }

            chunks.to_vec()
        } else {
            self.hit_regions.editor_splitter = None;
            vec![editor_zone]
        };

        // Viewport real do texto (desconta borda do split + gutter)
        if let Some(active_area) = pane_areas.get(self.split.focused) {
            let border_width: u16 = if self.split.is_split() { 2 } else { 0 };
            let gutter = if self.show_line_numbers {
                self.theme
                    .gutter_width
                    .min(active_area.width.saturating_sub(border_width))
            } else {
                0
            };
            self.last_editor_height =
                active_area.height.saturating_sub(border_width).max(1) as usize;
            self.last_editor_text_width = active_area
                .width
                .saturating_sub(border_width)
                .saturating_sub(gutter)
                .max(1) as usize;
            let editor_rectangle = if self.split.is_split() {
                Rect {
                    x: active_area.x.saturating_add(1),
                    y: active_area.y.saturating_add(1),
                    width: active_area.width.saturating_sub(2),
                    height: active_area.height.saturating_sub(2),
                }
            } else {
                *active_area
            };
            self.hit_regions.editor = Some(editor_rectangle);
            self.hit_regions.gutter = gutter;
            self.hit_regions.text_width = self.last_editor_text_width as u16;
            self.hit_regions.soft_wrap = self.soft_wrap;
            self.hit_regions.scroll_y = self.scroll_y;
        }
        self.ensure_cursor_visible();
        self.split.sync_scroll(self.scroll_y);

        for (pane_index, pane_area) in pane_areas.iter().enumerate() {
            let pane = &self.split.panes[pane_index.min(self.split.panes.len() - 1)];
            let Some(document) = self.store.get(pane.doc_id) else {
                continue;
            };
            let caret = document.caret().unwrap_or_default();
            let selection = document.selection();
            let extra_carets: Vec<_> = document
                .extra_carets()
                .iter()
                .filter_map(|offset| document.buffer().byte_to_caret(*offset).ok())
                .collect();
            // Evita re-highlight e alocação de string em buffers secundários não focados
            let highlights = if pane_index == self.split.focused {
                let pane_language = detect_language(document.path());
                let pane_source = document.buffer().as_string();
                self.highlight.update(pane_language, &pane_source);
                self.highlight.spans()
            } else {
                &[]
            };
            let view = EditorView {
                buffer: document.buffer(),
                caret,
                selection,
                extra_carets: &extra_carets,
                scroll_y: pane.scroll_y,
                show_line_numbers: self.show_line_numbers,
                highlights,
                show_cursor: self.focus == Focus::Editor
                    && (matches!(self.overlay, Overlay::None)
                        || matches!(self.overlay, Overlay::Completion { .. }))
                    && pane_index == self.split.focused,
                soft_wrap: self.soft_wrap,
                focused_pane: self.split.is_split() && pane_index == self.split.focused,
            };
            let cursor = render_editor(frame, *pane_area, &view, &self.theme);
            if pane_index == self.split.focused {
                self.focused_cursor_pos = cursor;
            }
        }

        if let Some(preview_rect) = preview_area {
            self.hit_regions.preview = Some(preview_rect);
            let active_document = self.store.active().ok();
            let document_id = active_document
                .map(|doc| doc.id())
                .unwrap_or_else(|| oride_core::DocumentId::from_raw(0));
            let document_version = active_document.map(|doc| doc.version()).unwrap_or(0);
            let base_directory = active_document.and_then(|doc| {
                doc.path()
                    .and_then(|path| path.parent().map(Path::to_path_buf))
            });

            let is_cache_valid = match &self.cached_preview {
                Some((cached_id, cached_version, cached_term_images, _)) => {
                    *cached_id == document_id
                        && *cached_version == document_version
                        && *cached_term_images == self.config.markdown.terminal_images
                }
                None => false,
            };

            if !is_cache_valid {
                let rendered_lines = render_preview_lines_with_config(
                    &source,
                    base_directory.as_deref(),
                    self.config.markdown.terminal_images,
                );
                self.cached_preview = Some((
                    document_id,
                    document_version,
                    self.config.markdown.terminal_images,
                    rendered_lines,
                ));
            }

            let preview_lines = self
                .cached_preview
                .as_ref()
                .map(|(_, _, _, lines)| lines.as_slice())
                .unwrap_or(&[]);

            // Cards de imagem expandem linhas; segue scroll do editor + offset fino.
            let max_scroll = preview_lines.len().saturating_sub(1);
            let base_scroll = self.scroll_y.min(max_scroll);
            let scroll = base_scroll
                .saturating_add(self.preview_scroll)
                .min(max_scroll);
            let caret_line = self
                .store
                .active()
                .ok()
                .and_then(|doc| doc.caret().ok())
                .map(|c| c.line + 1)
                .unwrap_or(0);
            let title = format!("preview · L{caret_line} · 🖼=placeholder · Alt+↑/↓ · Alt+P");
            let view = MdPreviewView {
                title: &title,
                lines: preview_lines,
                scroll,
            };
            render_md_preview(frame, preview_rect, &view, &self.theme);
        } else {
            self.hit_regions.preview = None;
        }
    }

    pub(crate) fn draw_status(&self, frame: &mut Frame, area: Rect) {
        let document = self.store.active().ok();
        let caret = document
            .and_then(|doc| doc.caret().ok())
            .unwrap_or_default();
        let mut title = document
            .map(|doc| doc.tab_title())
            .unwrap_or_else(|| "oride".into());
        if let Some(vim) = &self.vim {
            title = format!("[{}] {}", vim.mode.label(), title);
        }
        let dirty = document.map(|doc| doc.is_dirty()).unwrap_or(false);
        let blame = self.blame_cache.clone();
        let git_branch = match (&self.git_branch, self.git_ahead_behind) {
            (Some(branch), Some((ahead, behind))) => {
                if let Some(indicator) = oride_git::format_ahead_behind(ahead, behind) {
                    Some(format!("{branch} {indicator}"))
                } else {
                    Some(branch.clone())
                }
            }
            (Some(branch), None) => Some(branch.clone()),
            (None, _) => None,
        };

        let status = StatusModel {
            title,
            dirty,
            line: caret.line,
            column: caret.column,
            git_branch,
            blame,
            message: self.status_message.clone(),
            help_hint: "F1 help · Ctrl+Shift+P cmds".into(),
        };
        render_status(frame, area, &status, &self.theme);
    }

    pub(crate) fn draw_scm(&self, frame: &mut Frame, area: Rect) {
        let items: Vec<ScmItem> = self
            .scm_cache
            .iter()
            .map(|(status, path)| ScmItem {
                badge: status.badge(),
                path: path.display().to_string(),
            })
            .collect();
        render_scm_panel(
            frame,
            area,
            "SCM",
            &items,
            self.scm_selected,
            self.focus == Focus::Scm,
        );
    }

    pub(crate) fn focus_label(&self) -> &'static str {
        match self.focus {
            Focus::Editor => "FOCUS: EDITOR",
            Focus::Tree => "FOCUS: TREE",
            Focus::Terminal => "FOCUS: TERMINAL",
            Focus::Scm => "FOCUS: SCM",
        }
    }

    pub(crate) fn which_key_rows(&self) -> Vec<(String, String)> {
        vec![
            ("F1".into(), "todos os atalhos".into()),
            ("Ctrl+Shift+P".into(), "command palette".into()),
            ("Ctrl+P".into(), "abrir arquivo".into()),
            ("Ctrl+S".into(), "salvar".into()),
            ("Ctrl+F".into(), "find".into()),
            ("Ctrl+Shift+F".into(), "find no projeto".into()),
            ("Ctrl+`".into(), "terminal".into()),
            ("Ctrl+Shift+G".into(), "SCM panel".into()),
            ("Ctrl+Shift+O".into(), "buffer picker".into()),
            ("Ctrl+B".into(), "foco árvore".into()),
            ("Ctrl+E".into(), "foco editor".into()),
            ("Alt+F/E/V/G/I/H".into(), "menus".into()),
            ("Alt+/".into(), "este which-key".into()),
        ]
    }

    pub(crate) fn welcome_lines(&self) -> Vec<String> {
        vec![
            "Bem-vindo ao Oride".into(),
            "".into(),
            "  F1              todos os atalhos".into(),
            "  Ctrl+Shift+P    command palette".into(),
            "  Ctrl+P          abrir arquivo".into(),
            "  Ctrl+S          salvar".into(),
            "  Ctrl+\"          terminal interativo".into(),
            "  Ctrl+Shift+G    painel Git (SCM)".into(),
            "  Alt+F           menu File".into(),
            "".into(),
            "  Enter / Esc / Space — começar".into(),
        ]
    }

    pub fn ensure_cursor_visible(&mut self) {
        let Ok(document) = self.store.active() else {
            return;
        };
        let Ok(caret) = document.caret() else {
            return;
        };
        let height = self.last_editor_height.max(1);
        let text_width = self.last_editor_text_width.max(1);
        let soft_wrap = self.soft_wrap;

        // Acima do viewport → sobe
        if caret.line < self.scroll_y {
            self.scroll_y = caret.line;
            self.split.sync_scroll(self.scroll_y);
            return;
        }

        // Linha visual do caret relativa ao topo (scroll_y)
        let caret_visual_row = visual_row_of_caret(
            document.buffer(),
            self.scroll_y,
            caret.line,
            caret.column,
            text_width,
            soft_wrap,
        );

        if caret_visual_row < height {
            return; // Ainda visível
        }

        // Coloca o caret na última linha visual do viewport
        self.scroll_y = scroll_origin_for_caret(
            document.buffer(),
            caret.line,
            caret.column,
            height,
            text_width,
            soft_wrap,
        );
        self.split.sync_scroll(self.scroll_y);
    }
}

pub(crate) fn line_visual_height(content: &str, text_width: usize) -> usize {
    let width = text_width.max(1);
    let char_count = content.chars().count();
    if char_count == 0 {
        return 1;
    }
    char_count.div_ceil(width)
}

pub(crate) fn visual_row_of_caret(
    buffer: &oride_core::Buffer,
    scroll_y: usize,
    caret_line: usize,
    caret_column: usize,
    text_width: usize,
    soft_wrap: bool,
) -> usize {
    if caret_line < scroll_y {
        return 0;
    }
    if !soft_wrap {
        return caret_line - scroll_y;
    }
    let width = text_width.max(1);
    let mut visual_rows = 0usize;
    for line_index in scroll_y..caret_line {
        let line_text = buffer.line_text(line_index).unwrap_or_default();
        let trimmed_line = line_text.trim_end_matches(['\n', '\r']);
        visual_rows = visual_rows.saturating_add(line_visual_height(trimmed_line, width));
    }
    visual_rows.saturating_add(caret_column / width)
}

pub(crate) fn scroll_origin_for_caret(
    buffer: &oride_core::Buffer,
    caret_line: usize,
    caret_column: usize,
    viewport_rows: usize,
    text_width: usize,
    soft_wrap: bool,
) -> usize {
    let height = viewport_rows.max(1);
    if !soft_wrap {
        return caret_line.saturating_add(1).saturating_sub(height);
    }
    let width = text_width.max(1);
    let mut remaining = height.saturating_sub(1);
    let on_line_above = caret_column / width;
    if on_line_above >= remaining {
        return caret_line;
    }
    remaining -= on_line_above;
    let mut start_line = caret_line;
    while start_line > 0 && remaining > 0 {
        start_line -= 1;
        let line_text = buffer.line_text(start_line).unwrap_or_default();
        let trimmed_line = line_text.trim_end_matches(['\n', '\r']);
        let line_height = line_visual_height(trimmed_line, width);
        if line_height > remaining {
            start_line += 1;
            break;
        }
        remaining -= line_height;
    }
    start_line
}
