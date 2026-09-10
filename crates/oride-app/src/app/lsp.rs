//! Integração com Language Server Protocol (LSP).

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use oride_core::{DocumentError, DocumentId};
use oride_lsp::{character_column, utf16_column, CompletionItem, LspEvent, Position as LspPos};
use oride_syntax::LanguageId;

use super::state::{App, CompletionChoice, Focus, Overlay};

impl App {
    pub(crate) fn poll_lsp_events(&mut self) {
        let events: Vec<_> = self
            .lsp_clients
            .iter_mut()
            .flat_map(|(language, client)| {
                client
                    .poll_events()
                    .into_iter()
                    .map(|event| (*language, event))
            })
            .collect();

        let mut exited = Vec::new();
        for (language, event) in events {
            match event {
                LspEvent::Diagnostics { uri, diagnostics } => {
                    let path = uri_to_path(&uri);
                    self.diagnostics
                        .retain(|(diagnostic_path, _)| Some(diagnostic_path) != path.as_ref());
                    if let Some(target_path) = path {
                        for diagnostic in diagnostics {
                            self.diagnostics.push((target_path.clone(), diagnostic));
                        }
                    }
                }
                LspEvent::Exited => {
                    exited.push(language);
                    self.set_status(format!("LSP {} saiu", language.as_str()));
                }
                LspEvent::ServerMessage(server_message) => self.set_status(server_message),
            }
        }
        for language in exited {
            self.lsp_clients.remove(&language);
            self.lsp_failures
                .insert(language, "servidor encerrou".into());
        }
    }

    pub(crate) fn lsp_command_for(&self, language: LanguageId) -> Option<Vec<String>> {
        if let Some(command) = self.config.lsp.servers.get(language.as_str()) {
            return (!command.is_empty()).then(|| command.clone());
        }
        if language == LanguageId::OriScript {
            return (!self.config.lsp.oriscript_command.is_empty())
                .then(|| self.config.lsp.oriscript_command.clone());
        }
        self.plugin_host.lsp_command(language)
    }

    pub(crate) fn ensure_lsp_client(&mut self, language: LanguageId) -> Result<(), String> {
        if self.lsp_clients.contains_key(&language) {
            return Ok(());
        }
        if !self.config.lsp.enabled {
            return Err("LSP desativado na configuração".into());
        }
        if let Some(error) = self.lsp_failures.get(&language) {
            return Err(error.clone());
        }
        let command = self
            .lsp_command_for(language)
            .ok_or_else(|| format!("LSP não configurado para {}", language.as_str()))?;
        match oride_lsp::LspClient::spawn(&command, &self.workspace, self.config.lsp.timeout_ms) {
            Ok(client) => {
                self.lsp_clients.insert(language, client);
                Ok(())
            }
            Err(error) => {
                let message = format!("{}: {error}", command.join(" "));
                self.lsp_failures.insert(language, message.clone());
                Err(message)
            }
        }
    }

    pub(crate) fn lsp_sync_active(&mut self) {
        let Some(id) = self.store.active_id() else {
            return;
        };
        self.lsp_sync_document(id);
    }

    pub(crate) fn lsp_sync_document(&mut self, id: DocumentId) {
        let (path, text) = match self.store.get(id) {
            Some(doc) => match doc.path() {
                Some(p) => (p.to_path_buf(), doc.buffer().as_string()),
                None => return,
            },
            None => return,
        };
        let language = self.detect_document_language(Some(&path));
        if self.ensure_lsp_client(language).is_err() {
            return;
        }
        self.lsp_doc_version += 1;
        let version = self.lsp_doc_version;
        if let Some(client) = self.lsp_clients.get_mut(&language) {
            let _ = client.did_open(&path, language.as_str(), &text);
            let _ = client.did_change(&path, version, &text);
        }
    }

    pub(crate) fn lsp_open_active(&mut self) {
        let (path, text) = match self.store.active() {
            Ok(doc) => match doc.path() {
                Some(p) => (p.to_path_buf(), doc.buffer().as_string()),
                None => return,
            },
            Err(_) => return,
        };
        let language = self.detect_document_language(Some(&path));
        if self.ensure_lsp_client(language).is_err() {
            return;
        }
        if let Some(client) = self.lsp_clients.get_mut(&language) {
            let _ = client.did_open(&path, language.as_str(), &text);
        }
    }

    pub(crate) fn lsp_complete(&mut self) -> Result<(), DocumentError> {
        let (path, position) = self.active_lsp_pos()?;
        let language = self.detect_document_language(Some(&path));
        let (replace_start, prefix) = self.active_completion_prefix()?;
        let mut choices = self.offline_completion_choices(language, &prefix);
        self.lsp_open_active();
        let lsp_error = match self.ensure_lsp_client(language) {
            Ok(()) => match self.lsp_clients.get_mut(&language) {
                Some(client) => match client.completion(&path, position) {
                    Ok(items) => {
                        append_lsp_completion_choices(&mut choices, items, &prefix);
                        None
                    }
                    Err(error) => Some(error.to_string()),
                },
                None => Some("cliente LSP indisponível".into()),
            },
            Err(error) => Some(error),
        };
        deduplicate_completion_choices(&mut choices);

        if choices.is_empty() {
            self.set_status(match lsp_error {
                Some(error) => format!("completion indisponível: {error}"),
                None => "sem sugestões".into(),
            });
        } else {
            self.overlay = Overlay::Completion {
                items: choices,
                selected: 0,
                replace_start,
            };
        }
        Ok(())
    }

    pub(crate) fn active_completion_prefix(&self) -> Result<(usize, String), DocumentError> {
        let document = self.store.active()?;
        let caret = document.selection().head.as_usize();
        let source = document.buffer().as_string();
        let (start, prefix) = identifier_prefix_at(&source, caret);
        Ok((start, prefix.to_string()))
    }

    pub(crate) fn offline_completion_choices(
        &self,
        language: LanguageId,
        prefix: &str,
    ) -> Vec<CompletionChoice> {
        let normalized = prefix.to_lowercase();
        self.plugin_host
            .completion_words(language)
            .into_iter()
            .filter(|word| {
                let candidate = word.to_lowercase();
                candidate.starts_with(&normalized) && candidate != normalized
            })
            .map(|word| CompletionChoice {
                display: format!("{word} — local"),
                insert_text: word,
            })
            .collect()
    }

    pub(crate) fn lsp_hover(&mut self) -> Result<(), DocumentError> {
        let (path, position) = self.active_lsp_pos()?;
        let language = self.detect_document_language(Some(&path));
        if let Err(error) = self.ensure_lsp_client(language) {
            self.set_status(format!("hover indisponível: {error}"));
            return Ok(());
        }
        let Some(client) = self.lsp_clients.get_mut(&language) else {
            self.set_status("hover indisponível: cliente LSP ausente");
            return Ok(());
        };
        match client.hover(&path, position) {
            Ok(Some(hover_info)) => {
                self.overlay = Overlay::Hover {
                    text: hover_info.contents,
                };
            }
            Ok(None) => self.set_status("sem hover"),
            Err(error) => self.set_status(format!("hover: {error}")),
        }
        Ok(())
    }

    pub(crate) fn lsp_goto(&mut self) -> Result<(), DocumentError> {
        let (path, position) = self.active_lsp_pos()?;
        let language = self.detect_document_language(Some(&path));
        if let Err(error) = self.ensure_lsp_client(language) {
            self.set_status(format!("definition indisponível: {error}"));
            return Ok(());
        }
        let Some(client) = self.lsp_clients.get_mut(&language) else {
            self.set_status("definition indisponível: cliente LSP ausente");
            return Ok(());
        };
        match client.definition(&path, position) {
            Ok(Some(location)) => {
                if let Some(target_path) = uri_to_path(&location.uri) {
                    self.record_jump();
                    let _ = self.open_document_path(&target_path);
                    if let Ok(document) = self.store.active_mut() {
                        if let Some(offset) =
                            lsp_position_to_offset(document.buffer(), location.range.start)
                        {
                            document.jump_to_byte(offset);
                        }
                    }
                    self.focus = Focus::Editor;
                    self.ensure_cursor_visible();
                    self.set_status(format!("goto {}", target_path.display()));
                }
            }
            Ok(None) => self.set_status("sem definição"),
            Err(error) => self.set_status(format!("goto: {error}")),
        }
        Ok(())
    }

    pub(crate) fn lsp_format(&mut self) -> Result<(), DocumentError> {
        let active = self.store.active().ok().and_then(|document| {
            document
                .path()
                .map(|path| (path.to_path_buf(), document.buffer().as_string()))
        });
        let Some((path, source)) = active else {
            self.set_status("format: sem path");
            return Ok(());
        };
        let language = self.detect_document_language(Some(&path));
        if let Err(error) = self.ensure_lsp_client(language) {
            self.set_status(format!("format indisponível: {error}"));
            return Ok(());
        }
        let Some(client) = self.lsp_clients.get_mut(&language) else {
            self.set_status("format indisponível: cliente LSP ausente");
            return Ok(());
        };
        match client.formatting(
            &path,
            &source,
            self.config.editor.tab_size,
            self.config.editor.insert_spaces,
        ) {
            Ok(Some(formatted_text)) => {
                self.store
                    .active_mut()?
                    .replace_full_text(&formatted_text)?;
                self.lsp_sync_active();
                self.set_status("formatado");
            }
            Ok(None) => self.set_status("format: sem alterações"),
            Err(error) => self.set_status(format!("format: {error}")),
        }
        Ok(())
    }

    pub(crate) fn active_lsp_pos(&self) -> Result<(PathBuf, LspPos), DocumentError> {
        let document = self.store.active()?;
        let path = document.path().map(Path::to_path_buf).ok_or_else(|| {
            DocumentError::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "documento sem caminho de arquivo salvo",
            ))
        })?;
        let caret = document.caret()?;
        let line_text = document.buffer().line_text(caret.line)?;
        Ok((
            path,
            LspPos {
                line: caret.line as u32,
                character: utf16_column(&line_text, caret.column),
            },
        ))
    }
}

pub(crate) fn uri_to_path(uri: &str) -> Option<PathBuf> {
    let raw_path = uri.strip_prefix("file://").unwrap_or(uri);
    let decoded_path = raw_path.replace("%20", " ");
    Some(PathBuf::from(decoded_path))
}

pub(crate) fn lsp_position_to_offset(
    buffer: &oride_core::Buffer,
    position: LspPos,
) -> Option<oride_core::ByteOffset> {
    let line = position.line as usize;
    let line_text = buffer.line_text(line).ok()?;
    let column = character_column(&line_text, position.character)?;
    buffer
        .caret_to_byte(oride_core::Caret::new(line, column))
        .ok()
}

pub(crate) fn identifier_prefix_at(source: &str, caret: usize) -> (usize, &str) {
    if caret > source.len() || !source.is_char_boundary(caret) {
        return (caret.min(source.len()), "");
    }
    let mut start = caret;
    for (offset, character) in source[..caret].char_indices().rev() {
        if character == '_' || character.is_alphanumeric() {
            start = offset;
        } else {
            break;
        }
    }
    (start, &source[start..caret])
}

pub(crate) fn append_lsp_completion_choices(
    choices: &mut Vec<CompletionChoice>,
    items: Vec<CompletionItem>,
    prefix: &str,
) {
    let normalized = prefix.to_lowercase();
    let mut semantic: Vec<_> = items
        .into_iter()
        .filter(|item| completion_label_matches(&item.label, &normalized))
        .map(|item| {
            let display = match &item.detail {
                Some(detail) => format!("{} — {}", item.label, detail),
                None => item.label.clone(),
            };
            CompletionChoice {
                display,
                insert_text: item.insert_text.unwrap_or(item.label),
            }
        })
        .collect();
    semantic.append(choices);
    *choices = semantic;
}

pub(crate) fn completion_label_matches(label: &str, normalized_prefix: &str) -> bool {
    if normalized_prefix.is_empty() {
        return true;
    }
    let normalized_label = label.to_lowercase();
    normalized_label.starts_with(normalized_prefix)
        || normalized_label
            .rsplit(['.', ':'])
            .next()
            .is_some_and(|segment| segment.starts_with(normalized_prefix))
}

pub(crate) fn deduplicate_completion_choices(choices: &mut Vec<CompletionChoice>) {
    let mut seen = HashSet::new();
    choices.retain(|choice| seen.insert(choice.insert_text.clone()));
}
