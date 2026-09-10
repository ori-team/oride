//! Painel inferior do terminal embutido.

use ratatui::layout::{Position, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

use crate::theme::UiTheme;
use crate::tree::pad_row;

pub fn render_terminal_panel(
    frame: &mut Frame,
    area: Rect,
    lines: &[String],
    focused: bool,
    theme: &UiTheme,
    error: Option<&str>,
    cursor_col: Option<usize>,
) {
    if area.height == 0 {
        return;
    }
    let title = if focused {
        " TERMINAL · digite aqui · Esc=editor "
    } else {
        " terminal · Ctrl+\" foca "
    };
    let border = if focused {
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::DarkGray)
    };
    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(border)
        .style(theme.editor_style());
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let mut max = inner.height as usize;
    let w = inner.width as usize;
    let mut out_lines: Vec<Line> = Vec::new();
    if let Some(err) = error {
        if max > 0 {
            out_lines.push(Line::from(Span::styled(
                pad_row(&format!("! {err}"), w),
                Style::default().fg(Color::Black).bg(Color::Red),
            )));
            max = max.saturating_sub(1);
        }
    }
    let start = lines.len().saturating_sub(max);
    for l in &lines[start..] {
        out_lines.push(Line::from(Span::styled(
            pad_row(l, w),
            if focused {
                Style::default().fg(Color::White).bg(Color::Black)
            } else {
                theme.editor_style()
            },
        )));
    }
    while out_lines.len() < inner.height as usize {
        out_lines.push(Line::from(Span::styled(
            " ".repeat(w),
            Style::default().bg(Color::Black),
        )));
    }
    frame.render_widget(Paragraph::new(out_lines), inner);

    if focused && inner.height > 0 && inner.width > 0 {
        let active_rows = lines[start..].len();
        let last_line_idx = if active_rows > 0 {
            active_rows.saturating_sub(1)
        } else {
            0
        };
        let last_line_text = lines.last().map(|s| s.trim_end()).unwrap_or("");
        let col = cursor_col.unwrap_or_else(|| last_line_text.chars().count());
        let cursor_x = inner
            .x
            .saturating_add((col as u16).min(inner.width.saturating_sub(1)));
        let cursor_y = inner.y.saturating_add(last_line_idx as u16);
        frame.set_cursor_position(Position {
            x: cursor_x,
            y: cursor_y,
        });
    }
}
