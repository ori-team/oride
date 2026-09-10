//! Overlay de command palette / browser / find.

use ratatui::layout::{Constraint, Direction, Layout, Position, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph};
use ratatui::Frame;

use crate::theme::UiTheme;
use crate::tree::pad_row;

/// Barra compacta de find/replace (1–3 linhas no rodapé, não tapa a tela).
pub struct FindBarView<'a> {
    pub query: &'a str,
    pub replace: &'a str,
    pub show_replace: bool,
    pub focus_replace: bool,
    pub status: &'a str,
    pub options: &'a str,
}

pub struct PaletteView<'a> {
    pub title: &'a str,
    pub query: &'a str,
    pub items: &'a [String],
    pub selected: usize,
    /// Texto de ajuda sob a lista (atalhos do modal).
    pub hint: &'a str,
}

impl<'a> PaletteView<'a> {
    #[must_use]
    pub fn simple(title: &'a str, query: &'a str, items: &'a [String], selected: usize) -> Self {
        Self {
            title,
            query,
            items,
            selected,
            hint: "",
        }
    }
}

pub fn render_palette(frame: &mut Frame, area: Rect, view: &PaletteView<'_>, _theme: &UiTheme) {
    let width = (area.width * 4 / 5)
        .max(40)
        .min(area.width.saturating_sub(2));
    let height = (area.height * 2 / 3)
        .max(10)
        .min(area.height.saturating_sub(2));
    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(height)) / 4;
    let rect = Rect::new(x, y, width, height);

    frame.render_widget(Clear, rect);

    let block = Block::default()
        .title(format!(" {} ", view.title))
        .borders(Borders::ALL)
        .border_style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .style(Style::default().bg(Color::Black).fg(Color::White));
    let inner = block.inner(rect);
    frame.render_widget(block, rect);

    let hint_h = if view.hint.is_empty() { 0 } else { 1 };
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(hint_h),
        ])
        .split(inner);

    let prompt_w = chunks[0].width as usize;
    let prompt_text = pad_row(&format!("> {}▌", view.query), prompt_w);
    let prompt = Paragraph::new(Line::from(Span::styled(
        prompt_text,
        Style::default()
            .fg(Color::Black)
            .bg(Color::Gray)
            .add_modifier(Modifier::BOLD),
    )));
    frame.render_widget(prompt, chunks[0]);

    let list_area = chunks[1];
    let list_h = list_area.height as usize;
    let list_w = list_area.width as usize;
    if list_h == 0 || list_w == 0 {
        return;
    }

    let start = if view.items.is_empty() {
        0
    } else {
        view.selected
            .saturating_sub(list_h.saturating_sub(1))
            .min(view.items.len().saturating_sub(1))
    };

    let mut lines = Vec::with_capacity(list_h);
    let mut cursor_pos: Option<Position> = None;

    for row in 0..list_h {
        let idx = start + row;
        if idx >= view.items.len() {
            lines.push(Line::from(Span::styled(
                " ".repeat(list_w),
                Style::default().bg(Color::Black).fg(Color::DarkGray),
            )));
            continue;
        }
        let selected = idx == view.selected;
        let mark = if selected { "▶ " } else { "  " };
        let text = pad_row(&format!("{mark}{}", view.items[idx]), list_w);
        let style = if selected {
            // Linha inteira ciano — alto contraste (igual à árvore)
            Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White).bg(Color::Black)
        };
        lines.push(Line::from(Span::styled(text, style)));
        if selected {
            cursor_pos = Some(Position {
                x: list_area.x,
                y: list_area.y + row as u16,
            });
        }
    }

    frame.render_widget(Paragraph::new(lines), list_area);

    if hint_h > 0 {
        let hint = pad_row(view.hint, chunks[2].width as usize);
        frame.render_widget(
            Paragraph::new(Line::from(Span::styled(
                hint,
                Style::default().fg(Color::Yellow).bg(Color::Black),
            ))),
            chunks[2],
        );
    }

    if let Some(pos) = cursor_pos {
        frame.set_cursor_position(pos);
    }
}

pub struct ProjectFindView<'a> {
    pub title: &'a str,
    pub query: &'a str,
    pub replace_query: Option<&'a str>,
    pub file_glob: Option<&'a str>,
    pub focus_field: u8,
    pub items: &'a [String],
    pub selected: usize,
    pub hint: &'a str,
}

pub fn render_project_find(
    frame: &mut Frame,
    area: Rect,
    view: &ProjectFindView<'_>,
    _theme: &UiTheme,
) {
    let width = (area.width * 4 / 5)
        .max(40)
        .min(area.width.saturating_sub(2));
    let height = (area.height * 2 / 3)
        .max(10)
        .min(area.height.saturating_sub(2));
    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(height)) / 4;
    let rect = Rect::new(x, y, width, height);

    frame.render_widget(Clear, rect);

    let block = Block::default()
        .title(format!(" {} ", view.title))
        .borders(Borders::ALL)
        .border_style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .style(Style::default().bg(Color::Black).fg(Color::White));
    let inner = block.inner(rect);
    frame.render_widget(block, rect);

    let mut input_lines: u16 = 1;
    if view.replace_query.is_some() {
        input_lines += 1;
    }
    if view.file_glob.is_some() {
        input_lines += 1;
    }
    let hint_h = if view.hint.is_empty() { 0 } else { 1 };
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(input_lines),
            Constraint::Min(1),
            Constraint::Length(hint_h),
        ])
        .split(inner);

    let prompt_w = chunks[0].width as usize;
    if view.replace_query.is_some() || view.file_glob.is_some() {
        let mut widget_lines = Vec::new();

        let q_style = if view.focus_field == 0 {
            Style::default()
                .fg(Color::Black)
                .bg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White).bg(Color::DarkGray)
        };
        let q_cursor = if view.focus_field == 0 { "▌" } else { "" };
        let q_line = pad_row(&format!(" Find: {}{}", view.query, q_cursor), prompt_w);
        widget_lines.push(Line::from(Span::styled(q_line, q_style)));

        if let Some(replace_text) = view.replace_query {
            let r_style = if view.focus_field == 1 {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White).bg(Color::DarkGray)
            };
            let r_cursor = if view.focus_field == 1 { "▌" } else { "" };
            let r_line = pad_row(&format!(" Repl: {}{}", replace_text, r_cursor), prompt_w);
            widget_lines.push(Line::from(Span::styled(r_line, r_style)));
        }

        if let Some(glob_text) = view.file_glob {
            let g_style = if view.focus_field == 2 {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White).bg(Color::DarkGray)
            };
            let g_cursor = if view.focus_field == 2 { "▌" } else { "" };
            let g_line = pad_row(&format!(" Glob: {}{}", glob_text, g_cursor), prompt_w);
            widget_lines.push(Line::from(Span::styled(g_line, g_style)));
        }

        let input_widget = Paragraph::new(widget_lines);
        frame.render_widget(input_widget, chunks[0]);
    } else {
        let prompt_text = pad_row(&format!("> {}▌", view.query), prompt_w);
        let prompt = Paragraph::new(Line::from(Span::styled(
            prompt_text,
            Style::default()
                .fg(Color::Black)
                .bg(Color::Gray)
                .add_modifier(Modifier::BOLD),
        )));
        frame.render_widget(prompt, chunks[0]);
    }

    let list_area = chunks[1];
    let list_h = list_area.height as usize;
    let list_w = list_area.width as usize;
    if list_h == 0 || list_w == 0 {
        return;
    }

    let start = if view.items.is_empty() {
        0
    } else {
        view.selected
            .saturating_sub(list_h.saturating_sub(1))
            .min(view.items.len().saturating_sub(1))
    };

    let mut lines = Vec::with_capacity(list_h);
    let mut cursor_pos: Option<Position> = None;

    for row in 0..list_h {
        let idx = start + row;
        if idx >= view.items.len() {
            lines.push(Line::from(Span::styled(
                " ".repeat(list_w),
                Style::default().bg(Color::Black).fg(Color::DarkGray),
            )));
            continue;
        }
        let selected = idx == view.selected;
        let mark = if selected { "▶ " } else { "  " };
        let text = pad_row(&format!("{mark}{}", view.items[idx]), list_w);
        let style = if selected {
            Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White).bg(Color::Black)
        };
        lines.push(Line::from(Span::styled(text, style)));
        if selected {
            cursor_pos = Some(Position {
                x: list_area.x,
                y: list_area.y + row as u16,
            });
        }
    }

    frame.render_widget(Paragraph::new(lines), list_area);

    if hint_h > 0 {
        let hint = pad_row(view.hint, chunks[2].width as usize);
        frame.render_widget(
            Paragraph::new(Line::from(Span::styled(
                hint,
                Style::default().fg(Color::Yellow).bg(Color::Black),
            ))),
            chunks[2],
        );
    }

    if let Some(pos) = cursor_pos {
        frame.set_cursor_position(pos);
    }
}

/// Find/replace no rodapé — altura pequena, deixa o buffer legível.
pub fn render_find_bar(frame: &mut Frame, area: Rect, view: &FindBarView<'_>) {
    let lines = if view.show_replace { 3u16 } else { 2u16 };
    let height = lines.min(area.height);
    if height == 0 || area.width == 0 {
        return;
    }
    let y = area.y + area.height.saturating_sub(height);
    let rect = Rect::new(area.x, y, area.width, height);
    frame.render_widget(Clear, rect);

    let w = rect.width as usize;
    let q_style = if view.focus_replace {
        Style::default().fg(Color::White).bg(Color::Black)
    } else {
        Style::default()
            .fg(Color::Black)
            .bg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    };
    let r_style = if view.focus_replace {
        Style::default()
            .fg(Color::Black)
            .bg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White).bg(Color::Black)
    };
    let hint_style = Style::default().fg(Color::Cyan).bg(Color::Black);

    let q_line = pad_row(&format!("Find: {}▌  {}", view.query, view.status), w);
    let mut out = vec![Line::from(Span::styled(q_line, q_style))];
    if view.show_replace {
        let r_line = pad_row(&format!("Repl: {}▌", view.replace), w);
        out.push(Line::from(Span::styled(r_line, r_style)));
    }
    let opt = pad_row(view.options, w);
    out.push(Line::from(Span::styled(opt, hint_style)));

    frame.render_widget(Paragraph::new(out), rect);
}

/// Estrutura para visualização compacta de sugestões de autocompletar.
pub struct CompletionPopupView<'a> {
    pub items: &'a [String],
    pub selected: usize,
    pub cursor_pos: Option<Position>,
}

/// Renderiza o dropdown compacto de autocompletar posicionado diretamente abaixo (ou acima) do cursor.
pub fn render_completion_popup(
    frame: &mut Frame,
    area: Rect,
    view: &CompletionPopupView<'_>,
    _theme: &UiTheme,
) {
    if view.items.is_empty() || area.width == 0 || area.height == 0 {
        return;
    }

    // Calcula largura compacta baseada no tamanho dos itens
    let max_item_len = view
        .items
        .iter()
        .take(15)
        .map(|item| item.chars().count())
        .max()
        .unwrap_or(12);
    // 2 bordas + 2 prefixo ("▶ ") + 2 margem de respiro
    let width = ((max_item_len + 6).clamp(24, 45) as u16).min(area.width.saturating_sub(2));

    // Altura compacta: no máximo 7 itens visíveis + 2 para bordas superior/inferior
    let max_visible_items = 7.min(view.items.len());
    let height = ((max_visible_items + 2) as u16).min(area.height.saturating_sub(2));

    if width < 6 || height < 3 {
        return;
    }

    // Posicionamento relativo ao cursor onde o código está sendo digitado
    let (x, y) = if let Some(cursor) = view.cursor_pos {
        let desired_y = cursor.y.saturating_add(1);
        let y = if desired_y + height <= area.y + area.height {
            desired_y
        } else if cursor.y >= area.y + height {
            // Se estiver no final da tela, exibe logo acima da linha atual
            cursor.y.saturating_sub(height)
        } else {
            (area.y + area.height).saturating_sub(height)
        };

        let mut x = cursor.x;
        if x + width > area.x + area.width {
            x = (area.x + area.width).saturating_sub(width);
        }
        (x.max(area.x), y.max(area.y))
    } else {
        // Fallback caso a posição do cursor não esteja visível no viewport
        let x = area.x + 4.min(area.width.saturating_sub(width));
        let y = area.y + 2.min(area.height.saturating_sub(height));
        (x, y)
    };

    let popup_rect = Rect::new(x, y, width, height);

    // Limpa a área do popup para não vazar o texto de fundo do editor
    frame.render_widget(Clear, popup_rect);

    let block = Block::default()
        .title(" Suggest ")
        .borders(Borders::ALL)
        .border_style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .style(Style::default().bg(Color::Black).fg(Color::White));

    let inner = block.inner(popup_rect);
    frame.render_widget(block, popup_rect);

    let list_h = inner.height as usize;
    let list_w = inner.width as usize;
    if list_h == 0 || list_w == 0 {
        return;
    }

    let start = view
        .selected
        .saturating_sub(list_h.saturating_sub(1))
        .min(view.items.len().saturating_sub(1));

    let mut lines = Vec::with_capacity(list_h);
    for row in 0..list_h {
        let idx = start + row;
        if idx >= view.items.len() {
            lines.push(Line::from(Span::styled(
                " ".repeat(list_w),
                Style::default().bg(Color::Black),
            )));
            continue;
        }

        let is_selected = idx == view.selected;
        let mark = if is_selected { "▶ " } else { "  " };
        let item_str = &view.items[idx];

        if is_selected {
            let text = pad_row(&format!("{mark}{item_str}"), list_w);
            let style = Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD);
            lines.push(Line::from(Span::styled(text, style)));
        } else if let Some((name, detail)) = item_str.split_once(" — ") {
            let left = format!("{mark}{name}");
            let right = format!(" — {detail}");
            let left_len = left.chars().count();
            let right_len = right.chars().count();
            let total = left_len + right_len;
            if total <= list_w {
                let pad = list_w - total;
                lines.push(Line::from(vec![
                    Span::styled(left, Style::default().fg(Color::White).bg(Color::Black)),
                    Span::styled(right, Style::default().fg(Color::DarkGray).bg(Color::Black)),
                    Span::styled(" ".repeat(pad), Style::default().bg(Color::Black)),
                ]));
            } else {
                let text = pad_row(&format!("{mark}{item_str}"), list_w);
                lines.push(Line::from(Span::styled(
                    text,
                    Style::default().fg(Color::White).bg(Color::Black),
                )));
            }
        } else {
            let text = pad_row(&format!("{mark}{item_str}"), list_w);
            lines.push(Line::from(Span::styled(
                text,
                Style::default().fg(Color::White).bg(Color::Black),
            )));
        }
    }

    frame.render_widget(Paragraph::new(lines), inner);

    // Mantém o cursor visual do terminal fixado na posição em que o usuário está digitando
    if let Some(pos) = view.cursor_pos {
        frame.set_cursor_position(pos);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pad_keeps_width() {
        let s = pad_row("▶ item", 12);
        assert_eq!(s.chars().count(), 12);
    }
}
