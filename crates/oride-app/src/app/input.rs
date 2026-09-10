//! Tratamento de teclado, atalhos, navegação em sobreposições e painéis.

use std::path::{Path, PathBuf};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use oride_core::DocumentId;
use oride_fs::CreateKind;
use oride_keymap::{Action, KeyChord};
use oride_syntax::detect_language;

use crate::modal::{InsertPosition, VimAction, VimMode};

use crate::browser::{BrowseAction, BrowseMode};

use super::lsp::lsp_position_to_offset;
use super::state::{fuzzy_match, App, Focus, KeyCommand, Overlay, PromptKind};

impl App {
    /// Entrada principal de teclado (overlays / focus / keymap).
    pub fn handle_key(&mut self, key: KeyEvent) {
        // Surround: próximo char é o delimitador
        if self.surround_pending {
            self.surround_pending = false;
            self.overlay = Overlay::None;
            if let KeyCode::Char(character) = key.code {
                self.apply_surround(character);
            } else if key.code != KeyCode::Esc {
                self.set_status("surround cancelado");
            }
            return;
        }
        // Welcome / which-key / menu têm prioridade sobre o resto
        if self.show_welcome {
            self.handle_welcome_key(key);
            return;
        }
        if self.show_which_key {
            if matches!(key.code, KeyCode::Esc | KeyCode::Enter | KeyCode::Char('q')) {
                self.show_which_key = false;
            }
            return;
        }
        if self.menu_open.is_some() {
            self.handle_menu_key(key);
            return;
        }
        if self.handle_overlay_key(key) {
            return;
        }
        // Alt+letra abre menu (File/Edit/…) quando nenhum overlay estiver ativo
        if key.modifiers.contains(KeyModifiers::ALT)
            && !key.modifiers.contains(KeyModifiers::CONTROL)
            && !key.modifiers.contains(KeyModifiers::SHIFT)
        {
            if let KeyCode::Char(character) = key.code {
                let lower_char = character.to_ascii_lowercase();
                if let Some(menu_index) =
                    self.menus.iter().position(|menu| menu.hotkey == lower_char)
                {
                    self.menu_open = Some((menu_index, 0));
                    return;
                }
            }
        }
        // Preview MD: Alt+↑/↓ ajusta offset fino em cima do scroll do editor; Alt+Enter abre link
        if self.show_md_preview
            && key.modifiers.contains(KeyModifiers::ALT)
            && matches!(
                key.code,
                KeyCode::PageUp | KeyCode::PageDown | KeyCode::Up | KeyCode::Down | KeyCode::Enter
            )
        {
            match key.code {
                KeyCode::PageUp | KeyCode::Up => {
                    self.preview_scroll = self.preview_scroll.saturating_sub(3);
                }
                KeyCode::PageDown | KeyCode::Down => {
                    self.preview_scroll = self.preview_scroll.saturating_add(3);
                }
                KeyCode::Enter => {
                    self.open_active_preview_link();
                }
                _ => {}
            }
            return;
        }
        match self.focus {
            Focus::Tree => {
                if self.handle_tree_key(key) {
                    return;
                }
            }
            Focus::Terminal => {
                if self.handle_terminal_key(key) {
                    return;
                }
            }
            Focus::Scm => {
                if self.handle_scm_key(key) {
                    return;
                }
            }
            Focus::Editor => {
                if self.vim.is_some() {
                    if self.handle_vim_key(key) {
                        return;
                    }
                } else if key.code == KeyCode::Esc && key.modifiers.is_empty() {
                    if let Ok(document) = self.store.active_mut() {
                        let has_extra = !document.extra_carets().is_empty();
                        let has_selection = !document.selection().is_empty();
                        if has_extra || has_selection {
                            document.clear_extra_carets();
                            document.collapse_selection();
                            self.set_status("seleção / multi-cursor cancelados");
                            return;
                        }
                    }
                }
            }
        }
        if let Some(task) = self.check_task_shortcut(&key) {
            self.run_task(&task);
            return;
        }
        if let Some(command) = self.map_key(key) {
            self.apply(command);
        }
    }

    pub(crate) fn handle_overlay_key(&mut self, key: KeyEvent) -> bool {
        match &self.overlay {
            Overlay::None => false,
            Overlay::CommandPalette { .. } => {
                self.handle_palette_key(key);
                true
            }
            Overlay::Browse(_) => {
                self.handle_browse_key(key);
                true
            }
            Overlay::Prompt { .. } => {
                self.handle_prompt_key(key);
                true
            }
            Overlay::Help { .. } => {
                self.handle_help_key(key);
                true
            }
            Overlay::Find => {
                self.handle_find_key(key);
                true
            }
            Overlay::ProjectFind { .. } => {
                self.handle_project_find_key(key);
                true
            }
            Overlay::Diagnostics { .. } => {
                self.handle_diagnostics_key(key);
                true
            }
            Overlay::Completion { .. } => {
                self.handle_completion_key(key);
                true
            }
            Overlay::Hover { .. } => {
                if matches!(key.code, KeyCode::Esc | KeyCode::Enter | KeyCode::Char('q')) {
                    self.overlay = Overlay::None;
                }
                true
            }
            Overlay::BufferPicker { .. } => {
                self.handle_buffer_picker_key(key);
                true
            }
            Overlay::Diff { .. } => {
                self.handle_diff_key(key);
                true
            }
            Overlay::SurroundPick => true,
            Overlay::MultiPicker { .. } => {
                self.handle_multi_picker_key(key);
                true
            }
            Overlay::UndoTree { .. } => {
                self.handle_undo_tree_key(key);
                true
            }
            Overlay::ReloadConfirm { path } => {
                match key.code {
                    KeyCode::Enter => {
                        let file_path = path.clone();
                        self.overlay = Overlay::None;
                        if let Some(document_id) = self.store.document_id_for_path(&file_path) {
                            match self.reload_document(document_id) {
                                Ok(()) => {
                                    self.set_status(format!(
                                        "recarregado: {}",
                                        file_path.display()
                                    ));
                                }
                                Err(error) => self.set_status(format!("reload: {error}")),
                            }
                        }
                        self.pending_reload = None;
                    }
                    KeyCode::Esc => {
                        self.overlay = Overlay::None;
                        self.pending_reload = None;
                        self.set_status("reload ignorado");
                    }
                    _ => {}
                }
                true
            }
            Overlay::ThemePicker { .. } => {
                self.handle_theme_picker_key(key);
                true
            }
            Overlay::LocalePicker { .. } => {
                self.handle_locale_picker_key(key);
                true
            }
            Overlay::VimCommand { .. } => {
                self.handle_vim_command_key(key);
                true
            }
            Overlay::HealthCheck { .. } => {
                self.handle_health_check_key(key);
                true
            }
        }
    }

    pub(crate) fn handle_browse_key(&mut self, key: KeyEvent) {
        let Overlay::Browse(browser) = &mut self.overlay else {
            return;
        };
        let is_ctrl = key.modifiers.contains(KeyModifiers::CONTROL);
        let is_alt = key.modifiers.contains(KeyModifiers::ALT);
        let current_mode = browser.mode;

        let should_confirm = matches!(key.code, KeyCode::F(2))
            || (matches!(key.code, KeyCode::Enter) && (is_ctrl || is_alt))
            || (is_ctrl && matches!(key.code, KeyCode::Char('o')))
            || (is_ctrl
                && matches!(key.code, KeyCode::Char('s'))
                && current_mode == BrowseMode::SaveAs)
            || (is_ctrl
                && matches!(
                    key.code,
                    KeyCode::Char('\n')
                        | KeyCode::Char('\r')
                        | KeyCode::Char('j')
                        | KeyCode::Char('m')
                ));

        let action = match key.code {
            KeyCode::Esc => {
                self.overlay = Overlay::None;
                None
            }
            _ if should_confirm => Some(browser.confirm_folder()),
            KeyCode::Up | KeyCode::Char('k') if !is_ctrl => {
                browser.move_selection(-1);
                None
            }
            KeyCode::Down | KeyCode::Char('j') if !is_ctrl => {
                browser.move_selection(1);
                None
            }
            KeyCode::PageUp => {
                browser.move_selection(-10);
                None
            }
            KeyCode::PageDown => {
                browser.move_selection(10);
                None
            }
            KeyCode::Backspace if browser.filter.is_empty() => {
                browser.go_parent();
                None
            }
            KeyCode::Backspace => {
                browser.filter.pop();
                browser.selected = 0;
                None
            }
            KeyCode::Enter if current_mode == BrowseMode::SaveAs => {
                if browser.filter.trim().is_empty() {
                    self.set_status("save as: digite o nome do arquivo");
                    None
                } else {
                    Some(browser.confirm_folder())
                }
            }
            KeyCode::Right | KeyCode::Char('l') if !is_ctrl => Some(browser.activate()),
            KeyCode::Enter => Some(browser.activate()),
            KeyCode::Char(character) if !is_ctrl && !is_alt && !character.is_control() => {
                browser.filter.push(character);
                browser.selected = 0;
                None
            }
            _ => None,
        };

        if let Some(browse_action) = action {
            self.apply_browse_action(browse_action);
        }
    }

    pub(crate) fn apply_browse_action(&mut self, action: BrowseAction) {
        match action {
            BrowseAction::Stay => {}
            BrowseAction::OpenFile(path) => {
                self.overlay = Overlay::None;
                if let Err(error) = self.open_document_path(&path) {
                    self.set_status(format!("open: {error}"));
                } else {
                    self.focus = Focus::Editor;
                    self.scroll_y = 0;
                    self.set_status(format!("aberto · {}", path.display()));
                }
            }
            BrowseAction::OpenFolder(path) => {
                self.overlay = Overlay::None;
                self.open_workspace_folder(path);
            }
            BrowseAction::SaveAsPath(path) => {
                self.overlay = Overlay::None;
                let result = self
                    .store
                    .active_mut()
                    .and_then(|document| document.save_to(Some(&path)));
                match result {
                    Ok(()) => {
                        self.notify_saved_path(&path);
                        self.set_status(format!("salvo: {}", path.display()));
                        self.refresh_git_and_index();
                    }
                    Err(error) => self.set_status(format!("save as: {error}")),
                }
            }
        }
    }

    pub(crate) fn handle_find_key(&mut self, key: KeyEvent) {
        let is_ctrl = key.modifiers.contains(KeyModifiers::CONTROL);
        let is_alt = key.modifiers.contains(KeyModifiers::ALT);
        let is_shift = key.modifiers.contains(KeyModifiers::SHIFT);

        match key.code {
            KeyCode::Esc => {
                self.overlay = Overlay::None;
            }
            KeyCode::Tab => {
                if !self.find.show_replace {
                    self.find.show_replace = true;
                }
                self.find.focus_replace = !self.find.focus_replace;
            }
            KeyCode::F(3) if is_shift => {
                self.jump_find(false);
            }
            KeyCode::F(3) | KeyCode::Enter if !is_alt && !is_ctrl => {
                self.jump_find(true);
            }
            KeyCode::Enter if is_alt && is_ctrl => {
                self.replace_all_matches();
            }
            KeyCode::Enter if is_alt => {
                self.replace_current_match();
            }
            KeyCode::Char('c') if is_alt && !is_ctrl => {
                self.find.toggle_case();
                self.recompute_find_and_jump();
            }
            KeyCode::Char('a') if is_alt && !is_ctrl => {
                self.find.toggle_accents();
                self.recompute_find_and_jump();
            }
            KeyCode::Char('w') if is_alt && !is_ctrl => {
                self.find.toggle_whole_word();
                self.recompute_find_and_jump();
            }
            KeyCode::Char('r') if is_alt && !is_ctrl => {
                self.find.toggle_regex();
                self.recompute_find_and_jump();
            }
            KeyCode::Char('h') if is_ctrl => {
                self.find.show_replace = !self.find.show_replace;
                if self.find.show_replace {
                    self.find.focus_replace = true;
                }
            }
            KeyCode::Backspace => {
                if self.find.focus_replace && self.find.show_replace {
                    self.find.replace.pop();
                } else {
                    self.find.query.pop();
                    self.recompute_find_and_jump();
                }
            }
            KeyCode::Char(character) if !is_ctrl && !is_alt && !character.is_control() => {
                if self.find.focus_replace && self.find.show_replace {
                    self.find.replace.push(character);
                } else {
                    self.find.query.push(character);
                    self.recompute_find_and_jump();
                }
            }
            _ => {}
        }
        self.set_status(self.find.status());
    }

    pub(crate) fn handle_palette_key(&mut self, key: KeyEvent) {
        let Overlay::CommandPalette { query, selected } = &self.overlay else {
            return;
        };
        let query = query.clone();
        let selected = *selected;

        match key.code {
            KeyCode::Esc => {
                self.overlay = Overlay::None;
            }
            KeyCode::Enter => {
                let items = self.command_palette_items(&query);
                if let Some(item) = items.get(selected).cloned() {
                    self.overlay = Overlay::None;
                    if let Some(action) = Action::palette_actions()
                        .iter()
                        .find(|action| action.palette_label() == item)
                        .copied()
                    {
                        let _ = self.apply_action(action);
                    } else if let Some(cmd_id) = self.plugin_host.command_id_for_label(&item) {
                        self.run_plugin_command(cmd_id);
                    } else {
                        self.run_plugin_command(&item);
                    }
                }
            }
            KeyCode::Up => {
                let len = self.command_palette_items(&query).len();
                let new_selected = if len == 0 {
                    0
                } else {
                    selected.saturating_sub(1)
                };
                self.overlay = Overlay::CommandPalette {
                    query,
                    selected: new_selected,
                };
            }
            KeyCode::Down => {
                let len = self.command_palette_items(&query).len();
                let new_selected = if len == 0 {
                    0
                } else {
                    (selected + 1).min(len - 1)
                };
                self.overlay = Overlay::CommandPalette {
                    query,
                    selected: new_selected,
                };
            }
            KeyCode::Backspace => {
                let mut updated_query = query;
                updated_query.pop();
                self.overlay = Overlay::CommandPalette {
                    query: updated_query,
                    selected: 0,
                };
            }
            KeyCode::Char(character) if !key.modifiers.contains(KeyModifiers::CONTROL) => {
                let mut updated_query = query;
                updated_query.push(character);
                self.overlay = Overlay::CommandPalette {
                    query: updated_query,
                    selected: 0,
                };
            }
            _ => {}
        }
    }

    pub(crate) fn handle_theme_picker_key(&mut self, key: KeyEvent) {
        let Overlay::ThemePicker {
            query,
            selected,
            initial_theme,
            themes,
        } = &self.overlay
        else {
            return;
        };
        let mut query = query.clone();
        let mut selected = *selected;
        let initial_theme = initial_theme.clone();
        let themes = themes.clone();

        let filtered: Vec<String> = themes
            .iter()
            .filter(|t| fuzzy_match(&query, t))
            .cloned()
            .collect();
        let count = filtered.len();

        match key.code {
            KeyCode::Esc => {
                self.apply_theme_by_name(&initial_theme);
                self.config.theme = initial_theme;
                self.overlay = Overlay::None;
                self.set_status("troca de tema cancelada");
                return;
            }
            KeyCode::Enter => {
                if let Some(chosen) = filtered.get(selected) {
                    self.apply_theme_by_name(chosen);
                    if let Err(err) = oride_config::save_user_theme(chosen) {
                        self.set_status(format!("tema aplicado ({chosen}), erro ao salvar: {err}"));
                    } else {
                        self.set_status(format!("tema aplicado e salvo: {chosen}"));
                    }
                }
                self.overlay = Overlay::None;
                return;
            }
            KeyCode::Up => {
                if count > 0 {
                    selected = selected.saturating_sub(1);
                    if let Some(target) = filtered.get(selected) {
                        self.apply_theme_by_name(target);
                    }
                }
            }
            KeyCode::Down => {
                if count > 0 {
                    selected = (selected + 1).min(count - 1);
                    if let Some(target) = filtered.get(selected) {
                        self.apply_theme_by_name(target);
                    }
                }
            }
            KeyCode::Home => {
                selected = 0;
                if let Some(target) = filtered.first() {
                    self.apply_theme_by_name(target);
                }
            }
            KeyCode::End => {
                if count > 0 {
                    selected = count - 1;
                    if let Some(target) = filtered.get(selected) {
                        self.apply_theme_by_name(target);
                    }
                }
            }
            KeyCode::Backspace => {
                query.pop();
                selected = 0;
                let new_filtered: Vec<String> = themes
                    .iter()
                    .filter(|t| fuzzy_match(&query, t))
                    .cloned()
                    .collect();
                if let Some(target) = new_filtered.first() {
                    self.apply_theme_by_name(target);
                }
            }
            KeyCode::Char(character)
                if !key.modifiers.contains(KeyModifiers::CONTROL)
                    && !key.modifiers.contains(KeyModifiers::ALT)
                    && !character.is_control() =>
            {
                query.push(character);
                selected = 0;
                let new_filtered: Vec<String> = themes
                    .iter()
                    .filter(|t| fuzzy_match(&query, t))
                    .cloned()
                    .collect();
                if let Some(target) = new_filtered.first() {
                    self.apply_theme_by_name(target);
                }
            }
            _ => {}
        }

        self.overlay = Overlay::ThemePicker {
            query,
            selected,
            initial_theme,
            themes,
        };
    }

    pub(crate) fn handle_locale_picker_key(&mut self, key: KeyEvent) {
        let Overlay::LocalePicker {
            query,
            selected,
            locales,
        } = &self.overlay
        else {
            return;
        };
        let mut query = query.clone();
        let mut selected = *selected;
        let locales = locales.clone();

        let filtered: Vec<String> = locales
            .iter()
            .filter(|l| fuzzy_match(&query, l))
            .cloned()
            .collect();
        let count = filtered.len();

        match key.code {
            KeyCode::Esc => {
                self.overlay = Overlay::None;
                return;
            }
            KeyCode::Enter => {
                if let Some(chosen_label) = filtered.get(selected) {
                    if let Some(matched_locale) = oride_i18n::available_locales().iter().find(|l| {
                        chosen_label.contains(l.as_str())
                            || chosen_label.contains(&l.display_name())
                    }) {
                        self.update_locale(matched_locale.clone());
                    }
                }
                self.overlay = Overlay::None;
                return;
            }
            KeyCode::Up => {
                if count > 0 {
                    selected = selected.saturating_sub(1);
                }
            }
            KeyCode::Down => {
                if count > 0 {
                    selected = (selected + 1).min(count - 1);
                }
            }
            KeyCode::Home => {
                selected = 0;
            }
            KeyCode::End => {
                if count > 0 {
                    selected = count - 1;
                }
            }
            KeyCode::Backspace => {
                query.pop();
                selected = 0;
            }
            KeyCode::Char(character)
                if !key.modifiers.contains(KeyModifiers::CONTROL)
                    && !key.modifiers.contains(KeyModifiers::ALT)
                    && !character.is_control() =>
            {
                query.push(character);
                selected = 0;
            }
            _ => {}
        }

        self.overlay = Overlay::LocalePicker {
            query,
            selected,
            locales,
        };
    }

    pub(crate) fn handle_vim_command_key(&mut self, key: KeyEvent) {
        let Overlay::VimCommand { buffer } = &self.overlay else {
            return;
        };
        let mut buffer = buffer.clone();

        match key.code {
            KeyCode::Esc => {
                self.overlay = Overlay::None;
                return;
            }
            KeyCode::Enter => {
                let cmd = buffer.trim().to_string();
                self.overlay = Overlay::None;
                self.execute_vim_command(&cmd);
                return;
            }
            KeyCode::Backspace => {
                if buffer.is_empty() {
                    self.overlay = Overlay::None;
                    return;
                }
                buffer.pop();
            }
            KeyCode::Char(character)
                if !key.modifiers.contains(KeyModifiers::CONTROL)
                    && !key.modifiers.contains(KeyModifiers::ALT)
                    && !character.is_control() =>
            {
                buffer.push(character);
            }
            _ => {}
        }

        self.overlay = Overlay::VimCommand { buffer };
    }

    pub(crate) fn handle_health_check_key(&mut self, key: KeyEvent) {
        let Overlay::HealthCheck { scroll, report } = &self.overlay else {
            return;
        };
        let mut scroll = *scroll;
        let report = report.clone();
        let total_items = report.lsp_items.len() + 6;

        match key.code {
            KeyCode::Esc | KeyCode::Enter | KeyCode::Char('q') => {
                self.overlay = Overlay::None;
                return;
            }
            KeyCode::Up | KeyCode::Char('k') => {
                scroll = scroll.saturating_sub(1);
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if total_items > 0 {
                    scroll = (scroll + 1).min(total_items.saturating_sub(1));
                }
            }
            KeyCode::PageUp => {
                scroll = scroll.saturating_sub(5);
            }
            KeyCode::PageDown => {
                if total_items > 0 {
                    scroll = (scroll + 5).min(total_items.saturating_sub(1));
                }
            }
            KeyCode::Home => {
                scroll = 0;
            }
            KeyCode::End if total_items > 0 => {
                scroll = total_items.saturating_sub(1);
            }
            _ => {}
        }

        self.overlay = Overlay::HealthCheck { scroll, report };
    }

    pub(crate) fn execute_vim_command(&mut self, cmd: &str) {
        if cmd.is_empty() {
            return;
        }

        // Pulo de linha direto: :<numero>
        if let Ok(line_num) = cmd.parse::<usize>() {
            if let Ok(doc) = self.store.active_mut() {
                let target_line = line_num.saturating_sub(1);
                if let Ok(byte) = doc.buffer().line_to_byte(target_line) {
                    doc.jump_to_byte(byte);
                    self.scroll_y = target_line.saturating_sub(5);
                    self.ensure_cursor_visible();
                    self.set_status(format!("linha {line_num}"));
                }
            }
            return;
        }

        let parts: Vec<&str> = cmd.split_whitespace().collect();
        let main_cmd = parts[0];

        match main_cmd {
            "w" | "write" => {
                self.apply(KeyCommand::Action(Action::Save));
            }
            "q" | "quit" => {
                self.apply(KeyCommand::Action(Action::CloseTab));
            }
            "q!" => {
                if let Some(id) = self.store.active_id() {
                    self.close_tab_confirm = Some(id);
                }
                self.apply(KeyCommand::Action(Action::CloseTab));
            }
            "wq" | "x" => {
                self.apply(KeyCommand::Action(Action::Save));
                self.apply(KeyCommand::Action(Action::CloseTab));
            }
            "qa" | "qall" | "quitall" => {
                self.apply(KeyCommand::Action(Action::Quit));
            }
            "health" | "checkhealth" => {
                self.apply(KeyCommand::Action(Action::HealthCheck));
            }
            "tasks" => {
                self.apply(KeyCommand::Action(Action::RunTasks));
            }
            "run" => {
                if parts.len() > 1 {
                    let task_name = parts[1..].join(" ");
                    self.execute_task_runner(Some(&task_name));
                } else {
                    self.apply(KeyCommand::Action(Action::RunTasks));
                }
            }
            "e" | "edit" => {
                if parts.len() > 1 {
                    let file_path = PathBuf::from(parts[1]);
                    let full_path = if file_path.is_absolute() {
                        file_path
                    } else {
                        self.workspace.join(file_path)
                    };
                    if let Err(e) = self.open_document_path(&full_path) {
                        self.set_status(format!("erro ao abrir arquivo: {e}"));
                    }
                } else {
                    self.set_status("uso: :e <caminho>");
                }
            }
            "theme" => {
                if parts.len() > 1 {
                    self.apply_theme_by_name(parts[1]);
                } else {
                    self.apply(KeyCommand::Action(Action::SelectTheme));
                }
            }
            "lang" => {
                if parts.len() > 1 {
                    let loc = oride_i18n::Locale::from_str_loose(parts[1]);
                    self.update_locale(loc);
                } else {
                    self.apply(KeyCommand::Action(Action::SelectLocale));
                }
            }
            "noh" | "nohlsearch" => {
                if let Ok(doc) = self.store.active_mut() {
                    doc.collapse_selection();
                }
                self.set_status("");
            }
            _ => {
                self.set_status(format!("Comando desconhecido: {cmd}"));
            }
        }
    }

    pub(crate) fn handle_vim_key(&mut self, key: KeyEvent) -> bool {
        let Some(mut vim) = self.vim.take() else {
            return false;
        };

        let is_insert = vim.mode == VimMode::Insert;
        let is_visual = matches!(vim.mode, VimMode::Visual | VimMode::VisualLine);

        // Se estiver em Insert:
        if is_insert {
            if key.code == KeyCode::Esc
                || (key.code == KeyCode::Char('[') && key.modifiers.contains(KeyModifiers::CONTROL))
            {
                vim.mode = VimMode::Normal;
                vim.pending_operator = None;
                self.vim = Some(vim);
                self.set_status("-- NORMAL --");
                return true;
            }
            self.vim = Some(vim);
            return false;
        }

        // Se estiver em Visual / VisualLine e apertar Esc:
        if is_visual && key.code == KeyCode::Esc {
            vim.mode = VimMode::Normal;
            vim.visual_anchor = None;
            if let Ok(doc) = self.store.active_mut() {
                doc.collapse_selection();
            }
            self.vim = Some(vim);
            self.set_status("-- NORMAL --");
            return true;
        }

        let action = vim.handle_key(&key);
        let pending = vim.pending_operator;
        let mode = vim.mode;
        self.vim = Some(vim);

        match action {
            VimAction::EnterInsert(pos) => {
                match pos {
                    InsertPosition::AtCursor => {}
                    InsertPosition::AfterCursor => {
                        self.apply(KeyCommand::Action(Action::MoveRight { extend: false }));
                    }
                    InsertPosition::LineStart => {
                        self.apply(KeyCommand::Action(Action::MoveLineStart { extend: false }));
                    }
                    InsertPosition::LineEnd => {
                        self.apply(KeyCommand::Action(Action::MoveLineEnd { extend: false }));
                    }
                    InsertPosition::NewLineBelow => {
                        self.apply(KeyCommand::Action(Action::MoveLineEnd { extend: false }));
                        self.apply(KeyCommand::Action(Action::InsertNewline));
                    }
                    InsertPosition::NewLineAbove => {
                        self.apply(KeyCommand::Action(Action::MoveLineStart { extend: false }));
                        self.apply(KeyCommand::Action(Action::InsertNewline));
                        self.apply(KeyCommand::Action(Action::MoveUp { extend: false }));
                    }
                }
                self.set_status("-- INSERT --");
                true
            }
            VimAction::EnterVisual(is_line) => {
                if is_line {
                    self.apply(KeyCommand::Action(Action::MoveLineStart { extend: false }));
                    self.apply(KeyCommand::Action(Action::MoveLineEnd { extend: true }));
                    self.set_status("-- VISUAL LINE --");
                } else {
                    self.set_status("-- VISUAL --");
                }
                true
            }
            VimAction::ExitToNormal => {
                if let Ok(doc) = self.store.active_mut() {
                    doc.collapse_selection();
                }
                self.set_status("-- NORMAL --");
                true
            }
            VimAction::OpenCommandLine => {
                self.overlay = Overlay::VimCommand {
                    buffer: String::new(),
                };
                true
            }
            VimAction::MoveLeft => {
                let extend = matches!(mode, VimMode::Visual | VimMode::VisualLine);
                self.apply(KeyCommand::Action(Action::MoveLeft { extend }));
                true
            }
            VimAction::MoveRight => {
                let extend = matches!(mode, VimMode::Visual | VimMode::VisualLine);
                self.apply(KeyCommand::Action(Action::MoveRight { extend }));
                true
            }
            VimAction::MoveUp => {
                let extend = matches!(mode, VimMode::Visual | VimMode::VisualLine);
                self.apply(KeyCommand::Action(Action::MoveUp { extend }));
                true
            }
            VimAction::MoveDown => {
                let extend = matches!(mode, VimMode::Visual | VimMode::VisualLine);
                self.apply(KeyCommand::Action(Action::MoveDown { extend }));
                true
            }
            VimAction::WordForward | VimAction::WordEnd => {
                let extend = matches!(mode, VimMode::Visual | VimMode::VisualLine);
                for _ in 0..4 {
                    self.apply(KeyCommand::Action(Action::MoveRight { extend }));
                }
                true
            }
            VimAction::WordBackward => {
                let extend = matches!(mode, VimMode::Visual | VimMode::VisualLine);
                for _ in 0..4 {
                    self.apply(KeyCommand::Action(Action::MoveLeft { extend }));
                }
                true
            }
            VimAction::LineStart => {
                let extend = matches!(mode, VimMode::Visual | VimMode::VisualLine);
                self.apply(KeyCommand::Action(Action::MoveLineStart { extend }));
                true
            }
            VimAction::LineEnd => {
                let extend = matches!(mode, VimMode::Visual | VimMode::VisualLine);
                self.apply(KeyCommand::Action(Action::MoveLineEnd { extend }));
                true
            }
            VimAction::FirstLine => {
                let extend = matches!(mode, VimMode::Visual | VimMode::VisualLine);
                self.apply(KeyCommand::Action(Action::MoveDocStart { extend }));
                true
            }
            VimAction::LastLine => {
                let extend = matches!(mode, VimMode::Visual | VimMode::VisualLine);
                self.apply(KeyCommand::Action(Action::MoveDocEnd { extend }));
                true
            }
            VimAction::DeleteChar => {
                self.apply(KeyCommand::Action(Action::Delete));
                true
            }
            VimAction::DeleteLine => {
                self.apply(KeyCommand::Action(Action::MoveLineStart { extend: false }));
                self.apply(KeyCommand::Action(Action::MoveDown { extend: true }));
                self.apply(KeyCommand::Action(Action::Backspace));
                true
            }
            VimAction::DeleteSelection => {
                self.apply(KeyCommand::Action(Action::Backspace));
                true
            }
            VimAction::YankLine => {
                self.apply(KeyCommand::Action(Action::MoveLineStart { extend: false }));
                self.apply(KeyCommand::Action(Action::MoveDown { extend: true }));
                self.apply(KeyCommand::Action(Action::Copy));
                if let Ok(doc) = self.store.active_mut() {
                    doc.collapse_selection();
                }
                self.set_status("linha copiada");
                true
            }
            VimAction::YankSelection => {
                self.apply(KeyCommand::Action(Action::Copy));
                if let Ok(doc) = self.store.active_mut() {
                    doc.collapse_selection();
                }
                self.set_status("seleção copiada");
                true
            }
            VimAction::PasteAfter | VimAction::PasteBefore => {
                self.apply(KeyCommand::Action(Action::Paste));
                true
            }
            VimAction::Undo => {
                self.apply(KeyCommand::Action(Action::Undo));
                true
            }
            VimAction::Redo => {
                self.apply(KeyCommand::Action(Action::Redo));
                true
            }
            VimAction::None => {
                if pending.is_some() {
                    return true;
                }
                if key.modifiers.contains(KeyModifiers::CONTROL)
                    || key.modifiers.contains(KeyModifiers::ALT)
                {
                    return false;
                }
                true
            }
        }
    }

    pub(crate) fn check_task_shortcut(&self, key: &KeyEvent) -> Option<crate::tasks::TaskDef> {
        let chord = KeyChord::from_event(*key);
        let tasks = crate::tasks::discover_tasks(Some(&self.workspace));
        for task in tasks {
            if let Some(key_spec) = &task.key {
                if let Ok(task_chord) = oride_keymap::parse_chord(key_spec) {
                    if chord == task_chord {
                        return Some(task);
                    }
                }
            }
        }
        None
    }

    pub(crate) fn command_palette_items(&self, query: &str) -> Vec<String> {
        let mut items: Vec<String> = Action::palette_actions()
            .iter()
            .map(|action| action.palette_label().to_string())
            .collect();
        for command in self.plugin_host.palette_commands() {
            items.push(command.label.to_string());
        }
        items.retain(|label| fuzzy_match(query, label));
        items.sort();
        items
    }

    pub(crate) fn keybind_list_items(&self, query: &str) -> Vec<String> {
        self.keymap
            .list_bindings()
            .into_iter()
            .map(|(chord, action)| format!("{:<24}  {}", chord, action.palette_label()))
            .filter(|line| fuzzy_match(query, line))
            .collect()
    }

    pub(crate) fn handle_help_key(&mut self, key: KeyEvent) {
        let Overlay::Help { query, selected } = &self.overlay else {
            return;
        };
        let mut query = query.clone();
        let mut selected = *selected;
        let items_len = self.keybind_list_items(&query).len();

        match key.code {
            KeyCode::Esc | KeyCode::Enter | KeyCode::Char('q')
                if !key.modifiers.contains(KeyModifiers::CONTROL) =>
            {
                if matches!(key.code, KeyCode::Char('q')) && !query.is_empty() {
                    query.push('q');
                    selected = 0;
                    self.overlay = Overlay::Help { query, selected };
                    return;
                }
                self.overlay = Overlay::None;
                return;
            }
            KeyCode::Up => {
                selected = selected.saturating_sub(1);
            }
            KeyCode::Down => {
                if items_len > 0 {
                    selected = (selected + 1).min(items_len - 1);
                }
            }
            KeyCode::PageUp => {
                selected = selected.saturating_sub(10);
            }
            KeyCode::PageDown => {
                if items_len > 0 {
                    selected = (selected + 10).min(items_len - 1);
                }
            }
            KeyCode::Home => selected = 0,
            KeyCode::End if items_len > 0 => selected = items_len - 1,
            KeyCode::Backspace => {
                query.pop();
                selected = 0;
            }
            KeyCode::Char(character)
                if !key.modifiers.contains(KeyModifiers::CONTROL)
                    && !key.modifiers.contains(KeyModifiers::ALT)
                    && !character.is_control() =>
            {
                query.push(character);
                selected = 0;
            }
            _ => {}
        }

        let total_items = self.keybind_list_items(&query).len();
        if total_items == 0 {
            selected = 0;
        } else {
            selected = selected.min(total_items - 1);
        }
        self.overlay = Overlay::Help { query, selected };
    }

    pub(crate) fn handle_prompt_key(&mut self, key: KeyEvent) {
        let Overlay::Prompt { kind, buffer } = &self.overlay else {
            return;
        };
        let kind = *kind;
        let mut buffer = buffer.clone();
        match key.code {
            KeyCode::Esc => self.overlay = Overlay::None,
            KeyCode::Enter => {
                self.overlay = Overlay::None;
                match kind {
                    PromptKind::NewFile | PromptKind::NewDir => {
                        if let Some(tree) = self.tree.as_mut() {
                            let create_kind = match kind {
                                PromptKind::NewFile => CreateKind::File,
                                PromptKind::NewDir => CreateKind::Directory,
                                _ => unreachable!(),
                            };
                            match tree.create_under_selection(create_kind, &buffer) {
                                Ok(path) => {
                                    self.refresh_git_and_index();
                                    if kind == PromptKind::NewFile {
                                        if let Err(error) = self.open_document_path(&path) {
                                            self.set_status(format!("open: {error}"));
                                        } else {
                                            self.focus = Focus::Editor;
                                        }
                                    } else {
                                        self.set_status("folder created");
                                    }
                                }
                                Err(error) => self.set_status(format!("create: {error}")),
                            }
                        }
                    }
                    PromptKind::Rename => {
                        if let Some(tree) = self.tree.as_mut() {
                            match tree.rename_selected(&buffer) {
                                Ok(path) => {
                                    self.refresh_git_and_index();
                                    self.set_status(format!("renamed to {}", path.display()));
                                }
                                Err(error) => self.set_status(format!("rename: {error}")),
                            }
                        }
                    }
                    PromptKind::DeleteConfirm => {
                        if let Some(tree) = self.tree.as_mut() {
                            match tree.delete_selected() {
                                Ok(deleted) => {
                                    self.refresh_git_and_index();
                                    self.set_status(format!("deleted {}", deleted.display()));
                                }
                                Err(error) => self.set_status(format!("delete: {error}")),
                            }
                        }
                    }
                    PromptKind::Commit => match oride_git::commit(&self.workspace, &buffer) {
                        Ok(output) => {
                            self.refresh_scm_cache();
                            self.refresh_git_and_index();
                            self.set_status(format!("commit: {output}"));
                        }
                        Err(error) => self.set_status(format!("commit: {error}")),
                    },
                }
            }
            KeyCode::Backspace => {
                buffer.pop();
                self.overlay = Overlay::Prompt { kind, buffer };
            }
            KeyCode::Char(character) if !key.modifiers.contains(KeyModifiers::CONTROL) => {
                buffer.push(character);
                self.overlay = Overlay::Prompt { kind, buffer };
            }
            _ => {}
        }
    }

    pub(crate) fn handle_tree_key(&mut self, key: KeyEvent) -> bool {
        if key.modifiers.contains(KeyModifiers::CONTROL)
            || key.modifiers.contains(KeyModifiers::ALT)
        {
            return false;
        }
        match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                if let Some(tree) = self.tree.as_mut() {
                    tree.move_selection(-1);
                    self.ensure_tree_visible();
                }
                true
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if let Some(tree) = self.tree.as_mut() {
                    tree.move_selection(1);
                    self.ensure_tree_visible();
                }
                true
            }
            KeyCode::Enter => {
                if let Some(tree) = self.tree.as_mut() {
                    match tree.activate_selected() {
                        Ok(Some(path)) => {
                            if let Err(error) = self.open_document_path(&path) {
                                self.set_status(format!("open: {error}"));
                            } else {
                                let language = detect_language(Some(path.as_path()));
                                self.focus = Focus::Editor;
                                self.scroll_y = 0;
                                self.set_status(format!("aberto · {}", language.as_str()));
                            }
                        }
                        Ok(None) => {
                            self.ensure_tree_visible();
                            self.set_status("↑↓ navegar · Enter abrir/expandir · ←→ colapsar/expandir · Esc editor");
                        }
                        Err(error) => self.set_status(format!("tree: {error}")),
                    }
                }
                true
            }
            KeyCode::Right | KeyCode::Char('l') => {
                if let Some(tree) = self.tree.as_mut() {
                    if let Err(error) = tree.expand_selected() {
                        self.set_status(format!("tree: {error}"));
                    }
                    self.ensure_tree_visible();
                }
                true
            }
            KeyCode::Left | KeyCode::Char('h') => {
                if let Some(tree) = self.tree.as_mut() {
                    if let Err(error) = tree.collapse_or_parent() {
                        self.set_status(format!("tree: {error}"));
                    }
                    self.ensure_tree_visible();
                }
                true
            }
            KeyCode::Char(' ') => {
                if let Some(tree) = self.tree.as_mut() {
                    if let Err(error) = tree.toggle_selected() {
                        self.set_status(format!("tree: {error}"));
                    }
                    self.ensure_tree_visible();
                }
                true
            }
            KeyCode::Char('r') => {
                if let Some(tree) = self.tree.as_ref() {
                    if let Some(row) = tree.selected_row() {
                        if row.path != *tree.root() {
                            self.overlay = Overlay::Prompt {
                                kind: PromptKind::Rename,
                                buffer: row.name,
                            };
                        }
                    }
                }
                true
            }
            KeyCode::Char('d') => {
                if let Some(tree) = self.tree.as_ref() {
                    if let Some(row) = tree.selected_row() {
                        if row.path != *tree.root() {
                            self.overlay = Overlay::Prompt {
                                kind: PromptKind::DeleteConfirm,
                                buffer: format!("Apagar '{}'?", row.name),
                            };
                        }
                    }
                }
                true
            }
            KeyCode::Char('y') | KeyCode::Char('c') => {
                if let Some(tree) = self.tree.as_ref() {
                    if let Some(row) = tree.selected_row() {
                        let path_str = row
                            .path
                            .strip_prefix(&self.workspace)
                            .unwrap_or(&row.path)
                            .display()
                            .to_string();
                        let _ = crate::clipboard::copy_text(&path_str);
                        self.set_status(format!("copiado: {path_str}"));
                    }
                }
                true
            }
            KeyCode::Tab | KeyCode::Esc => {
                self.focus = Focus::Editor;
                self.set_status("foco: editor (Ctrl+B árvore · Ctrl+E editor)");
                true
            }
            KeyCode::PageUp => {
                if let Some(tree) = self.tree.as_mut() {
                    tree.move_selection(-10);
                    self.ensure_tree_visible();
                }
                true
            }
            KeyCode::PageDown => {
                if let Some(tree) = self.tree.as_mut() {
                    tree.move_selection(10);
                    self.ensure_tree_visible();
                }
                true
            }
            KeyCode::Home => {
                if let Some(tree) = self.tree.as_mut() {
                    tree.set_selected(0);
                    self.ensure_tree_visible();
                }
                true
            }
            KeyCode::End => {
                if let Some(tree) = self.tree.as_mut() {
                    let total_rows = tree.count_visible_rows().saturating_sub(1);
                    tree.set_selected(total_rows);
                    self.ensure_tree_visible();
                }
                true
            }
            _ => false,
        }
    }

    pub(crate) fn handle_terminal_key(&mut self, key: KeyEvent) -> bool {
        if key.code == KeyCode::Esc && key.modifiers.is_empty() {
            self.focus = Focus::Editor;
            self.set_status("foco: editor");
            return true;
        }

        if key.modifiers.contains(KeyModifiers::CONTROL)
            || key.modifiers.contains(KeyModifiers::ALT)
        {
            if let Some(command) = self.map_key(key) {
                if let KeyCommand::Action(action) = command {
                    if matches!(
                        action,
                        Action::ToggleTerminal
                            | Action::FocusEditor
                            | Action::FocusTree
                            | Action::FocusToggleTreeEditor
                            | Action::ToggleTree
                            | Action::ToggleScm
                            | Action::FocusScm
                            | Action::CommandPalette
                            | Action::Help
                            | Action::Quit
                            | Action::WhichKey
                            | Action::Welcome
                            | Action::TerminalGrow
                            | Action::TerminalShrink
                            | Action::NextTab
                            | Action::PrevTab
                    ) {
                        self.apply(command);
                        return true;
                    }
                }
            }
        }

        let Some(terminal) = self.terminal.as_mut() else {
            return false;
        };

        if key.modifiers.contains(KeyModifiers::CONTROL)
            && !key.modifiers.contains(KeyModifiers::ALT)
        {
            if let KeyCode::Char(character) = key.code {
                let control_byte = (character.to_ascii_lowercase() as u8) & 0x1f;
                if control_byte != 0 {
                    let _ = terminal.write_bytes(&[control_byte]);
                    return true;
                }
            }
        }

        if key.modifiers.contains(KeyModifiers::ALT)
            && !key.modifiers.contains(KeyModifiers::CONTROL)
        {
            if let KeyCode::Char(character) = key.code {
                let mut buffer = [0u8; 4];
                let encoded_str = character.encode_utf8(&mut buffer);
                let mut sequence = Vec::with_capacity(1 + encoded_str.len());
                sequence.push(0x1b);
                sequence.extend_from_slice(encoded_str.as_bytes());
                let _ = terminal.write_bytes(&sequence);
                return true;
            }
        }

        match key.code {
            KeyCode::Enter => {
                let _ = terminal.write_str("\r");
            }
            KeyCode::Backspace => {
                let _ = terminal.write_bytes(&[0x7f]);
            }
            KeyCode::Delete => {
                let _ = terminal.write_bytes(&[0x1b, b'[', b'3', b'~']);
            }
            KeyCode::Tab => {
                let _ = terminal.write_str("\t");
            }
            KeyCode::Char(character) if !key.modifiers.contains(KeyModifiers::CONTROL) => {
                let mut buffer = [0u8; 4];
                let encoded_str = character.encode_utf8(&mut buffer);
                let _ = terminal.write_str(encoded_str);
            }
            KeyCode::Up => {
                let _ = terminal.write_bytes(&[0x1b, b'[', b'A']);
            }
            KeyCode::Down => {
                let _ = terminal.write_bytes(&[0x1b, b'[', b'B']);
            }
            KeyCode::Right => {
                let _ = terminal.write_bytes(&[0x1b, b'[', b'C']);
            }
            KeyCode::Left => {
                let _ = terminal.write_bytes(&[0x1b, b'[', b'D']);
            }
            KeyCode::Home => {
                let _ = terminal.write_bytes(&[0x1b, b'[', b'H']);
            }
            KeyCode::End => {
                let _ = terminal.write_bytes(&[0x1b, b'[', b'F']);
            }
            KeyCode::PageUp => {
                let _ = terminal.write_bytes(&[0x1b, b'[', b'5', b'~']);
            }
            KeyCode::PageDown => {
                let _ = terminal.write_bytes(&[0x1b, b'[', b'6', b'~']);
            }
            _ => return false,
        }
        true
    }

    pub(crate) fn handle_diagnostics_key(&mut self, key: KeyEvent) {
        let Overlay::Diagnostics { selected } = &self.overlay else {
            return;
        };
        let mut selected = *selected;
        let total_diagnostics = self.diagnostics.len();
        match key.code {
            KeyCode::Esc | KeyCode::Char('q') => self.overlay = Overlay::None,
            KeyCode::Up => selected = selected.saturating_sub(1),
            KeyCode::Down if total_diagnostics > 0 => {
                selected = (selected + 1).min(total_diagnostics - 1)
            }
            KeyCode::Enter if total_diagnostics > 0 => {
                if let Some((path, diagnostic)) = self.diagnostics.get(selected).cloned() {
                    let _ = self.open_document_path(&path);
                    if let Ok(document) = self.store.active_mut() {
                        if let Some(offset) =
                            lsp_position_to_offset(document.buffer(), diagnostic.range.start)
                        {
                            document.jump_to_byte(offset);
                        }
                    }
                    self.scroll_y = 0;
                    self.ensure_cursor_visible();
                    self.overlay = Overlay::None;
                    self.focus = Focus::Editor;
                }
            }
            _ => {}
        }
        if matches!(self.overlay, Overlay::Diagnostics { .. }) {
            self.overlay = Overlay::Diagnostics { selected };
        }
    }

    pub(crate) fn handle_completion_key(&mut self, key: KeyEvent) {
        let Overlay::Completion {
            items,
            selected,
            replace_start,
        } = &self.overlay
        else {
            return;
        };
        let mut selected = *selected;
        let replace_start = *replace_start;
        let items = items.clone();
        let total_items = items.len();
        match key.code {
            KeyCode::Esc => {
                self.overlay = Overlay::None;
                return;
            }
            KeyCode::Char(character)
                if !key
                    .modifiers
                    .intersects(KeyModifiers::CONTROL | KeyModifiers::ALT) =>
            {
                self.overlay = Overlay::None;
                self.apply(KeyCommand::InsertChar(character));
                return;
            }
            KeyCode::Backspace => {
                self.overlay = Overlay::None;
                self.apply(KeyCommand::Action(Action::Backspace));
                let _ = self.refresh_auto_completion();
                return;
            }
            KeyCode::Left | KeyCode::Right => {
                self.overlay = Overlay::None;
                let action = if key.code == KeyCode::Left {
                    Action::MoveLeft { extend: false }
                } else {
                    Action::MoveRight { extend: false }
                };
                self.apply(KeyCommand::Action(action));
                return;
            }
            KeyCode::Up => selected = selected.saturating_sub(1),
            KeyCode::Down if total_items > 0 => selected = (selected + 1).min(total_items - 1),
            KeyCode::Enter | KeyCode::Tab if total_items > 0 => {
                if let Some(item) = items.get(selected) {
                    if let Ok(document) = self.store.active_mut() {
                        let replace_end = document.selection().head;
                        document.select_byte_range(
                            oride_core::ByteOffset::new(replace_start),
                            replace_end,
                        );
                        let _ = document.insert_text(&item.insert_text);
                    }
                    self.lsp_sync_active();
                }
                self.overlay = Overlay::None;
                return;
            }
            _ if key
                .modifiers
                .intersects(KeyModifiers::CONTROL | KeyModifiers::ALT) =>
            {
                self.overlay = Overlay::None;
                if let Some(command) = self.map_key(key) {
                    self.apply(command);
                }
                return;
            }
            _ => {}
        }
        self.overlay = Overlay::Completion {
            items,
            selected,
            replace_start,
        };
    }

    pub(crate) fn handle_project_find_key(&mut self, key: KeyEvent) {
        let Overlay::ProjectFind {
            query,
            selected,
            case_sensitive,
            use_regex,
            hits,
            status,
            replace_query,
            file_glob,
            focus_field,
        } = &self.overlay
        else {
            return;
        };
        let mut query = query.clone();
        let mut selected = *selected;
        let mut case_sensitive = *case_sensitive;
        let mut use_regex = *use_regex;
        let mut hits = hits.clone();
        let mut status = status.clone();
        let mut replace_query = replace_query.clone();
        let mut file_glob = file_glob.clone();
        let mut focus_field = *focus_field;
        let is_ctrl = key.modifiers.contains(KeyModifiers::CONTROL);
        let is_alt = key.modifiers.contains(KeyModifiers::ALT);

        match key.code {
            KeyCode::Esc => {
                self.overlay = Overlay::None;
                return;
            }
            KeyCode::Tab => {
                if replace_query.is_none() && file_glob.is_none() {
                    replace_query = Some(String::new());
                    focus_field = 1;
                } else {
                    let mut active_fields = vec![0];
                    if replace_query.is_some() {
                        active_fields.push(1);
                    }
                    if file_glob.is_some() {
                        active_fields.push(2);
                    }
                    let current_pos = active_fields
                        .iter()
                        .position(|&f| f == focus_field)
                        .unwrap_or(0);
                    let next_pos = (current_pos + 1) % active_fields.len();
                    focus_field = active_fields[next_pos];
                }
            }
            KeyCode::Char('g') if is_alt && !is_ctrl => {
                if file_glob.is_none() {
                    file_glob = Some(String::new());
                    focus_field = 2;
                } else if focus_field == 2 {
                    focus_field = 0;
                } else {
                    focus_field = 2;
                }
            }
            KeyCode::Char('h') if is_ctrl => {
                if replace_query.is_none() {
                    replace_query = Some(String::new());
                    focus_field = 1;
                } else if focus_field == 1 {
                    focus_field = 0;
                } else {
                    replace_query = None;
                    if focus_field == 1 {
                        focus_field = 0;
                    }
                }
            }
            KeyCode::Enter
                if (is_alt || is_ctrl || focus_field == 1) && replace_query.is_some() =>
            {
                if let Some(replacement) = replace_query.as_deref() {
                    self.execute_project_replace(
                        &query,
                        replacement,
                        file_glob.as_deref(),
                        case_sensitive,
                        use_regex,
                    );
                    return;
                }
            }
            KeyCode::Enter => {
                if let Some(hit) = hits.get(selected).cloned() {
                    self.jump_to_project_hit(hit);
                }
                return;
            }
            KeyCode::Up => selected = selected.saturating_sub(1),
            KeyCode::Down if !hits.is_empty() => {
                selected = (selected + 1).min(hits.len() - 1);
            }
            KeyCode::Char('c') if is_alt && !is_ctrl => {
                case_sensitive = !case_sensitive;
                self.recompute_project_find(
                    &query,
                    file_glob.as_deref(),
                    &mut hits,
                    &mut status,
                    case_sensitive,
                    use_regex,
                );
                selected = 0;
            }
            KeyCode::Char('r') if is_alt && !is_ctrl => {
                use_regex = !use_regex;
                self.recompute_project_find(
                    &query,
                    file_glob.as_deref(),
                    &mut hits,
                    &mut status,
                    case_sensitive,
                    use_regex,
                );
                selected = 0;
            }
            KeyCode::Backspace => {
                if focus_field == 1 {
                    if let Some(ref mut repl) = replace_query {
                        repl.pop();
                    }
                } else if focus_field == 2 {
                    if let Some(ref mut glob) = file_glob {
                        glob.pop();
                    }
                    self.recompute_project_find(
                        &query,
                        file_glob.as_deref(),
                        &mut hits,
                        &mut status,
                        case_sensitive,
                        use_regex,
                    );
                    selected = 0;
                } else {
                    query.pop();
                    self.recompute_project_find(
                        &query,
                        file_glob.as_deref(),
                        &mut hits,
                        &mut status,
                        case_sensitive,
                        use_regex,
                    );
                    selected = 0;
                }
            }
            KeyCode::Char(character) if !is_ctrl && !is_alt && !character.is_control() => {
                if focus_field == 1 {
                    if let Some(ref mut repl) = replace_query {
                        repl.push(character);
                    }
                } else if focus_field == 2 {
                    if let Some(ref mut glob) = file_glob {
                        glob.push(character);
                    }
                    self.recompute_project_find(
                        &query,
                        file_glob.as_deref(),
                        &mut hits,
                        &mut status,
                        case_sensitive,
                        use_regex,
                    );
                    selected = 0;
                } else {
                    query.push(character);
                    self.recompute_project_find(
                        &query,
                        file_glob.as_deref(),
                        &mut hits,
                        &mut status,
                        case_sensitive,
                        use_regex,
                    );
                    selected = 0;
                }
            }
            _ => {}
        }

        if !hits.is_empty() {
            selected = selected.min(hits.len() - 1);
        } else {
            selected = 0;
        }
        self.overlay = Overlay::ProjectFind {
            query,
            selected,
            case_sensitive,
            use_regex,
            hits,
            status,
            replace_query,
            file_glob,
            focus_field,
        };
    }

    pub(crate) fn handle_menu_key(&mut self, key: KeyEvent) {
        let Some((menu_index, item_index)) = self.menu_open else {
            return;
        };
        let total_menus = self.menus.len();
        let total_items = self
            .menus
            .get(menu_index)
            .map(|menu| menu.items.len())
            .unwrap_or(0);
        match key.code {
            KeyCode::Esc => self.menu_open = None,
            KeyCode::Left => {
                let next_menu = if menu_index == 0 {
                    total_menus.saturating_sub(1)
                } else {
                    menu_index - 1
                };
                self.menu_open = Some((next_menu, 0));
            }
            KeyCode::Right => {
                let next_menu = if total_menus == 0 {
                    0
                } else {
                    (menu_index + 1) % total_menus
                };
                self.menu_open = Some((next_menu, 0));
            }
            KeyCode::Up => {
                let next_item = item_index.saturating_sub(1);
                self.menu_open = Some((menu_index, next_item));
            }
            KeyCode::Down => {
                let next_item = if total_items == 0 {
                    0
                } else {
                    (item_index + 1).min(total_items - 1)
                };
                self.menu_open = Some((menu_index, next_item));
            }
            KeyCode::Enter => {
                let action_id = self
                    .menus
                    .get(menu_index)
                    .and_then(|menu| menu.items.get(item_index))
                    .map(|item| item.action_id.clone());
                self.menu_open = None;
                if let Some(action_id) = action_id {
                    self.apply_action_id(&action_id);
                }
            }
            KeyCode::Char(character) => {
                if let Some(menu) = self.menus.get(menu_index) {
                    let lower_char = character.to_ascii_lowercase();
                    if let Some(index) = menu
                        .items
                        .iter()
                        .position(|item| item.label.to_ascii_lowercase().starts_with(lower_char))
                    {
                        self.menu_open = Some((menu_index, index));
                    }
                }
            }
            _ => {}
        }
    }

    pub(crate) fn handle_welcome_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc | KeyCode::Enter | KeyCode::Char('q') | KeyCode::Char(' ') => {
                self.show_welcome = false;
            }
            _ => {
                self.show_welcome = false;
            }
        }
    }

    pub(crate) fn handle_scm_key(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Esc => {
                self.focus = Focus::Editor;
                true
            }
            KeyCode::Up | KeyCode::Char('k') => {
                self.scm_selected = self.scm_selected.saturating_sub(1);
                true
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if !self.scm_cache.is_empty() {
                    self.scm_selected = (self.scm_selected + 1).min(self.scm_cache.len() - 1);
                }
                true
            }
            KeyCode::Char('r') => {
                self.refresh_scm_cache();
                self.refresh_git_and_index();
                self.set_status("SCM refreshed");
                true
            }
            KeyCode::Char('d') => {
                if let Some((_, relative_path)) = self.scm_cache.get(self.scm_selected).cloned() {
                    let full_path = self.workspace.join(relative_path);
                    self.open_diff_for_path(&full_path);
                }
                true
            }
            KeyCode::Char('s') => {
                if let Some((_, relative_path)) = self.scm_cache.get(self.scm_selected).cloned() {
                    let full_path = self.workspace.join(&relative_path);
                    match oride_git::stage_path(&self.workspace, &full_path) {
                        Ok(()) => {
                            self.refresh_scm_cache();
                            self.refresh_git_and_index();
                            self.set_status(format!("staged: {}", relative_path.display()));
                        }
                        Err(error) => self.set_status(format!("stage: {error}")),
                    }
                }
                true
            }
            KeyCode::Char('u') => {
                if let Some((_, relative_path)) = self.scm_cache.get(self.scm_selected).cloned() {
                    let full_path = self.workspace.join(&relative_path);
                    match oride_git::unstage_path(&self.workspace, &full_path) {
                        Ok(()) => {
                            self.refresh_scm_cache();
                            self.refresh_git_and_index();
                            self.set_status(format!("unstaged: {}", relative_path.display()));
                        }
                        Err(error) => self.set_status(format!("unstage: {error}")),
                    }
                }
                true
            }
            KeyCode::Char('c') => {
                self.overlay = Overlay::Prompt {
                    kind: PromptKind::Commit,
                    buffer: String::new(),
                };
                true
            }
            KeyCode::Char('p') => {
                let _ = self.apply_action(Action::GitPull);
                true
            }
            KeyCode::Char('P') => {
                let _ = self.apply_action(Action::GitPush);
                true
            }
            KeyCode::Enter => {
                if let Some((_, relative_path)) = self.scm_cache.get(self.scm_selected).cloned() {
                    let full_path = self.workspace.join(relative_path);
                    self.record_jump();
                    match self.open_document_path(&full_path) {
                        Ok(_) => {
                            self.focus = Focus::Editor;
                            self.scroll_y = 0;
                            self.set_status(format!("opened {}", full_path.display()));
                        }
                        Err(error) => self.set_status(format!("open: {error}")),
                    }
                }
                true
            }
            _ => false,
        }
    }

    pub(crate) fn handle_buffer_picker_key(&mut self, key: KeyEvent) {
        let Overlay::BufferPicker { query, selected } = &self.overlay else {
            return;
        };
        let mut query = query.clone();
        let mut selected = *selected;
        let items = self.buffer_picker_items(&query);
        match key.code {
            KeyCode::Esc => self.overlay = Overlay::None,
            KeyCode::Up => selected = selected.saturating_sub(1),
            KeyCode::Down if !items.is_empty() => {
                selected = (selected + 1).min(items.len() - 1);
            }
            KeyCode::Backspace => {
                query.pop();
                selected = 0;
            }
            KeyCode::Char(character) if !character.is_control() => {
                query.push(character);
                selected = 0;
            }
            KeyCode::Enter => {
                if let Some((document_id, _)) = items.get(selected).cloned() {
                    self.record_jump();
                    let _ = self.store.set_active(document_id);
                    self.split.set_focused_doc(document_id);
                    self.focus = Focus::Editor;
                    self.scroll_y = 0;
                    self.overlay = Overlay::None;
                    return;
                }
            }
            _ => {}
        }
        if !matches!(self.overlay, Overlay::None) {
            self.overlay = Overlay::BufferPicker { query, selected };
        }
    }

    pub(crate) fn buffer_picker_items(&self, query: &str) -> Vec<(DocumentId, String)> {
        let lower_query = query.to_lowercase();
        let mut result_items = Vec::new();
        for tab in self.store.tab_summaries() {
            let path = self
                .store
                .get(tab.id)
                .and_then(|document| document.path().map(Path::to_path_buf));
            let label = if let Some(path) = path {
                let relative = path.strip_prefix(&self.workspace).unwrap_or(path.as_path());
                format!(
                    "{}{}",
                    if tab.dirty { "● " } else { "  " },
                    relative.display()
                )
            } else {
                format!("{}{}", if tab.dirty { "● " } else { "  " }, tab.title)
            };
            if lower_query.is_empty() || label.to_lowercase().contains(&lower_query) {
                result_items.push((tab.id, label));
            }
        }
        result_items
    }

    pub(crate) fn handle_diff_key(&mut self, key: KeyEvent) {
        let Overlay::Diff {
            path,
            lines,
            scroll,
        } = &self.overlay
        else {
            return;
        };
        let path = path.clone();
        let lines = lines.clone();
        let mut scroll = *scroll;
        match key.code {
            KeyCode::Esc | KeyCode::Char('q') => {
                self.overlay = Overlay::None;
                return;
            }
            KeyCode::Up | KeyCode::Char('k') => scroll = scroll.saturating_sub(1),
            KeyCode::Down | KeyCode::Char('j') => {
                scroll = (scroll + 1).min(lines.len().saturating_sub(1));
            }
            KeyCode::PageUp => scroll = scroll.saturating_sub(10),
            KeyCode::PageDown => {
                scroll = (scroll + 10).min(lines.len().saturating_sub(1));
            }
            KeyCode::Home => scroll = 0,
            KeyCode::End => scroll = lines.len().saturating_sub(1),
            _ => {}
        }
        self.overlay = Overlay::Diff {
            path,
            lines,
            scroll,
        };
    }

    pub(crate) fn handle_multi_picker_key(&mut self, key: KeyEvent) {
        let Overlay::MultiPicker { query, selected } = &self.overlay else {
            return;
        };
        let mut query = query.clone();
        let mut selected = *selected;
        let items = self.multi_picker_items(&query);
        match key.code {
            KeyCode::Esc => self.overlay = Overlay::None,
            KeyCode::Up => selected = selected.saturating_sub(1),
            KeyCode::Down if !items.is_empty() => {
                selected = (selected + 1).min(items.len() - 1);
            }
            KeyCode::Backspace => {
                query.pop();
                selected = 0;
            }
            KeyCode::Char(character) if !character.is_control() => {
                query.push(character);
                selected = 0;
            }
            KeyCode::Enter => {
                if let Some(item) = items.get(selected).cloned() {
                    self.overlay = Overlay::None;
                    self.run_multi_picker_item(&item);
                    return;
                }
            }
            _ => {}
        }
        if !matches!(self.overlay, Overlay::None) {
            self.overlay = Overlay::MultiPicker { query, selected };
        }
    }

    pub(crate) fn multi_picker_items(&self, query: &str) -> Vec<String> {
        let lower_query = query.to_lowercase();
        let mut items = Vec::new();
        // buffers
        for tab in self.store.tab_summaries() {
            let label = format!("buf: {}", tab.title);
            if lower_query.is_empty() || label.to_lowercase().contains(&lower_query) {
                items.push(label);
            }
        }
        // commands
        for action in Action::palette_actions() {
            let label = format!("cmd: {}", action.palette_label());
            if lower_query.is_empty() || label.to_lowercase().contains(&lower_query) {
                items.push(label);
            }
        }
        // files (limite para responsividade)
        for path in self.file_index.iter().take(500) {
            let relative = path.strip_prefix(&self.workspace).unwrap_or(path);
            let label = format!("file: {}", relative.display());
            if lower_query.is_empty() || label.to_lowercase().contains(&lower_query) {
                items.push(label);
            }
            if items.len() > 400 {
                break;
            }
        }
        items
    }

    pub(crate) fn run_multi_picker_item(&mut self, item: &str) {
        if let Some(name) = item.strip_prefix("buf: ") {
            for tab in self.store.tab_summaries() {
                if tab.title == name {
                    let _ = self.store.set_active(tab.id);
                    self.split.set_focused_doc(tab.id);
                    self.focus = Focus::Editor;
                    return;
                }
            }
        } else if let Some(label) = item.strip_prefix("cmd: ") {
            if let Some(action) = Action::palette_actions()
                .iter()
                .find(|action| action.palette_label() == label)
                .copied()
            {
                let _ = self.apply_action(action);
            }
        } else if let Some(relative_path) = item.strip_prefix("file: ") {
            let full_path = self.workspace.join(relative_path);
            let _ = self.open_document_path(&full_path);
            self.focus = Focus::Editor;
            self.scroll_y = 0;
        }
    }

    pub(crate) fn handle_undo_tree_key(&mut self, key: KeyEvent) {
        let Overlay::UndoTree { selected } = &self.overlay else {
            return;
        };
        let mut selected = *selected;
        let labels = self
            .store
            .active()
            .map(|document| document.undo_history_labels())
            .unwrap_or_default();
        let total_items = labels.len();
        match key.code {
            KeyCode::Esc | KeyCode::Char('q') => self.overlay = Overlay::None,
            KeyCode::Up => selected = selected.saturating_sub(1),
            KeyCode::Down if total_items > 0 => {
                selected = (selected + 1).min(total_items.saturating_sub(1))
            }
            KeyCode::Enter => {
                if total_items > 0 {
                    let steps = total_items - 1 - selected;
                    for _ in 0..=steps {
                        let _ = self.store.active_mut().map(|document| document.undo());
                    }
                    self.set_status(format!("undo ×{}", steps + 1));
                }
                self.overlay = Overlay::None;
            }
            _ => {}
        }
        if matches!(self.overlay, Overlay::UndoTree { .. }) {
            self.overlay = Overlay::UndoTree { selected };
        }
    }
}
