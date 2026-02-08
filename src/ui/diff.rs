use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::app::App;

pub fn draw(frame: &mut Frame, app: &App, area: Rect) {
    let diffs = app.pending_diffs();
    let title = format!(" Review Changes ({} pending) ", diffs.len());
    let block = Block::default().borders(Borders::ALL).title(title);
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let warning_lines = if app.any_requires_logout() { 2 } else { 0 };
    let row_count = diffs.len() + warning_lines;
    let constraints: Vec<Constraint> = (0..row_count)
        .map(|_| Constraint::Length(1))
        .chain(std::iter::once(Constraint::Min(0)))
        .collect();

    let rows = Layout::default()
        .constraints(constraints)
        .split(inner);

    for (i, (def, old, new)) in diffs.iter().enumerate() {
        if i >= rows.len() {
            break;
        }
        let old_display = match old {
            Some(val) => format!("{val}"),
            None => "Not set".to_string(),
        };
        let line = Line::from(vec![
            Span::styled(
                format!("  {:<36}", def.description),
                Style::default().fg(Color::White),
            ),
            Span::styled(old_display, Style::default().fg(Color::Red)),
            Span::styled("  →  ", Style::default().fg(Color::DarkGray)),
            Span::styled(format!("{new}"), Style::default().fg(Color::Green)),
            if def.requires_logout {
                Span::styled("  (logout required)", Style::default().fg(Color::Yellow))
            } else {
                Span::raw("")
            },
        ]);
        frame.render_widget(Paragraph::new(line), rows[i]);
    }

    if app.any_requires_logout() {
        let warn_idx = diffs.len();
        if warn_idx + 1 < rows.len() {
            let warning = Line::from(Span::styled(
                "  ⚠  Some changes require logout to take effect",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ));
            frame.render_widget(Paragraph::new(warning), rows[warn_idx + 1]);
        }
    }
}
